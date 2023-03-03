//! # Purpose:
//! create a Future that reads 100 chars from a file,
//! ond once done, yields a String made of these 100 chars

use std::future::Future;
use std::task::{Poll as TaskPoll, Wake};
use std::pin::Pin;
use std::task::{Context, Waker};
use poll::file::File;
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use poll::Poll;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError, sync_channel};

pub struct PollWakerContext {
    poll: Poll,
    map: HashMap<c_int, Waker>
}

impl PollWakerContext {
    pub fn new(poll: Poll) -> PollWakerContext {
        PollWakerContext {
            poll,
            map: HashMap::new()
        }
    }

    pub fn add(&mut self, file: Arc<Mutex<File>>, waker: Waker) {
        self.map.insert(file.lock().unwrap().file_descriptor, waker);
    }

    pub fn block(&mut self) {
        if let Ok(Some(file)) = self.poll.wait() {
            if let Some(waker) = self.map.get(&file.lock().unwrap().file_descriptor) {
                waker.clone().wake();
            }
        }
    }
}


pub struct FileReadFuture {
    file: Arc<Mutex<File>>,
    bytes: Vec<u8>,
    poll_waker_context: Arc<Mutex<PollWakerContext>>
}

// I have a problem with FileReadFuture.file not implementing Send and Sync, which I do not understand
// I work around that by implementing Sync and Send for FileReadFuture
unsafe impl Sync for FileReadFuture{

}

unsafe impl Send for FileReadFuture{

}

impl FileReadFuture {
    pub fn new(file: Arc<Mutex<File>>, poll_waker_context: Arc<Mutex<PollWakerContext>>) -> FileReadFuture {
        FileReadFuture {
            file,
            bytes: Vec::new(),
            poll_waker_context,
        }
    }
}

impl Future for FileReadFuture {
    type Output = CString;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<CString> {
        let mut newBytes =self.file.lock().unwrap().read().unwrap().into_bytes();
        self.bytes.append(&mut newBytes);

        if self.bytes.len() >= 100 {
            let cstring = CString::new(self.bytes.clone()).unwrap();
            std::task::Poll::Ready(cstring)
        } else {
            let mut pwc = self.poll_waker_context.lock().unwrap();
            pwc.add(self.file.clone(), cx.waker().clone());
            std::task::Poll::Pending
        }
    }
}

trait IoBlocker {
    fn block(&self);
}

impl IoBlocker for Arc<Mutex<PollWakerContext>> {
    fn block(&self) {
        self.lock().unwrap().block();
    }
}

pub struct Executor<IO_BLOCKER> {
    queue_receiver: Receiver<Arc<Task>>,
    queue_sender: SyncSender<Arc<Task>>,
    io_blocker: IO_BLOCKER, // Arc<Mutex<PollWakerContext>>,
}

impl<IO_BLOCKER> Executor<IO_BLOCKER>
where IO_BLOCKER: IoBlocker
{

    pub fn new(io_blocker: IO_BLOCKER) -> Executor<IO_BLOCKER> {
        let (queue_sender,  queue_receiver) = sync_channel(10);
        Executor {
            queue_receiver,
            queue_sender,
            io_blocker
        }
    }

    pub fn queue(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future: Pin<Box<dyn Future<Output = ()> + 'static + Send>> = Box::pin(future);
        let future = Mutex::new(future);
        let queue_sender = self.queue_sender.clone();
        let task = Task{
            future,
            queue_sender,
        };
        let task = Arc::new(task);
        self.queue_sender.send(task);
    }

    pub fn execute(&self) {
        loop {
            println!("Entering execute loop");
            // poll all tasks that are queued
            while let Ok(task) = self.queue_receiver.try_recv() {
                println!("Received a task in execute loop");
                self.poll_task(task);
                println!("Polled a task in execute loop");
            }
            // block on io. This might wake tasks and therefore queue them, so we can loop back to
            // processing queued tasks again.
            self.io_blocker.block();
        }
    }

    fn poll_task(&self, mut task: Arc<Task>) {

        let waker: Waker = task.clone().into();
        let mut context = Context::from_waker(&waker);

        let fut_poll_result= task.future.lock().unwrap().as_mut().poll(&mut context);
        match fut_poll_result {
            TaskPoll::Ready(()) => (),
            TaskPoll::Pending => (),
        }
    }
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    queue_sender: SyncSender<Arc<Task>>,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
         self.queue_sender.send(self.clone());
     }
}

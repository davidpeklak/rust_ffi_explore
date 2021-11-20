//! # Purpose:
//! create a Future that reads 100 chars from a file,
//! ond once done, yields a String made of these 100 chars

use std::future::Future;
use std::task::{Poll as TaskPoll, Wake};
use std::pin::Pin;
use std::task::{Context, Waker};
use crate::file::File;
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use crate::poll::Poll;
use std::collections::HashMap;
use crate::clib::c_int;
use std::ops::Deref;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};

struct PollWakerContext {
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

    pub fn add(&mut self, file: Arc<File>, waker: Waker) {
        self.map.insert(file.file_descriptor, waker);
    }
}

struct FileReadFuture {
    file: Arc<File>,
    bytes: Vec<u8>,
    poll_waker_context: Arc<Mutex<PollWakerContext>>
}

impl Future for FileReadFuture {
    type Output = CString;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<CString> {
        let mut newBytes =self.file.read().unwrap().into_bytes();
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

struct Executor {
    queue_receiver: Receiver<Arc<Task>>,
    queue_sender: SyncSender<Arc<Task>>,
}

impl Executor {
    fn queue(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future: Pin<Box<dyn Future<Output = ()> + 'static + Send>> = Box::pin(future);
        let future = Mutex::new(future);
        let task = Task{
            future
        };
        let task = Arc::new(task);
        self.queue_sender.send(task);
    }

    fn execute(&self) {
        let recv_result = self.queue_receiver.try_recv();
        match recv_result {
            Ok(task) => self.poll_task(task),
            Err(TryRecvError::Disconnected) => panic!("TryRecvError::Disconnected"),
            Err(TryRecvError::Empty) => self.io_block(),
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

    fn io_block(&self) {
      unimplemented!()
    }
}

struct Task {
    future: Mutex<Pin<Box<Future<Output = ()> + Send>>>
}
 impl Wake for Task {
     fn wake(self: Arc<Self>) {
         unimplemented!()
     }
 }

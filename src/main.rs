use std::ffi::CString;
use ffi_explore::clib::*;
use ffi_explore::file::File;
use ffi_explore::poll::Poll;
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use ffi_explore::fut::{FileReadFuture, PollWakerContext, Executor};

fn main() {
    let mut pipe1 = {
        let path = CString::new("pipe1").unwrap();
        let file = File::new(path).unwrap();
        Arc::new(Mutex::new(file))
    };

    let mut pipe2 = {
        let path = CString::new("pipe2").unwrap();
        let file = File::new(path).unwrap();
        Arc::new(Mutex::new(file))
    };

    let mut poll = Poll::new().unwrap();

    poll.add(pipe1.clone()).unwrap();
    poll.add(pipe2.clone()).unwrap();

    let poll_waker_context= PollWakerContext::new(poll);
    let poll_waker_context = Mutex::new(poll_waker_context);
    let poll_waker_context = Arc::new(poll_waker_context);

    let future1 = FileReadFuture::new(pipe1, poll_waker_context.clone());
    let future2 = FileReadFuture::new(pipe2, poll_waker_context.clone());

    let future1 = async {
        let result = future1.await;
        println!("Future1 finished with {}", result.to_str().unwrap())
    };

    let future2 = async {
        let result = future2.await;
        println!("Future1 finished with {}", result.to_str().unwrap())
    };

    let executor = Executor::new(poll_waker_context);

    executor.queue(future1);
    executor.queue(future2);

    executor.execute();
}

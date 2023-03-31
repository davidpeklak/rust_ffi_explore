use std::{
    sync::mpsc::Sender,
    task::{RawWaker, RawWakerVTable, Waker},
};

use crate::executor::ExecutorToken;

pub fn create_waker(sender: Sender<ExecutorToken>, executor_token: ExecutorToken) -> Waker {
    let raw_waker = create_raw_waker(sender, executor_token);
    unsafe { Waker::from_raw(raw_waker) }
}

#[derive(Clone)]
struct MyWaker {
    sender: Sender<ExecutorToken>,
    executor_token: ExecutorToken,
}

fn create_raw_waker(sender: Sender<ExecutorToken>, executor_token: ExecutorToken) -> RawWaker {
    let my_waker = MyWaker {
        sender,
        executor_token,
    };
    let my_waker = Box::new(my_waker);
    let my_waker = Box::into_raw(my_waker);

    RawWaker::new(my_waker as *const (), &VTABLE)
}

fn my_waker_wake(s: *const ()) {
    let my_waker: *mut MyWaker = s as *mut MyWaker;
    let my_waker = unsafe { Box::from_raw(my_waker) }; // when the box will be dropped, the waker will be dealocated
    my_waker.sender.send(my_waker.executor_token).unwrap();
}

fn my_waker_wake_by_ref(s: *const ()) {
    let my_waker: *mut MyWaker = s as *mut MyWaker;
    let my_waker = unsafe { my_waker.as_ref() }.unwrap();
    my_waker.sender.send(my_waker.executor_token).unwrap();
}

fn my_waker_clone(s: *const ()) -> RawWaker {
    let my_waker: *const MyWaker = s as *const MyWaker;
    let my_waker = unsafe { my_waker.as_ref() }.unwrap();
    let sender = my_waker.sender.clone();
    let executor_token = my_waker.executor_token;

    create_raw_waker(sender, executor_token)
}

fn my_waker_drop(s: *const ()) {
    let my_waker: *mut MyWaker = s as *mut MyWaker;
    let _my_waker = unsafe { Box::from_raw(my_waker) }; // when the box will be dropped, the waker will be dealocated
}

const VTABLE: RawWakerVTable = RawWakerVTable::new(
    my_waker_clone,       // clone
    my_waker_wake,        // wake
    my_waker_wake_by_ref, // wake by ref (don't decrease refcount)
    my_waker_drop,        // drop
);

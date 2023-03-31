use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::waker::create_waker;

pub type ExecutorToken = u64;

struct Executor {
    sender: Sender<ExecutorToken>,
    receiver: Receiver<ExecutorToken>,
    futures: HashMap<ExecutorToken, Pin<Box<dyn Future<Output = ()>>>>,
}

impl Executor {
    fn new() -> Executor {
        let (sender, receiver) = channel();
        Executor {
            sender,
            receiver,
            futures: HashMap::new(),
        }
    }

    fn register<F>(&mut self, future: F, executor_token: ExecutorToken)
    where
        F: Future<Output = ()> + 'static,
    {
        let future = Box::pin(future);
        self.futures.insert(executor_token, future);
        self.sender.send(executor_token).unwrap();
    }

    fn execute(&mut self) {
        while let Ok(executor_token) = self.receiver.try_recv() {
            
        }
}

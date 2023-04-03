use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::mpsc::{channel, Receiver, Sender},
    task::Context,
};

use crate::waker;

pub type ExecutorToken = u64;

pub struct Executor {
    sender: Sender<ExecutorToken>,
    receiver: Receiver<ExecutorToken>,
    futures: HashMap<ExecutorToken, Pin<Box<dyn Future<Output = ()>>>>,
}

impl Executor {
    pub fn new() -> Executor {
        let (sender, receiver) = channel();
        Executor {
            sender,
            receiver,
            futures: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, future: F, executor_token: ExecutorToken)
    where
        F: Future<Output = ()> + 'static,
    {
        let future = Box::pin(future);
        self.futures.insert(executor_token, future);
        self.sender.send(executor_token).unwrap();
    }

    pub fn execute(&mut self) {
        while let Ok(executor_token) = self.receiver.try_recv() {
            match self.futures.get_mut(&executor_token) {
                None => println!("No future found for executor_token {}", executor_token),
                Some(future) => {
                    let waker = waker::create_waker(self.sender.clone(), executor_token);
                    let mut context = Context::from_waker(&waker);
                    future.as_mut().poll(&mut context).is_ready();
                }
            }
        }
    }
}

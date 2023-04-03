use std::{cell::RefCell, rc::Rc};

use future::{executor::Executor, future::ReadNChars, reactor::Reactor};
use poll::file::File;

fn main() {
    let reactor = Reactor::new();
    let reactor = RefCell::new(reactor);
    let reactor = Rc::new(reactor);
    let mut executor = Executor::new();

    let executor_token = 1;
    let pipe_1 = File::new("pipe1").unwrap();
    let fut_1 = create_fut(pipe_1, reactor.clone());

    executor.register(fut_1, executor_token);

    loop {
        executor.execute();
        reactor.borrow_mut().poll_wait();
    }
}


async fn create_fut<'a>(file: File, reactor: Rc<RefCell<Reactor>>) {
    let result = ReadNChars::new(reactor, file, 10).await;
    println!("I read {}", result)
}

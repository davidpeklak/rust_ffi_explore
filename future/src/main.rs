use std::cell::RefCell;

use future::{executor::Executor, future::ReadNChars, reactor::Reactor};
use poll::file::File;

fn main() {
    let reactor = Reactor::new();
    let reactor = RefCell::from(reactor);
    let mut executor = Executor::new();

    let executor_token = 1;
    let pipe_1 = File::new("pipe1").unwrap();
    let mut fut_1 = create_fut(pipe_1, &reactor);

    executor.register(fut_1, executor_token);

    loop {
        executor.execute();
        reactor.borrow_mut().poll_wait();
    }
}


async fn create_fut<'a>(file: File, reactor: &'a RefCell<Reactor>) {
    let result = ReadNChars::new(reactor, file, 10).await;
    println!("I read {}", result)
}

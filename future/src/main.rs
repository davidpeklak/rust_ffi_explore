use std::{cell::RefCell, rc::Rc};

use future::{executor::Executor, future::ReadNChars, reactor::Reactor};
use poll::file::File;

fn main() {
    let reactor = Reactor::new();
    let reactor = RefCell::new(reactor);
    let reactor = Rc::new(reactor);
    let mut executor = Executor::new();

    let executor_token = 1;
    let fut_1 = create_fut(reactor.clone());

    executor.register(fut_1, executor_token);

    loop {
        executor.execute();
        reactor.borrow_mut().poll_wait();
    }
}


async fn create_fut<'a>(reactor: Rc<RefCell<Reactor>>) {
    let pipe_1 = File::new("pipe1").unwrap();
    println!("Opended pipe1 with fd {}", pipe_1.file_descriptor);
    let fut1 = ReadNChars::new(reactor.clone(), pipe_1, 10);

    let pipe_2 = File::new("pipe2").unwrap();
    println!("Opended pipe2 with fd {}", pipe_2.file_descriptor);
    let fut2 = ReadNChars::new(reactor, pipe_2, 10);

    let result_1 = fut1.await;
    println!("I read {} from pipe1", result_1);

    let result_2 = fut2.await;
    println!("I read {} from pipe2", result_2);

    println!("I read {} {}", result_1, result_2);
}

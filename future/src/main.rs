use std::{cell::RefCell, rc::Rc, path::Path, fmt::Display};

use future::{executor::Executor, future::ReadNChars, reactor::Reactor};
use poll::file::File;

fn main() {
    let reactor = Reactor::new();
    let reactor = RefCell::new(reactor);
    let reactor = Rc::new(reactor);
    let mut executor = Executor::new();

    let executor_token = 1;
    let fut = create_fut(reactor.clone(), "pipe1", "pipe2", 10);
    executor.register(fut, executor_token);

    let executor_token = 2;
    let fut = create_fut(reactor.clone(), "pipe3", "pipe4", 4);
    executor.register(fut, executor_token);

    loop {
        executor.execute();
        reactor.borrow_mut().poll_wait();
    }
}

async fn create_fut<'a, P>(reactor: Rc<RefCell<Reactor>>, p1: P, p2: P, n: usize)
where P: AsRef<Path> + Display
{
    let pipe_1 = File::new(&p1).unwrap();
    println!("Opended {} with fd {}", p1, pipe_1.file_descriptor);
    let fut1 = ReadNChars::new(reactor.clone(), pipe_1, n);

    let pipe_2 = File::new(&p2).unwrap();
    println!("Opended {} with fd {}", p2, pipe_2.file_descriptor);
    let fut2 = ReadNChars::new(reactor, pipe_2, n);

    let result_1 = fut1.await;
    println!("I read {} from {}", result_1, p1);

    let result_2 = fut2.await;
    println!("I read {} from {}", result_2, p2);

    println!("I read {} {}", result_1, result_2);
}

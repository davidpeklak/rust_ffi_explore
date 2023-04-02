use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use std::io::Read;

use future::reactor::Reactor;
use poll::{file::File, Token};

type ExecutorToken = usize;

fn main() {
    println!("Hello world");
}

pub struct ReadNChars<'a> {
    reactor: &'a RefCell<Reactor>,
    file: File,
    token: Token,
    n: usize,
    buf: String,
}

impl<'a> ReadNChars<'a> {
    fn new(reactor: &'a RefCell<Reactor>, file: File, n: usize) -> ReadNChars<'a> {
        let token = file.file_descriptor as Token;
        let buf = String::new();

        reactor.borrow_mut().poll_add(&file, token);

        ReadNChars {
            reactor,
            file,
            token,
            n,
            buf,
        }
    }
}

impl<'a> Future for ReadNChars<'a> {
    type Output = String;

    // Poll is the what drives the state machine forward and it's the only
    // method we'll need to call to drive futures to completion.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        let this = self.get_mut();
        this.file.read_to_string(&mut this.buf).unwrap();
        if this.buf.len() < this.n {
            this.reactor
                .borrow_mut()
                .add_waker(this.token, cx.waker().clone());
            Poll::Pending
        } else {
            Poll::Ready(this.buf.clone())
        }
    }
}

use std::{
    cell::RefCell,
    future::Future,
    io::Read,
    pin::Pin,
    task::{Context, Poll}, rc::Rc,
};

use crate::reactor::Reactor;
use poll::{file::File, Token};

pub struct ReadNChars {
    reactor: Rc<RefCell<Reactor>>,
    file: File,
    token: Token,
    n: usize,
    buf: String,
}

impl ReadNChars {
    pub fn new(reactor: Rc<RefCell<Reactor>>, file: File, n: usize) -> ReadNChars {
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

impl Future for ReadNChars{
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
            this.reactor
                .borrow_mut()
                .remove_waker(this.token);
            this.reactor
            .borrow_mut()
            .poll_remove(&this.file);
            Poll::Ready(this.buf.clone())
        }
    }
}

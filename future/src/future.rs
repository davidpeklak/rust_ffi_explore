use std::{
    cell::RefCell,
    future::Future,
    io::Read,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use crate::reactor::Reactor;
use poll::{file::File, Token};

pub struct ReadNChars {
    reactor: Rc<RefCell<Reactor>>,
    file: File,
    token: Token,
    n: usize,
    state: ReadNCharsState,
}

enum ReadNCharsState {
    NeverPolled(),
    InProgress(String),
    Done,
}

impl ReadNChars {
    pub fn new(reactor: Rc<RefCell<Reactor>>, file: File, n: usize) -> ReadNChars {
        let token = file.file_descriptor as Token;
        let state = ReadNCharsState::NeverPolled();

        ReadNChars {
            reactor,
            file,
            token,
            n,
            state,
        }
    }
}

impl Future for ReadNChars {
    type Output = String;

    // Poll is the what drives the state machine forward and it's the only
    // method we'll need to call to drive futures to completion.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        let this = self.get_mut();

        match &mut this.state {
            ReadNCharsState::NeverPolled() => {
                this.reactor.borrow_mut().poll_add(&this.file, this.token);
                this.state = ReadNCharsState::InProgress(String::new());
                this.reactor
                    .borrow_mut()
                    .add_waker(this.token, cx.waker().clone());
                Poll::Pending
            }
            ReadNCharsState::InProgress(ref mut buf) => {
                this.file.read_to_string(buf).unwrap();
                if buf.len() < this.n {
                    this.reactor
                        .borrow_mut()
                        .add_waker(this.token, cx.waker().clone());
                    Poll::Pending
                } else {
                    this.reactor.borrow_mut().remove_waker(this.token);
                    this.reactor.borrow_mut().poll_remove(&this.file);
                    let result = Poll::Ready(buf.clone());
                    this.state = ReadNCharsState::Done;
                    result
                }
            }
            ReadNCharsState::Done => {
                panic!("ReadNChars already done")
            }
        }
    }
}

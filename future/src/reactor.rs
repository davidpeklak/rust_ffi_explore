use std::{collections::HashMap, sync::mpsc::Sender, task::Waker};

use poll::{file::File, Poll, Token};
pub struct Reactor {
    token_waker_map: HashMap<Token, Waker>,
    poll: Poll,
}

impl Reactor {
    pub fn poll_wait(&mut self) {
        println!("poll.wait...");
        match self.poll.wait() {
            Err(e) => println!("Error: {:?}", e),
            Ok(None) => println!("Nothing polled"),
            Ok(Some(token)) => {
                println!("Received event token {:?}", token);
                match self.token_waker_map.remove(&token) {
                    None => println!("No waker for token {}", token),
                    Some(waker) => {
                        println!("Found waker for token {}", token);
                        waker.wake();
                    }
                }
            }
        }
    }

    pub fn poll_add(&mut self, file: &File, token: Token) {
        self.poll.add(file, token).unwrap();
    }

    pub fn add_waker(&mut self, token: Token, waker: Waker) {
        self.token_waker_map.insert(token, waker);
    }
}

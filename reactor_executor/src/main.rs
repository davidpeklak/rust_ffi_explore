use poll::file::File;
use poll::{Poll, Token};
use std::io::Read;
use std::sync::mpsc::{channel, Receiver, Sender};

struct Reactor {
    sender: Sender<Token>,
    poll: Poll,
}

impl Reactor {
    fn poll_wait(&self) {
        println!("poll.wait...");
        match self.poll.wait() {
            Err(e) => println!("Error: {:?}", e),
            Ok(None) => println!("Nothing polled"),
            Ok(Some(token)) => {
                println!("Received event token {:?}", token);
                self.sender.send(token).unwrap();
            }
        }
    }

    fn poll_add(&mut self, file: &File, token: Token) {
        self.poll.add(file, token).unwrap();
    }
}

struct Executor {
    receiver: Receiver<Token>,
    actions: Vec<(Token, Box<dyn FnMut()>)>,
}

impl Executor {
    fn new(receiver: Receiver<Token>) -> Executor {
        Executor {
            receiver,
            actions: Vec::new(),
        }
    }

    fn register<F>(&mut self, token: Token, action: F)
    where
        F: FnMut() + 'static,
    {
        self.actions.push((token, Box::new(action)));
    }

    fn execute(&mut self) {
        while let Ok(token) = self.receiver.try_recv() {
            self.actions
                .iter_mut()
                .filter(|(t, _)| *t == token)
                .for_each(|(_, f)| f());
        }
    }
}


fn main() {
    let poll = Poll::new().unwrap();

    let (sender, receiver) = channel();

    let mut reactor = Reactor { sender, poll };
    let mut executor = Executor::new(receiver);

    let token_1 = 1;
    let mut pipe_1 = File::new("pipe1").unwrap();
    reactor.poll_add(&pipe_1, token_1);
    executor.register(token_1, move || {
        let mut str = String::new();
        pipe_1.read_to_string(&mut str).unwrap();
        println!("Read {:?} from file {:?}", str, pipe_1);
    });

    let token_2 = 2;
    let mut pipe_2 = File::new("pipe2").unwrap();
    reactor.poll_add(&pipe_2, token_2);
    executor.register(token_2, move || {
        let mut str = String::new();
        pipe_2.read_to_string(&mut str).unwrap();
        println!("Read {:?} from file {:?}", str, pipe_2);
    });


    loop {
        println!("Iterating runtime loop");
        reactor.poll_wait();
        executor.execute();
    }
}

use poll::{file::File, Poll, Token};
use std::{collections::HashMap, io::Read};

fn main() {
    let mut poll = Poll::new().unwrap();

    let mut token_map: HashMap<Token, File> = HashMap::new();

    let token_1 = 1;
    let pipe1 = File::new("pipe1").unwrap();
    poll.add(&pipe1, token_1).unwrap();
    token_map.insert(token_1, pipe1);

    let token_2 = 2;
    let pipe2 = File::new("pipe2").unwrap();
    poll.add(&pipe2, token_2).unwrap();
    token_map.insert(token_2, pipe2);

    loop {
        println!("poll.wait...");
        match poll.wait() {
            Err(e) => println!("Error: {:?}", e),
            Ok(None) => println!("Nothing polled"),
            Ok(Some(token)) => {
                println!("Received event for token {:?}", token);
                let mut file = token_map.get_mut(&token).unwrap();

                let mut str = String::new();
                file.read_to_string(&mut str).unwrap();
                println!("Read {:?} from file {:?}", str, file);
                if str.starts_with("exit") {
                    println!("Exiting");
                    return;
                }
            }
        }
    }
}

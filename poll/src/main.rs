use poll::{file::File, Poll};
use std::io::Read;

fn main() {
    let mut poll = Poll::new().unwrap();

    let pipe1 = File::new("pipe1").unwrap();

    let pipe2 = File::new("pipe2").unwrap();

    poll.add(pipe1).unwrap();

    poll.add(pipe2).unwrap();

    loop {
        println!("poll.wait...");
        match poll.wait() {
            Err(e) => println!("Error: {:?}", e),
            Ok(None) => println!("Nothing polled"),
            Ok(Some(mut file)) => {
                println!("Received event for file {:?}", file);
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

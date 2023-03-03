use poll::{file::File, Poll};
use std::{ffi::CString, io::Read};

fn main() {
    let mut poll = Poll::new().unwrap();

    let pipe1 = {
        let path = CString::new("pipe1").unwrap();
        File::new(path).unwrap()
    };

    let pipe2 = {
        let path = CString::new("pipe2").unwrap();
        File::new(path).unwrap()
    };

    poll.add(pipe1).map_err(|(_, e)| e).unwrap();

    poll.add(pipe2).map_err(|(_, e)| e).unwrap();

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

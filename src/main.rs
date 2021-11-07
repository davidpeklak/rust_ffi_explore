use std::ffi::CString;
use ffi_explore::clib::*;
use ffi_explore::file::File;
use ffi_explore::poll::Poll;
use std::sync::Arc;
use std::ops::Deref;

fn main() {
    let mut pipe1 = {
        let path = CString::new("pipe1").unwrap();
        let file = File::new(path).unwrap();
        Arc::new(file)
    };

    let mut pipe2 = {
        let path = CString::new("pipe2").unwrap();
        let file = File::new(path).unwrap();
        Arc::new(file)
    };

    let mut poll = Poll::new().unwrap();

    poll.add(pipe1.clone()).unwrap();
    poll.add(pipe2.clone()).unwrap();

    loop {
        println!("Waiting");
        let file = poll.wait().unwrap();
        if let Some(file) = poll.wait().unwrap() {
            println!("poll.wait() returned event for {}", file.file_descriptor);
            let chunk = file.read().unwrap();
            println!("pipe1: {}", chunk.to_str().unwrap());
        }
        else {
            println!("Wait returned None")
        }
    }
}

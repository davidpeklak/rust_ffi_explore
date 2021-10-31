use std::ffi::CString;
use ffi_explore::clib::*;
use ffi_explore::file::File;
use ffi_explore::poll::Poll;

fn main() {
    let mut pipe1 = {
        let path = CString::new("pipe1").unwrap();
        File::new(path).unwrap()
    };

    let mut pipe2 = {
        let path = CString::new("pipe2").unwrap();
        File::new(path).unwrap()
    };

    let mut poll = Poll::new().unwrap();

    poll.add(&pipe1).unwrap();
    poll.add(&pipe2).unwrap();

    loop {
        println!("Waiting");
        poll.wait().unwrap();
        let chunk = pipe1.read().unwrap();
        println!("pipe1: {}", chunk.to_str().unwrap());
        let chunk = pipe2.read().unwrap();
        println!("pipe2: {}", chunk.to_str().unwrap());
    }
}

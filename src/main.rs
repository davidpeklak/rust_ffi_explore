use std::ffi::CString;
use ffi_explore::clib::*;
use ffi_explore::file::File;
use ffi_explore::poll::Poll;

fn main() {
    let path = CString::new("pipe1").unwrap();

    let mut file = File::new(path).unwrap();

    let mut poll = Poll::new().unwrap();

    poll.add(&file).unwrap();

    loop {
        println!("Waiting");
        poll.wait().unwrap();
        let chunk = file.read().unwrap();
        println!("{}", chunk.to_str().unwrap())
    }
}

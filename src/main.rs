use std::ffi::CString;
use ffi_explore::clib::*;
use ffi_explore::file::File;
use ffi_explore::poll::Poll;

fn main() {
    let path = CString::new("pipe1").unwrap();

    let file = File::new(path).unwrap();

    let mut poll = Poll::new().unwrap();

    poll.add(&file).unwrap();

    loop {
        println!("Waiting");
        poll.wait().unwrap();

        loop {
            let c = unsafe { fgetc(file.file) };
            if c == EOF {
                break;
            } else {
                print!("{}", unsafe { std::char::from_u32_unchecked(c as u32) })
            }
        }
    }
}

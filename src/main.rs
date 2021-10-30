use std::ffi::CString;
use ffi_explore::clib::*;
use ffi_explore::file::File;

fn main() {
    let path = CString::new("pipe1").unwrap();

    let file = File::new(path).unwrap();

    let poll = unsafe {
        epoll_create(1)
    };

    if poll < 1 {
        panic!("poll is < 0")
    }

    let mut event= epoll_event {
        events: EPOLLIN,
        data: 0
    };

    let result = unsafe {
        let op = EPOLL_CTL_ADD;
        epoll_ctl(poll, op, file.file_descriptor, std::ptr::addr_of_mut!(event))
    };

    if result < 0 {
        panic!("epoll_ctl returned < 0")
    }

    let mut events = [
        epoll_event {
            events: 0,
            data: 0
        },
        epoll_event {
            events: 0,
            data: 0
        },
    ];

    let number_of_events = unsafe {
        epoll_wait(poll, events.as_mut_ptr(), 2, -1)
    };

    if number_of_events < 1 {
        panic!("no events received")
    }

    loop {
        let c = unsafe { fgetc(file.file) };
        if c == EOF {
            break;
        } else {
            print!("{}", unsafe { std::char::from_u32_unchecked(c as u32) })
        }
    }
}

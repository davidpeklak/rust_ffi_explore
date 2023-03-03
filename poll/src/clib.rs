use std::os::raw::c_char;

pub enum FILE {}

#[allow(non_camel_case_types, dead_code)]
pub type size_t = usize;
#[allow(non_camel_case_types, dead_code)]
pub type ssize_t = isize;
#[allow(non_camel_case_types)]
pub type c_int = i32;
#[allow(non_camel_case_types, dead_code)]
pub type c_short = i16;

pub const EOF: c_int = -1;
#[allow(dead_code)]
pub const O_RDONLY: c_int = 0;
pub const O_RDWR: c_int = 2;
pub const O_NONBLOCK: c_int = 2048;

pub const EPOLLIN: u32 = 0x1;
pub const EPOLL_CTL_ADD: c_int = 1;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct epoll_event {
    pub events: u32,
    pub data: u64,
}

#[link(name = "c")]
extern "C" {
    // https://www.gnu.org/software/libc/manual/html_node/Opening-and-Closing-Files.html#Opening-and-Closing-Files
    pub fn open(filename: *const c_char, flags: c_int) -> c_int;

    // https://man7.org/linux/man-pages/man3/fopen.3.html
    pub fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;

    // https://www.gnu.org/software/libc/manual/html_node/Character-Input.html#Character-Input
    pub fn fgetc(stream: *mut FILE) -> c_int;

    // https://man7.org/linux/man-pages/man2/epoll_create.2.html
    // size is ignored, but must be > 0
    pub fn epoll_create(size: c_int) -> c_int;

    // https://man7.org/linux/man-pages/man2/epoll_ctl.2.html
    pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;

    // https://man7.org/linux/man-pages/man2/epoll_wait.2.html
    pub fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
}

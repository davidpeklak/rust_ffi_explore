use std::os::raw::c_char;
use std::ffi::CString;

pub enum FILE {}

#[allow(non_camel_case_types)]
pub type size_t = usize;
#[allow(non_camel_case_types)]
pub type ssize_t = isize;
#[allow(non_camel_case_types)]
pub type c_int = i32;
#[allow(non_camel_case_types)]
pub type c_short = i16;

pub const EOF: c_int = -1;
pub const O_RDONLY: c_int = 0;
pub const O_RDWR: c_int = 2;

#[allow(non_camel_case_types)]
#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[link(name = "c")]
extern "C" {
    // https://www.gnu.org/software/libc/manual/html_node/Opening-and-Closing-Files.html#Opening-and-Closing-Files
    fn open(filename: *const c_char, flags: c_int) -> c_int;

    // https://www.gnu.org/software/libc/manual/html_node/
    // fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;

    // https://man7.org/linux/man-pages/man3/fopen.3.html
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;

    // https://www.gnu.org/software/libc/manual/html_node/Character-Input.html#Character-Input
    fn fgetc(stream: *mut FILE) -> c_int;
    // https://man7.org/linux/man-pages/man3/fileno.3.html
    fn fileno(stream: *mut FILE) -> c_int;
    // https://man7.org/linux/man-pages/man2/epoll_create.2.html
    // size is ignored, but must be > 0
    fn epoll_create(size: c_int) -> c_int;
}

fn main() {
    let file_name = CString::new("pipe1").unwrap();
    let file_descriptor  = unsafe {
        let flags = O_RDWR;
        open(file_name.as_ptr(), flags)
    };

    if file_descriptor < 0 {
        panic!("file descriptor is < 0");
    }

    println!("File descriptor is {}", file_descriptor);

    let file= unsafe {
        let mode = CString::new("r").unwrap();
        fdopen(file_descriptor, mode.as_ptr())
    };

    if file.is_null() {
        panic!("FILE is null");
    }

    println!("Stream opened");

    loop {
        let c = unsafe { fgetc(file) };
        if c == EOF {
            break;
        } else {
            print!("{}", unsafe { std::char::from_u32_unchecked(c as u32) })
        }
    }
}

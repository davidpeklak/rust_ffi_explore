use std::os::raw::c_char;
use std::ffi::{CStr, CString};

pub enum FILE {}
#[allow(non_camel_case_types)]
pub type size_t = usize;
#[allow(non_camel_case_types)]
pub type ssize_t = isize;

#[link(name = "c")]
extern "C" {
    // https://www.gnu.org/software/libc/manual/html_node/
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    // https://www.gnu.org/software/libc/manual/html_node/Line-Input.html
    fn getline (line_ptr: *const *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
}

fn main() {
    let file_name = CString::new("src/main.rs").unwrap();
    let open_type = CString::new("r").unwrap();
    let file = unsafe {
        fopen(file_name.as_ptr(), open_type.as_ptr())
    };

    if file.is_null() {
        println!("file is null")
    } else {
        println!("file is not null");
        let mut v: [c_char; 200] = [0; 200];
        let mut n = 200;
        let line = unsafe {
            let c_ptr: *mut c_char = std::ptr::addr_of_mut!(v[0]);
            let c_ptr_ptr: *const *mut c_char = std::ptr::addr_of!(c_ptr);
            let n_ptr: *mut size_t = std::ptr::addr_of_mut!(n);
            getline(c_ptr_ptr, n_ptr, file);
            CStr::from_ptr(c_ptr)
        };
        println!("line: {}", line.to_str().unwrap());
    }

}

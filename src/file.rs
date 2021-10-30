use crate::clib::{open, O_RDWR, c_int, fdopen, FILE};
use std::ffi::CString;

#[derive(Debug)]
pub enum Error {
    FileDescriptorError,
    FileStructError,
}

type Result<T> = std::result::Result<T, Error>;

pub struct File {
    pub file_descriptor: c_int,
    pub file: *mut FILE,
}

impl File {
    pub fn new(path: CString) -> Result<File> {
        let file_descriptor  = unsafe {
            let flags = O_RDWR;
            open(path.as_ptr(), flags)
        };

        if file_descriptor < 0 {
            return Err(Error::FileDescriptorError)
        };

        let file = unsafe {
            let mode = CString::new("r").unwrap();
            fdopen(file_descriptor, mode.as_ptr())
        };

        if file.is_null() {
            return Err(Error::FileStructError)
        };

        Ok(File {
            file_descriptor,
            file
        })
    }
}

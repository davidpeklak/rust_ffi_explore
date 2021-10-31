use crate::clib::{open, O_RDWR, c_int, fdopen, FILE, O_RDONLY, O_NONBLOCK};
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
            // this is the only combination of flags that seems to work.
            // O_RDWR only blocks on fgetc(...) and thus never gets out of the loop of reading chars
            // O_RDONLY | O_NONBLOCK closes the file when the process that writes to it closes, so
            // e. g. it closes after `echo "Foo" > pipe1`
            // O_RDONLY only blocks on opening the file, i.e. the program does not reach the point
            // of creating the Poll until something is written to the pipe
            let flags = O_RDWR | O_NONBLOCK;
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

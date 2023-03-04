use crate::clib::{c_int, fdopen, fgetc, open, path_to_c_string, EOF, FILE, O_NONBLOCK, O_RDWR};
use std::{
    ffi::{CString, NulError},
    path::Path,
};

#[derive(Debug)]
pub enum Error {
    FileDescriptorError,
    FileStructError,
    FileStringNulError,
}

impl From<NulError> for Error {
    fn from(_: NulError) -> Self {
        Error::FileStringNulError
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct File {
    pub file_descriptor: c_int,
    pub file: *mut FILE,
}

impl File {
    pub fn new<P>(path: P) -> Result<File>
    where
        P: AsRef<Path>,
    {
        let c_string = path_to_c_string(path);
        let file_descriptor = unsafe {
            // this is the only combination of flags that seems to work.
            // O_RDWR only blocks on fgetc(...) and thus never gets out of the loop of reading chars
            // O_RDONLY | O_NONBLOCK closes the file when the process that writes to it closes, so
            // e. g. it closes after `echo "Foo" > pipe1`
            // O_RDONLY only blocks on opening the file, i.e. the program does not reach the point
            // of creating the Poll until something is written to the pipe
            let flags = O_RDWR | O_NONBLOCK;
            open(c_string.as_ptr(), flags)
        };

        if file_descriptor < 0 {
            return Err(Error::FileDescriptorError);
        };

        let file = unsafe {
            let mode = CString::new("r").unwrap();
            fdopen(file_descriptor, mode.as_ptr())
        };

        if file.is_null() {
            return Err(Error::FileStructError);
        };

        Ok(File {
            file_descriptor,
            file,
        })
    }
}

impl std::io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        for i in 0..buf.len() {
            let c = unsafe { fgetc(self.file) };
            if c == EOF {
                return Ok(i);
            } else {
                buf[i] = c as u8;
            }
        }
        Ok(buf.len())
    }
}

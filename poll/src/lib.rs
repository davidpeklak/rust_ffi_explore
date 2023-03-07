use crate::clib::{
    c_int, epoll_create, epoll_ctl, epoll_event, epoll_wait, EPOLLIN, EPOLL_CTL_ADD,
};
use crate::file::File;

mod clib;
pub mod file;

#[derive(Debug)]
pub enum Error {
    ErrorOnCreate,
    ErrorOnAdd,
    ErrorOnWait,
}

pub type Result<T> = std::result::Result<T, Error>;

pub type Token = u64;

pub struct Poll {
    fd: c_int,
}

impl Poll {
    pub fn new() -> Result<Poll> {
        let fd = unsafe { epoll_create(1) };

        if fd < 1 {
            Err(Error::ErrorOnCreate)
        } else {
            Ok(Poll { fd })
        }
    }

    /// Adds a `File`to the Poll. Note that the file is consumed.
    /// In case of an error, the  file is returned in a tuple together with the error.
    pub fn add(&mut self, file: &File, token: Token) -> std::result::Result<(), Error> {
        let file_descriptor = file.file_descriptor;
        let mut event = epoll_event {
            events: EPOLLIN,
            data: token,
        };

        let result = unsafe {
            let op = EPOLL_CTL_ADD;
            epoll_ctl(self.fd, op, file_descriptor, std::ptr::addr_of_mut!(event))
        };

        if result < 0 {
            Err(Error::ErrorOnAdd)
        } else {
            Ok(())
        }
    }

    pub fn wait(&self) -> Result<Option<Token>> {
        let mut events = [epoll_event { events: 0, data: 0 }];

        let number_of_events = unsafe { epoll_wait(self.fd, events.as_mut_ptr(), 1, -1) };

        if number_of_events < 1 {
            Err(Error::ErrorOnWait)
        } else if number_of_events == 0 {
            Ok(None)
        } else
        /* number_of_events must be 1 */
        {
            let token = events[0].data as Token;
            Ok(Some(token))
        }
    }
}

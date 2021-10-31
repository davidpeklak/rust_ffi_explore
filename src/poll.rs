use crate::clib::{c_int, epoll_create, epoll_event, EPOLLIN, EPOLL_CTL_ADD, epoll_ctl, epoll_wait};
use crate::file::File;

#[derive(Debug)]
pub enum Error {
    ErrorOnCreate,
    ErrorOnAdd,
    ErrorOnWait
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Poll {
    fd: c_int,
}

impl Poll {
    pub fn new() -> Result<Poll> {
        let fd = unsafe {
            epoll_create(1)
        };

        if fd < 1 {
            return Err(Error::ErrorOnCreate);
        }

        Ok(Poll {
            fd
        })
    }

    pub fn add(&mut self, file: &File) -> Result<()> {
        let mut event= epoll_event {
            events: EPOLLIN,
            data: 0
        };

        let result = unsafe {
            let op = EPOLL_CTL_ADD;
            epoll_ctl(self.fd, op, file.file_descriptor, std::ptr::addr_of_mut!(event))
        };

        if result < 0 {
            return Err(Error::ErrorOnAdd);
        }

        Ok(())
    }

    pub fn wait(&self) -> Result<()> {
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
            epoll_wait(self.fd, events.as_mut_ptr(), 2, -1)
        };

        if number_of_events < 1 {
            return Err(Error::ErrorOnWait);
        }

        Ok(())
    }
}

pub struct PollEvents<const SIZE: usize> {
    events: [epoll_event; SIZE],
}

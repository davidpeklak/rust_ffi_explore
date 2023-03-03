use crate::clib::{
    c_int, epoll_create, epoll_ctl, epoll_event, epoll_wait, EPOLLIN, EPOLL_CTL_ADD,
};
use crate::file::File;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

mod clib;
pub mod file;

#[derive(Debug)]
pub enum Error {
    ErrorOnCreate,
    ErrorOnAdd,
    ErrorOnWait,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Poll {
    file_map: HashMap<c_int, RefCell<File>>,
    fd: c_int,
}

impl Poll {
    pub fn new() -> Result<Poll> {
        let fd = unsafe { epoll_create(1) };

        if fd < 1 {
            return Err(Error::ErrorOnCreate);
        }

        let file_map = HashMap::new();

        Ok(Poll { file_map, fd })
    }

    /// Adds a `File`to the Poll. Note that the file is consumed.
    /// In case of an error, the  file is returned in a tuple together with the error.
    pub fn add(&mut self, file: File) -> std::result::Result<(), (File, Error)> {
        let file_descriptor = file.file_descriptor;
        let mut event = epoll_event {
            events: EPOLLIN,
            data: file_descriptor as u64,
        };

        let result = unsafe {
            let op = EPOLL_CTL_ADD;
            epoll_ctl(self.fd, op, file_descriptor, std::ptr::addr_of_mut!(event))
        };

        if result < 0 {
            return Err((file, Error::ErrorOnAdd));
        } else {
            self.file_map.insert(file_descriptor, RefCell::new(file));
            Ok(())
        }
    }

    pub fn wait(&self) -> Result<Option<RefMut<File>>> {
        let mut events = [epoll_event { events: 0, data: 0 }];

        let number_of_events = unsafe { epoll_wait(self.fd, events.as_mut_ptr(), 1, -1) };

        if number_of_events < 1 {
            return Err(Error::ErrorOnWait);
        } else if number_of_events == 0 {
            return Ok(None);
        } else
        /* number_of_events must be 1 */
        {
            let file_descriptor = events[0].data as c_int;
            if let Some(file_from_map) = self.file_map.get(&file_descriptor) {
                return Ok(Some(file_from_map.borrow_mut()));
            } else {
                return Err(Error::ErrorOnWait);
            }
        }
    }
}

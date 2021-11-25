use crate::clib::{c_int, epoll_create, epoll_event, EPOLLIN, EPOLL_CTL_ADD, epoll_ctl, epoll_wait};
use crate::file::File;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    ErrorOnCreate,
    ErrorOnAdd,
    ErrorOnWait
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Poll {
    file_map: HashMap<c_int, Arc<Mutex<File>>>,
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

        let file_map = HashMap::new();

        Ok(Poll {
            file_map,
            fd
        })
    }

    pub fn add(&mut self, file: Arc<Mutex<File>>) -> Result<()> {
        let mut event= epoll_event {
            events: EPOLLIN,
            data: file.lock().unwrap().file_descriptor as u64,
        };

        let result = unsafe {
            let op = EPOLL_CTL_ADD;
            epoll_ctl(self.fd, op, file.lock().unwrap().file_descriptor, std::ptr::addr_of_mut!(event))
        };

        if result < 0 {
            return Err(Error::ErrorOnAdd);
        } else {
            let file_descriptor = {
                file.lock().unwrap().file_descriptor
            };
            self.file_map.insert(file_descriptor, file);
            Ok(())
        }
    }

    pub fn wait(&self) -> Result<Option<Arc<Mutex<File>>>> {
        let mut events = [
            epoll_event {
                events: 0,
                data: 0
            }
        ];

        let number_of_events = unsafe {
            epoll_wait(self.fd, events.as_mut_ptr(), 1, -1)
        };

        if number_of_events < 1 {
            return Err(Error::ErrorOnWait);
        } else if number_of_events == 0 {
            return Ok(None)
        } else /* number_of_events must be 1 */ {
            let file_descriptor = events[0].data as c_int;
            if let Some(file_from_map) = self.file_map.get(&file_descriptor) {
                return Ok(Some(file_from_map.clone()));
            } else {
                return Err(Error::ErrorOnWait);
            }
        }

    }
}

struct PollEvents<const SIZE: usize> {
    events: [epoll_event; SIZE],
}

use std::io::{Error, Result};

use crate::ffi::syscalls::epoll_wait;

use super::syscalls::{close, epoll_create1, epoll_ctl, EpollEvent, EPOLL_CTL_ADD};

type Events = Vec<EpollEvent>;

/// epoll registry
pub(crate) struct Registry {
    /// raw epoll fd
    raw_fd: i32,
}

/// takes in an error code, if negative returns an error
#[inline]
pub(crate) fn return_result<T>(result: T, code: i32) -> Result<T> {
    if code < 0 {
        Err(Error::last_os_error())
    } else {
        Ok(result)
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe { close(self.raw_fd) };
        if res < 0 {
            let err = Error::last_os_error();
            eprintln!("ERROR: {:?}", err);
        }
    }
}

pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        let fd = unsafe { epoll_create1(0) };
        let res = Self {
            registry: Registry { raw_fd: fd },
        };
        return_result(res, fd)
    }

    pub fn poll(&self, timeout: Option<i32>, events: &mut Events) -> Result<()> {
        let epfd = self.registry.raw_fd;
        // if none is passed, the time out won't happen
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;
        debug_assert!(max_events > 0);
        let res = unsafe { epoll_wait(epfd, events.as_mut_ptr(), max_events, timeout) };
        if res < 0 {
            Err(Error::last_os_error())
        } else {
            // UNSAFE: the epoll_wait syscall returns the total amount of events as its return result,
            // up to the max capacity of the buffer passed in
            unsafe { events.set_len(res as usize) };
            Ok(())
        }
    }

    pub fn register(&self, source: i32, token: usize, interests: i32) -> Result<()> {
        let mut event = EpollEvent::new(interests, token);
        // we're registering this event here
        let op = EPOLL_CTL_ADD;
        let err = unsafe { epoll_ctl(self.registry.raw_fd, op, source, &mut event) };
        return_result((), err)
    }
}

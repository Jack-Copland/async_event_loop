// epoll flags

/// epoll read in flag
pub const EPOLLIN: i32 = 0x001;
/// epoll write out flag
pub const EPOLLOUT: i32 = 0x004;
/// request edge-triggered epoll
pub const EPOLLET: i32 = 1 << 31;
/// Requests one-shot notification for the associated file
/// descriptor.  This means that after an event notified for
/// the file descriptor by epoll_wait(2), the file descriptor
/// is disabled in the interest list and no other events will
/// be reported by the epoll interface.  The user must call
/// epoll_ctl() with EPOLL_CTL_MOD to rearm the file
/// descriptor with a new event mask.
pub const EPOLLONESHOT: i32 = 1 << 30;
/// add an fd to the interface
pub const EPOLL_CTL_ADD: i32 = 1;
/// add an fd to the interface
pub const EPOLL_CTL_DEL: i32 = 2;
/// add an fd to the interface
pub const EPOLL_CTL_MOD: i32 = 3;

#[repr(C, packed)]
pub struct EpollEvent {
    event: i32,
    data: usize,
}

impl EpollEvent {
    pub fn new(event: i32, data: usize) -> Self {
        Self { event, data }
    }
    pub fn token(&self) -> usize {
        self.data
    }
}

extern "C" {
    /// close a file descriptor
    pub(crate) fn close(fd: i32) -> i32;
    /// returns an fd to a new epoll instance
    pub(crate) fn epoll_create1(flags: i32) -> i32;
    /// System call to add, modify or remove entries in the interest list of the epoll instance
    /// referred to by the file descriptor epfd. It requests that the operation op be performed for
    /// the target file descriptor, fd.
    ///
    /// Valid values for the op argument are:
    /// `EPOLL_CTL_ADD`: Adds an entry to the interest list. The entry includes the fd, a reference to
    /// the corresponding open fd
    /// `EPOLL_CTL_MOD`: Change the settings of an fd in the interest list to the new settings
    /// specified in event
    /// EPOLL_CTL_DELETE: Remove the target file descriptor fd from the interest list. The event
    /// argument is ignored and can be NULL
    ///
    /// For a list of valid events, refer to man epoll_ctl, or this page `https://man7.org/linux/man-pages/man2/epoll_ctl.2.html`
    ///
    /// It returns 0 on success, and -1 on error
    pub(crate) fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut EpollEvent) -> i32;
    /// The epoll_wait() system call waits for events on the epoll(7)
    /// instance referred to by the file descriptor epfd.  The buffer
    /// pointed to by events is used to return information from the ready
    /// list about file descriptors in the interest list that have some
    /// events available.  Up to maxevents are returned by epoll_wait().
    /// The maxevents argument must be greater than zero.
    ///
    /// The timeout argument specifies the number of milliseconds that
    /// epoll_wait() will block.  Time is measured against the
    /// CLOCK_MONOTONIC clock.
    /// The timeout argument specifies the number of milliseconds that
    /// epoll_wait() will block.  Time is measured against the
    /// CLOCK_MONOTONIC clock.
    ///
    /// A call to epoll_wait() will block until either:

    /// •  a file descriptor delivers an event;

    /// •  the call is interrupted by a signal handler; or

    /// •  the timeout expires.
    ///
    /// Note that the timeout interval will be rounded up to the system
    /// clock granularity, and kernel scheduling delays mean that the
    /// blocking interval may overrun by a small amount.  Specifying a
    /// timeout of -1 causes epoll_wait() to block indefinitely, while
    /// specifying a timeout equal to zero causes epoll_wait() to return
    /// immediately, even if no events are available.
    ///
    /// The events field is a bit mask that indicates the events that
    /// have occurred for the corresponding open file description.
    ///
    /// On success, epoll_wait returns the number of file descriptors ready for the requested I/O
    /// operation, 0 if no file descriptor is ready, or -1 if error
    pub(crate) fn epoll_wait(
        epfd: i32,
        epoll_event: *mut EpollEvent,
        maxevents: i32,
        timeout: i32,
    ) -> i32;
}

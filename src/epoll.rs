use libc::*;


struct EventHandler {
    epollfd: i32,
}


impl EventHandler {
    pub fn create() -> EventHandler {
        unsafe {
            let efd = epoll_create1(0);
            return EventHandler{ epollfd: efd };
        }
    }
}

use libc::*;


pub struct EventHandler {
    epollfd: i32,
}


impl EventHandler {
    pub fn create() -> EventHandler {
        unsafe {
            let efd = epoll_create1(0);
            return EventHandler{ epollfd: efd };
        }
    }



    pub fn poll(&self) {
        let mut event = [epoll_event { events: 0, u64: 0 }; 32];
        unsafe {
            let result = epoll_wait(self.epollfd, event.as_mut_ptr(), event.len() as i32, -1);
        }

    }
}

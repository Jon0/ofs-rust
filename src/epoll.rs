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



    pub fn poll(&self) {
        //let mut event = [epoll_event { events: 0, u64: 0 }; 32];
        let mut event = epoll_event { events: 0, u64: 0 };
        unsafe {
            let result = epoll_wait(self.epollfd, &mut event, 1, -1);
        }

    }
}

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


    pub fn add_source(&self, fd: i32) {
        let mut ev = epoll_event { events: 0, u64: 0 };
        unsafe {
            let result = epoll_ctl(self.epollfd, EPOLL_CTL_ADD, fd, &mut ev);
        }
    }



    pub fn poll(&self) {
        let mut event = [epoll_event { events: 0, u64: 0 }; 32];
        unsafe {
            let result = epoll_wait(self.epollfd, event.as_mut_ptr(), event.len() as i32, -1);
        }

    }
}


pub trait EventSource {
    fn listen(&self, handler: &mut EventHandler);
}

extern crate fuse;
extern crate libc;

use std::mem;
use libc::*;


pub struct SockAddr4 {

}


impl SockAddr4 {
    pub fn bind(&self, fd: i32) -> i32 {
        let addr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_addr: in_addr { s_addr: 0 },
            sin_port: 3648,
            sin_zero: [0; 8]
        };
        unsafe {
            let addr_ptr: *const sockaddr =  mem::transmute(&addr);
            let addr_size = mem::size_of::<sockaddr_in>() as u32;
            return bind(fd, addr_ptr, addr_size);
        }
    }
}


/*
 * readable stream
 * add generic address type
 */
pub struct SockStream {
    fd: i32
}


impl SockStream {
    pub fn read<T>(&self, mut obj: &mut T) {
        unsafe {
            let obj_ptr: *mut c_void = mem::transmute(&mut obj);
            let obj_size = mem::size_of::<T>() as usize;
            let read_size = read(self.fd, obj_ptr, obj_size);
            if read_size < obj_size as isize {
                println!("size mismatch");
            }
        }
    }
}



pub struct SockAcceptor {
    fd: i32,
}


impl SockAcceptor {
    pub fn open() -> SockAcceptor {
        let addr = SockAddr4 {};
        unsafe {
            let fd = socket(AF_INET, SOCK_STREAM, 0);
            let err = addr.bind(fd);
            if err < 0 {
                println!("error binding socket");
                close(fd);
            }
            else {
                listen(fd, 5);
            }
            return SockAcceptor { fd: fd };
        }
    }


    pub fn accept(&self) -> SockStream {
        let mut addr = sockaddr_in {
            sin_family: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_port: 0,
            sin_zero: [0; 8]
        };
        unsafe {
            let addr_ptr: *mut sockaddr = mem::transmute(&mut addr);
            let mut addr_size = mem::size_of::<sockaddr_in>() as u32;
            let addr_size_ptr = &mut addr_size;
            let newfd = accept(self.fd, addr_ptr, addr_size_ptr);
            return SockStream { fd: newfd };
        }
    }
}


fn main() {
    let acceptor = SockAcceptor::open();
    loop {
        let socket = acceptor.accept();
        println!("socket connected");
        let mut input: u32 = 0;
        socket.read(&mut input);
    }
}

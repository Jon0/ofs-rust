extern crate fuse;
extern crate libc;

pub mod message;

use std::mem;
use libc::*;
use message::*;


pub trait Bindable {
    fn bind(&self, fd: i32) -> i32;
}


pub struct SockAddr4 {
    port: u16,
}


impl Bindable for SockAddr4 {
    fn bind(&self, fd: i32) -> i32 {
        let addr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_addr: in_addr { s_addr: 0 },
            sin_port: self.port.to_be(),
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
    pub fn read<T>(&self, mut obj: &mut T) -> Result<usize, i32> {
        unsafe {
            let obj_ptr: *mut c_void = mem::transmute(&mut obj);
            let obj_size = mem::size_of::<T>() as usize;
            let read_size = read(self.fd, obj_ptr, obj_size);
            if read_size < 0 {
                return Err(read_size as i32);
            }
            else {
                return Ok(read_size as usize);
            }
        }
    }
}



pub struct SockAcceptor {
    fd: i32,
}


impl SockAcceptor {
    pub fn open<T: Bindable>(addr: &T) -> Result<SockAcceptor, i32> {
        unsafe {
            let fd = socket(AF_INET, SOCK_STREAM, 0);
            let err = addr.bind(fd);
            if err < 0 {
                close(fd);
                return Err(err);
            }
            else {
                listen(fd, 5);
                return Ok(SockAcceptor { fd: fd });
            }
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
    let addr = SockAddr4 { port: 1234 };
    let serv = Server::init();
    match SockAcceptor::open(&addr) {
        Ok(acceptor) => loop {
            let socket = acceptor.accept();
            println!("socket connected");
            let mut input: u32 = 0;
            match socket.read(&mut input) {
                Ok(count) => println!("Read: {}", count),
                Err(err) => println!("Error: {}", err),
            }
        },
        Err(err) => println!("Error: {}", err),
    }
}

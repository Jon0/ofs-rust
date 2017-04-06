extern crate fuse;
extern crate libc;

pub mod epoll;
pub mod message;
pub mod socket;

use message::*;
use socket::*;


fn read_message(socket: &mut SockStream) -> AnyMessage {
    let mut input: u32 = 0;
    match socket.read(&mut input) {
        Ok(count) => println!("Read: {}", count),
        Err(err) => println!("Error: {}", err),
    }
    return AnyMessage::init();
}


fn main() {
    let addr = SockAddr4::create(1234);
    let mut serv = Server::init();
    match SockAcceptor::open(&addr) {
        Ok(acceptor) => loop {
            let mut socket = acceptor.accept();
            println!("socket connected");
            let msg = read_message(&mut socket);
            msg.apply(&mut serv);
        },
        Err(err) => println!("Error: {}", err),
    }
}

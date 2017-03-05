extern crate fuse;

use std::net::{TcpListener, TcpStream};


fn handle_client(stream: TcpStream) {

}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:2204").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }
}

use mio::*;
use mio::tcp::{TcpListener, TcpStream};
use epoll::*;
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


fn accept_connections() {
    let mut serv = Server::init();
    let mut handler = EventHandler::create();
    let addr = SockAddr4::create(1234);
    match SockAcceptor::open(&addr) {
        Ok(acceptor) => loop {
            let mut socket = acceptor.accept();
            socket.listen(&mut handler);
            println!("socket connected");
            let msg = read_message(&mut socket);
            msg.apply(&mut serv);
        },
        Err(err) => println!("Error: {}", err),
    }
}


fn run_server() {
    const SERVER: Token = Token(0);
    const CLIENT: Token = Token(1);

    let addr = "127.0.0.1:13265".parse().unwrap();

    // Setup the server socket
    let server = TcpListener::bind(&addr).unwrap();

    // Create a poll instance
    let poll = Poll::new().unwrap();

    // Start listening for incoming connections
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

    // Setup the client socket
    let sock = TcpStream::connect(&addr).unwrap();

    // Register the socket
    poll.register(&sock, CLIENT, Ready::readable(), PollOpt::edge()).unwrap();

    // Create storage for events
    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    // Accept and drop the socket immediately, this will close
                    // the socket and notify the client of the EOF.
                    let _ = server.accept();
                }
                CLIENT => {
                    // The server just shuts down the socket, let's just exit
                    // from our event loop.
                    return;
                }
                _ => unreachable!(),
            }
        }
    }
}

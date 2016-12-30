use std::net::{TcpListener, TcpStream};
use std::io::Error;

/// returns only when listener.incoming() breaks
///
/// probably only when the networking part of the kernel
/// stops working or when hardware stops working
///
/// requires a function that accepts a tcpstream
pub fn start<F: Fn(&TcpStream)>(handle_stream: F) {

    // TODO: error handling
    let listener : TcpListener = TcpListener::bind("127.0.0.1:1337").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(&stream);
            }
            Err(_) => { /* connection failed */ }
        }
    }
    ()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_start() {

        use std::thread;
        use std::net::SocketAddr;
        use std::str::FromStr;
        use std::time::Duration;
        use std::net::TcpStream;
        use std::io::Write;
        let millis = Duration::from_millis(50);


        let testing_socket = SocketAddr::from_str("127.0.0.1:1337").unwrap();

        let start_thread = thread::spawn(move || {
            fn handle_stream(a: &TcpStream) {}
            dbugln!("starting server");
            super::start(handle_stream);
        });

        // wait some time until the server exists (50ms)
        thread::park_timeout(millis);

        let mut stream = TcpStream::connect(testing_socket).unwrap();
        dbugln!("{:?}",stream);

        let _ = stream.write(&b"QUIT\r\n"[..]);
        dbugln!("quit sent");

    dbugln!("server quit");
    }
}

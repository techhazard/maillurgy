use std::net::{TcpListener, TcpStream};
use std::io::Error;

/// returns only when listener.incoming() breaks
///
/// probably only when the networking part of the kernel
/// stops working or when hardware stops working
///
/// requires a function that accepts a tcpstream
pub fn start<F: Fn(TcpStream)>(handle_stream: F) -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:1337")?;

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(stream);
            }
            Err(_) => { /* connection failed */ }
        }
    }
    Ok(())
}

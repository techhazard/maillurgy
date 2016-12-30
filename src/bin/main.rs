#[macro_use]
extern crate maillurgy;

use maillurgy::socket;

use maillurgy::smtp::server::server as smtp_server;


fn main() {

  let p = socket::start(smtp_server);
}


#[cfg(test)]
mod test {
    #[test]
    fn test_main() {

        use std::thread;
        use std::net::SocketAddr;
        use std::str::FromStr;
        use std::time::Duration;
        use std::net::TcpStream;
        use std::io::Write;
        let millis = Duration::from_millis(50);


        let testing_socket = SocketAddr::from_str("127.0.0.1:1337").unwrap();

        let start_thread = thread::spawn(move || {
            super::main();
        });

        // wait some time until the server exists (50ms)
        thread::park_timeout(millis);

        let mut stream = TcpStream::connect(testing_socket).unwrap();
        dbugln!("{:?}",stream);

        let _ = stream.write(&b"QUIT\r\n"[..]);
        dbugln!("quit sent");

    }
}

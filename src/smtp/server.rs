use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};

pub fn server(mut stream: & TcpStream) {
    let closed_message = b"554 serveroffline";
    let opening_message = b"220 WEBADDR maillurgy";
    let unrecognised_command_message = b"500 Command not recognized";
    let _ = stream.write(closed_message);
    let _ = stream.flush();
    println!("welcome message printed");

    let mut buf = [0; 2048];

    loop {

        let cont = match stream.read(&mut buf) {

            Ok(buflen) => handle_buffer(&buf[..buflen]),

            // TODO: maybe return an Err(e) here?
            Err(e) => {println!("buf err: {}", e); false},
        };

        if cont {
            let _ = stream.write_all(unrecognised_command_message);
        } else {
            println!("break");
            break
        }
    }
    println!("shutdown");
    let _ = stream.shutdown(Shutdown::Both);
}

fn handle_buffer(buffer: &[u8]) -> bool {

    println!("{:?}", &buffer);
    let a = super::parse::commands::end_of_transmission(buffer);
    println!("{:?}", a);
    if a.is_done() {
        println!("quitting");
        return false
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{handle_buffer, server};
    use std::net::{TcpStream, TcpListener};
    use std::io::Write;

    #[test]
    fn test_handle_buffer() {
        assert_eq!(true, handle_buffer(b"aoeuoeaao"));
        assert_eq!(false, handle_buffer(b"QUIT\r\n"));
        assert_eq!(true, handle_buffer(b""));
    }

    #[test]
    fn test_server() {
        use std::thread;

        let client = TcpListener::bind("127.0.0.2:20244").unwrap();

        let stream = TcpStream::connect("127.0.0.2:20244").unwrap();
        println!("{:?}",stream);
        let (mut connection, _) = client.accept().unwrap();
        println!("{:?}", connection);

        let client_thread = thread::spawn(move || {
            use std::time::Duration;

            let millis = Duration::from_millis(50);

            thread::park_timeout(millis);
            let _ = connection.write("not quit\r\n".as_bytes());

            // we must send quit otherwise the call to
            // server() below will not terminate
            thread::park_timeout(millis);
            let _ = connection.write("QUIT\r\n".as_bytes());
        });

        server(&stream);

        let res = client_thread.join();
        println!("join result {:?}", res);
    }

    #[test]
    fn test_server_bad_fd() {
        use std::os::unix::io::{FromRawFd};

        // TODO: use some sort of safe variant
        let tcp_stream = unsafe {TcpStream::from_raw_fd(-1)};

        server(&tcp_stream);
    }

}

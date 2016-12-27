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

            Ok(buflen) if buflen > 0 => handle_buffer(&buf[..buflen]),

            Err(e) => {println!("buf err: {}", e); false},
            // buflen == 0
            Ok(_) => continue,
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

    #[test]
    fn test_handle_buffer() {
        assert_eq!(true, handle_buffer(b"aoeuoeaao"));
        assert_eq!(false, handle_buffer(b"QUIT\r\n"));
    }

    #[test]
    fn test_server() {
        use std::os::unix::io::{AsRawFd, FromRawFd};
        use std::net::{TcpStream, TcpListener};
        use std::io::{Read, Write};

        let listener = TcpListener::bind("127.0.0.2:20244").unwrap();

        let stream = TcpStream::connect("127.0.0.2:20244").unwrap();
        println!("{:?}",stream);
        let (mut connection, _) = listener.accept().unwrap();
        println!("{:?}", connection);
        connection.write_all("QUIT\r\n".as_bytes());

        server(&stream);
    }

}

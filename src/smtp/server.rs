use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};

pub fn server(mut stream: TcpStream) {
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

            Err(e) => {println!("buferr: {}", e); false},

            Ok(_) => continue,
        };

        if cont {
            stream.write_all(unrecognised_command_message);
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


#[test]
fn test_handle_buffer() {
    assert_eq!(true, handle_buffer(b"aoeuoeaao"));
    assert_eq!(false, handle_buffer(b"QUIT\r\n"));
}

#[test]
fn test_server() {
    use std::net::TcpListener;
    use std::fs::OpenOptions;
    use std::os::unix::io::FromRawFd;
    use std::os::unix::io::IntoRawFd;

    // TODO: do all this in RAM instead of on disk
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open("/tmp/maillurgy-test.tmp").expect("failed to create test file");
    file.write_all("\0\naoeuaoeu\r\nQUIT\r\n".as_bytes());

    // TODO: use safe variant
    let tcp_stream = unsafe {TcpStream::from_raw_fd(file.into_raw_fd())};

    server(tcp_stream);
}

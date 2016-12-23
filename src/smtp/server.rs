use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};

pub fn server(mut stream: TcpStream) {
    let closed_message = b"554 serveroffline";
    let opening_message = b"220 WEBADDR maillurgy";
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

        if !cont {
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

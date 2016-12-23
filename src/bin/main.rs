extern crate maillurgy;

use maillurgy::socket;

use maillurgy::smtp::server::server as smtp_server;

#[macro_use]
extern crate nom;


fn main() {

  let p = socket::start(smtp_server);
  println!("main end: {:?}", p);
}

// fn main() {
//
    // socket::start(smtp_server);
// }

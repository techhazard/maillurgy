extern crate maillurgy;

use maillurgy::socket;

use maillurgy::smtp::server::server as smtp_server;


fn main() {

  let p = socket::start(smtp_server);
  println!("main end: {:?}", p);
}

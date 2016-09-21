#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter};

pub fn new() -> Nickel {
    let mut server = Nickel::new();
    server.get("/", middleware!("Home"));
    server
}

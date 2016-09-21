extern crate brdgme_server;

fn main() {
    brdgme_server::new().listen("0.0.0.0:8080");
}

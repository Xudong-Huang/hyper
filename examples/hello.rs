#![deny(warnings)]
extern crate may;
extern crate hyper;
extern crate env_logger;

use hyper::server::{Request, Response};

static PHRASE: &'static [u8] = b"Hello World!";

fn hello(_: Request, res: Response) {
    res.send(PHRASE).unwrap();
}

fn main() {
    env_logger::init().unwrap();
    may::config().set_io_workers(2).set_stack_size(0x2000);
    let _listening = hyper::Server::http("127.0.0.1:3000").unwrap()
        .handle(hello);
    println!("Listening on http://127.0.0.1:3000");
}

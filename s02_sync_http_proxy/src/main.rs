use std::{net::TcpListener, thread};
use s02_sync_http_proxy::http_proxy::handle_connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1081").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            if let Err(x) = handle_connection(stream) {
                println!("{:?}", x);
            }
        });
    }
}

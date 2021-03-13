use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    println!("Echo Server Starting...");

    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = format!("Echo: {}", String::from_utf8_lossy(&buffer[..]));

    stream.write(response.as_bytes()).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
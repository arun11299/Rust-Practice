extern crate webserver;
use webserver::ThreadPool;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;

fn handle_connection(mut stream : TcpStream)
{
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Received request: {}", String::from_utf8_lossy(&buffer[..]));

    let mut file = File::open("hello.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK \r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8069").unwrap();
    let mut tp = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established: {:?}", stream);
        tp.execute(move || {
            handle_connection(stream);
        });
    }
}

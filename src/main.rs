use std::io::prelude::*; // this brings in traits like Read that allow us to read from
                         // the stream
use std::{fs, net::{TcpStream, TcpListener}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer);
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")

    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}",status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let home = b"GET / HTTP/1.1\r\n";
    let products = b"GET /products HTTP/1.1\r\n";

    let (status_line, filename) = 
        //@Routes: GET /
        if buffer.starts_with(home) {
            ("HTTP/1.1 200 OK", "./views/index.html")
        //@Routes: GET /products
        } else if buffer.starts_with(products) {
            ("HTTP/1.1 200 OK", "./views/products.html")
        //@Routes: GET *
        } else {
            ("HTTP/1.1 400 NOT FOUND", "./views/404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

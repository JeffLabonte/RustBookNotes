use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream};
use std::fs;

static GET_METHOD:&[u8]  = b"GET / HTTP/1.1\r\n";
static NOT_FOUND:&str = "HTTP/1.1 404 NOT FOUND \r\n\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    if buffer.starts_with(GET_METHOD) {
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap(); // INFO: Close the stream 
    }else{
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}{}", NOT_FOUND, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

}

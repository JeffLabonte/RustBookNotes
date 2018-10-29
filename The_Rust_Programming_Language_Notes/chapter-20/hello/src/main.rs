use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream};
use std::fs;
use std::thread;
use std::time::Duration;

static HTTP_OK_STATUS:&str = "HTTP/1.1 200 OK\r\n\r\n";
static HTTP_NOT_FOUND_STATUS:&str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

static HTTP_GET_METHOD:&[u8]  = b"GET / HTTP/1.1\r\n";
static HTTP_POST_METHOD:&[u8]  = b"POST / HTTP/1.1\r\n";
static HTTP_SLEEP_METHOD:&[u8] = b"GET /sleep HTTP/1.1\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        
        pool.execute(||{
            handle_connection(stream);
        })
        //thread::spawn(||{
        //    handle_connection(stream);
        //});
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let (status_line, filename) = if buffer.starts_with(HTTP_GET_METHOD){
        (HTTP_OK_STATUS, "hello.html")
    } else if buffer.starts_with(HTTP_SLEEP_METHOD){
        thread::sleep(Duration::from_secs(5));
        (HTTP_OK_STATUS, "hello.html")
    }
    else{
        (HTTP_NOT_FOUND_STATUS, "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}",status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // INFO: Close the stream 

}

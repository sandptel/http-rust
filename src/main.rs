#[allow(unused_imports)]
use std::net::TcpListener;
use http::{Request, Response, StatusCode};
use std::io::Write;


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //cl

    let response = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();


    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                stream.write(response);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(read_bytes) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("Read {read_bytes}:\n{}", req_str);
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Deployed Container is reachable from web!</body></html>\r\n";
    match stream.write_all(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

// listen to simple request on a specified port to ensure CD for this project is working as intended
fn main() {
    // listen on 0.0.0.0 because you should be in a docker container
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    println!("Listening for connections on port {}", 8000);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

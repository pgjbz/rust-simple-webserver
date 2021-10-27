use std::io::prelude::*; // into scope to get access to certain traits 
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		handle_connection(stream);
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();
					 //HTTP-Version Status-Code Reason-Phrase CRLF
	stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
	stream.flush().unwrap();
}
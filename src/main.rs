use std::fs;
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
	let content = fs::read_to_string("view/hello.html").unwrap();
					 			//HTTP-Version Status-Code Reason-Phrase CRLF
	let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
							content.len(),
							content
						);
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
use std::fs;

use std::net::{TcpListener, TcpStream};

use rust_simple_webserver::http::{HttpMethod, HttpStatus, parse_http_method, read_buffer, write_content};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		handle_connection(stream);
	}
}

fn handle_connection(mut stream: TcpStream) {

	let buffer = read_buffer(&mut stream);

	if let HttpMethod::GET = parse_http_method(&buffer) {
		let content = fs::read_to_string("view/hello.html").unwrap();
		write_content(stream, &content, HttpStatus::Ok);
	}
}
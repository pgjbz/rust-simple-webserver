use std::net::TcpStream;
use std::io::prelude::*; // into scope to get access to certain traits 

const GET_INIT:  &'static [u8] = b"GET / HTTP/1.1\r\n";

pub enum HttpMethod {
	GET,
	UNKNOWN
}


#[derive(Clone, Copy)]
pub enum HttpStatus {
	Ok = 200,
	NotFound = 404
}

impl HttpStatus {
	pub fn to_string(&self) -> String {
		match self {
			HttpStatus::Ok => String::from("OK"),
			HttpStatus::NotFound => String::from("NOT FOUND")
		}
	}
}


pub fn parse_http_method(buffer: &[u8]) -> HttpMethod {
	if buffer.starts_with(GET_INIT) {
		HttpMethod::GET
	} else {
		HttpMethod::UNKNOWN
	}
}

pub fn write_content(mut stream: TcpStream, content: &str, status: HttpStatus) {
								//HTTP-Version Status-Code Reason-Phrase CRLF
	let response = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}", 
		status as u32, 
		status.to_string(),
		content.len(), 
		content);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}	

pub fn read_buffer(stream: &mut TcpStream) -> [u8; 1024] {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();
	buffer
}
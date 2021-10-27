use core::fmt;
// extern crate strum;
// extern crate strum_macros,EnumString;
use std::net::TcpStream;
use std::io::prelude::*; // into scope to get access to certain traits 

const GET_INIT:  &'static [u8] = b"GET / HTTP/1.1\r\n";

pub enum HttpMethod {
	GET,
	UNKNOWN
}


#[derive(Debug, Clone, Copy)]
pub enum HttpStatus {
	OK = 200,
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
		status.clone() as u32, 
		status,
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
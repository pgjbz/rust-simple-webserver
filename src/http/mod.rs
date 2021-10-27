

use std::collections::HashMap;
use std::fs;
use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::Mutex; // into scope to get access to certain traits 

const GET_INIT:  &'static [u8] = b"GET";

pub type Routes = HashMap::<String, fn(TcpStream)>;

fn root_exec<'r>(stream: TcpStream) {
	let content = fs::read_to_string("view/hello.html").unwrap();
	write_content(stream, &content, HttpStatus::Ok);
}

pub fn default_not_found(stream: TcpStream) {
	let content = fs::read_to_string("view/404.html").unwrap();
	write_content(stream, &content, HttpStatus::NotFound);
}

lazy_static! {
    pub static ref ROUTES: Mutex<Routes> = {
        let mut routes = Routes::new();
		routes.insert(String::from("/"), root_exec);
        Mutex::new(routes)
    };    
}

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

pub fn extract_path(buffer: &[u8]) -> Result<String, ()> {
	let str = String::from_utf8_lossy(buffer);
	let str_slipt = str.split_whitespace();
	if let Some(val) = str_slipt.skip(1).next() {
		Ok(String::from(val))
	} else {
		Err(())
	}
}
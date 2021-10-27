use std::collections::HashMap;
use std::{fs, thread};
use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::Mutex;
use std::time::Duration;

const GET_INIT:  &'static [u8] = b"GET";

pub type Routes = HashMap::<String, fn(&mut Request)>;

pub struct Request<'a> {
	stream: &'a mut TcpStream,
	pub path: String,
	pub method: HttpMethod
}

impl<'a> Request<'a> {
	pub fn new(method: HttpMethod, path: String, stream: &'a mut TcpStream) -> Self {
		Self {
			stream,
			path, 
			method
		}
	}
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

pub fn get_request(stream: &mut TcpStream) -> Box<Request> {
	let buffer = read_buffer(stream); 
	let method = parse_http_method(&buffer);
	let path = if let Ok(path) = extract_path(&buffer) {
		path
	} else {
		"404".to_string()
	};
	Box::new(Request::new(method, path, stream))
}

pub fn default_not_found(request: &mut Request) {
	let content = load_file_to_string("view/404.html");
	write_content(request, &content, HttpStatus::NotFound);
}


fn load_file_to_string(path: &str) -> String {
	if let Ok(content) = fs::read_to_string(path) {
		String::from(content)
	} else {
		"error".to_string()
	}
}

lazy_static! {
    pub static ref ROUTES: Mutex<Routes> = {
        let mut routes = Routes::new();
		routes.insert(String::from("/"), root_exec);
		routes.insert(String::from("/sleep"), sleep);
        Mutex::new(routes)
    };    
}

fn root_exec(request: &mut Request) {
	let content = load_file_to_string("view/hello.html");
	write_content(request, &content, HttpStatus::Ok);
}

fn sleep(request: &mut Request) {
	thread::sleep(Duration::from_secs(5));
	let content = load_file_to_string("view/sleep.html");
	write_content(request, &content, HttpStatus::Ok);
}

pub fn parse_http_method(buffer: &[u8]) -> HttpMethod {
	if buffer.starts_with(GET_INIT) {
		HttpMethod::GET
	} else {
		HttpMethod::UNKNOWN
	}
}

fn write_content(request: &mut Request, content: &str, status: HttpStatus) {
								//HTTP-Version Status-Code Reason-Phrase CRLF
	let response = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}", 
		status as u32, 
		status.to_string(),
		content.len(), 
		content);

	request.stream.write(response.as_bytes()).unwrap();
	request.stream.flush().unwrap();
	
}	

fn read_buffer(stream: &mut TcpStream) -> [u8; 1024] {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();
	buffer
}

fn extract_path(buffer: &[u8]) -> Result<String, ()> {
	let str = String::from_utf8_lossy(buffer);
	let str_slipt = str.split_whitespace();
	if let Some(val) = str_slipt.skip(1).next() {
		Ok(String::from(val))
	} else {
		Err(())
	}
}
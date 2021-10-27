use std::net::{TcpListener, TcpStream};

use rust_simple_webserver::http::{HttpMethod, ROUTES, default_not_found, extract_path, parse_http_method, read_buffer};


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
		let path = if let Ok(path) = extract_path(&buffer) {
			path
		} else {
			"/404".to_string()
		};

		if let Some(ex) = ROUTES.lock().unwrap().get(&path) {
			ex(stream);
		} else {
			default_not_found(stream);
		}
	}
}
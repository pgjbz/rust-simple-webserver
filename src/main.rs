use std::{net::{TcpListener, TcpStream}};

use rust_simple_webserver::http::{HttpMethod, ROUTES, default_not_found, get_request};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	for stream in listener.incoming() {
		let stream = &mut stream.unwrap();
		handle_connection(stream);
	}
}

fn handle_connection(stream: &mut TcpStream) {

	let request = &mut get_request(stream);
	if let HttpMethod::GET = request.method {
		if let Some(ex) = ROUTES.lock().unwrap().get(&request.path) {
			ex(request);
		} else {
			default_not_found(request);
		}
	}
}
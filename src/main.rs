use std::{net::{TcpListener, TcpStream}, sync::{Arc, Mutex}};
use rust_simple_webserver::{http::{self, GETS, HttpMethod}, thread_pool::ThreadPool};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(5);
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		pool.execute(|| {
			handle_connection(stream);
		});
		// handle_connection(stream);
	}
}

fn handle_connection(stream: TcpStream) {

	let stream =  Arc::new(Mutex::new(stream));

	let request = &mut http::get_request(stream);
	if let HttpMethod::GET = request.method {
		if let Some(ex) = GETS.get(&request.path) {
			ex(request);
		} else {
			http::default_not_found(request);
		}
	}
}
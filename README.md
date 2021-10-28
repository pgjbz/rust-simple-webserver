# Simple webserver

Project based on rustbook chapter 20

Source: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

To build this project are used
 - Cargo 1.56.0
 - Rust 1.56.0


Ideas to implement from book:

- Add more documentation to ThreadPool and its public methods.
- Add tests of the library’s functionality.
- Change calls to unwrap to more robust error handling.
- Use ThreadPool to perform some task other than serving web requests.
- Find a thread pool crate on crates.io and implement a similar web server using the crate instead. Then compare its API and robustness to the thread pool we implemented.

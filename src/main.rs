// bring prelude into scope for Read/Write methods
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use hello::ThreadPool;

fn main() {
    // bind works like new, returns a new instance of TcpListener
    // returns a Result<T, E> because binding might require administrator privileges
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // create finite threadpool
    let pool = ThreadPool::new(4);

    // a stream represents an open connection between client and server
    // a connection is the name for the full request response process
    // not iterating over connections, iterating over ATTEMPTS
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

        println!("Shutting down.");
    }
}

fn handle_connection(mut stream: TcpStream) {
    // mutable because it tracks returned data internally, internal state can change
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    // simulate slow request
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

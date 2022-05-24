extern crate maim_thread_synchronous;
use maim_thread_synchronous::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        // incomingはacceptがloopしてる感じ
        let stream = stream.unwrap();
        let pool = ThreadPool::new(4);
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, contents) = if buffer.starts_with(get) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", "get")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(20));
        ("HTTP/1.1 200 OK\r\n\r\n", "sleep")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404")
    };

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

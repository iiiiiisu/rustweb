extern crate chrono;
use rustweb::handlers;
use rustweb::http::Request;
use rustweb::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use chrono::prelude::*;

static ROUTES: [(&'static str, fn(Request) -> String); 3] = [
    ("/", handlers::index),
    ("/sleep", handlers::sleep),
    ("/404", handlers::h404),
];

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8787").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let req = Request::new(String::from_utf8_lossy(&buffer[..]).to_string());
	let now: DateTime<Local> = Local::now();
    println!("[{}] {} {}", now.format("%Y/%m/%d %H:%M:%S"), req.method, req.url);
    for (key, handler) in &ROUTES {
        if *key == req.url {
            let resp = &handler(req);
            stream.write(resp.as_bytes()).unwrap();
            stream.flush().unwrap();
            break;
        }
    }
}

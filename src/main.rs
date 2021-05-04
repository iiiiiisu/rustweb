extern crate chrono;
use chrono::prelude::*;
use rustweb::apps::handlers;
use rustweb::pkg::http::Request;
use rustweb::pkg::server::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

static ROUTES: [(&'static str, fn(Request) -> String); 3] = [
    ("/", handlers::index),
    ("/sleep", handlers::sleep),
    ("/404", handlers::h404),
];

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8787").unwrap();
    match ThreadPool::new(4) {
        Ok(pool) => {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
        }
        Err(e) => println!("Start Server Error: {}", e),
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let now = Local::now().format("%Y/%m/%d %H:%M:%S");
    match Request::new(String::from_utf8_lossy(&buffer[..]).to_string()) {
        Ok(req) => {
            println!("[{}] {} {}", now, req.method, req.url);
            for (key, handler) in &ROUTES {
                if *key == req.url {
                    let resp = &handler(req);
                    stream.write(resp.as_bytes()).unwrap();
                    stream.flush().unwrap();
                    break;
                }
            }
        }
        Err(e) => println!("[{}] {}", now, e),
    }
}

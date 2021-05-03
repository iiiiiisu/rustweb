use super::http;
use super::http::status;
use super::http::Request;
use std::fs;
use std::thread;
use std::time::Duration;

pub fn index(_req: Request) -> String {
    let filename = "res/hello.html";
    let cnt = fs::read_to_string(filename).unwrap();
    http::get_response(status::OK, &cnt)
}

pub fn sleep(_req: Request) -> String {
    thread::sleep(Duration::from_secs(5));
    let filename = "res/hello.html";
    let cnt = fs::read_to_string(filename).unwrap();
    http::get_response(status::OK, &cnt)
}

pub fn h404(_req: Request) -> String {
    let filename = "res/404.html";
    let cnt = fs::read_to_string(filename).unwrap();
    http::get_response(status::NOT_FOUND, &cnt)
}

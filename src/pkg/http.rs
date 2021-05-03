pub mod status {
    pub const OK: &str = "HTTP/1.1 200 OK";
    pub const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";
}

pub fn get_response(status: &str, body: &str) -> String {
    format!("{}\r\n\r\n{}", status, body)
}

pub struct Request {
    pub url: String,
    pub method: String,
    pub version: String,
    pub header: String,
    pub body: String,
}

impl Request {
    pub fn new(cnt: String) -> Request {
        let mut method = String::new();
        let mut url = String::new();
        let mut version = String::new();
        if let Some(pos) = cnt.find("\r\n") {
            let (first_line, cnt) = cnt.split_at(pos);
            let paras: Vec<&str> = first_line.split(" ").collect();
            method = String::from(paras[0]);
            url = String::from(paras[1]);
            version = String::from(paras[2]);
            if let Some(pos) = cnt.find("\r\n\r\n") {
                let (header, body) = cnt.split_at(pos);
                return Request {
                    url,
                    method,
                    version,
                    header: String::from(header.trim()),
                    body: String::from(body.trim()),
                };
            }
        }
        Request {
            url,
            method,
            version,
            header: String::new(),
            body: String::new(),
        }
    }
}

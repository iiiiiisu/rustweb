pub mod status {
    pub const OK: &str = "HTTP/1.1 200 OK";
    pub const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";
}

pub fn get_response(status: &str, body: &str) -> String {
    format!("{}\r\n\r\n{}", status, body)
}

#[derive(Debug)]
pub enum RequestParseError {
    NotValidFormat,
    NoMethodMatched,
    NotValidUrl,
    NotValidVersion,
}

impl std::fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RequestParseError::NotValidFormat => write!(f, "Request format is not valid."),
            RequestParseError::NoMethodMatched => write!(f, "Request method is not valid."),
            RequestParseError::NotValidUrl => write!(f, "Request url is not valid."),
            RequestParseError::NotValidVersion => write!(f, "Request version is not valid."),
        }
    }
}

pub struct Request {
    pub url: String,
    pub method: String,
    pub version: String,
    pub header: String,
    pub body: String,
}

impl Request {
    pub fn new(cnt: String) -> Result<Request, RequestParseError> {
        if let Some(pos) = cnt.find("\r\n") {
            let (first_line, cnt) = cnt.split_at(pos);
            let paras: Vec<&str> = first_line.split(" ").collect();
            let method = String::from(paras[0]);
            let url = String::from(paras[1]);
            let version = String::from(paras[2]);
            if let Some(pos) = cnt.find("\r\n\r\n") {
                let (header, body) = cnt.split_at(pos);
                return Ok(Request {
                    url,
                    method,
                    version,
                    header: String::from(header.trim()),
                    body: String::from(body.trim()),
                });
            }
        }
        Err(RequestParseError::NotValidFormat)
    }
}

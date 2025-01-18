use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
    headers: Vec<Header>,
    body: String,
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Result<Self, Error> {
        let preprocessed_response = raw_response.trim_start().replace("\n\r", "\n");

        let (status_line, remaining) = match preprocessed_response.split_once('\n') {
            Some((s, r)) => (s, r),
            None => {
                return Err(Error::Network(format!(
                    "invalid http response: {}",
                    preprocessed_response
                )))
            }
        };

        let (headers, body) = match remaining.split_once("\n\n") {
            Some((h, b)) => {
                let mut headers = Vec::new();
                for header in h.split('\n') {
                    let splitted_header: Vec<&str> = header.splitn(2, ' ').collect();
                    headers.push(Header::new(
                        String::from(splitted_header[0].trim()),
                        String::from(splitted_header[1].trim()),
                    ));
                }
                (headers, b)
            }
            None => (Vec::new(), remaining),
        };
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

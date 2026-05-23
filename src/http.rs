use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
pub fn fetch(url: &str) -> String {
    let without_scheme = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);

    let (domain, path) = match without_scheme.split_once('/') {
        Some((domain, path)) => (domain, format!("/{}", path)),
        None => (without_scheme, "/".to_string()),
    };

    let request_url = format!("{}:{}", domain, "80");

    let mut stream = TcpStream::connect(request_url).unwrap();

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, domain
    );

    stream.write_all(request.as_bytes()).unwrap();

    let mut law_response = String::new();
    stream.read_to_string(&mut law_response).unwrap();

    stream.shutdown(Shutdown::Both).unwrap();

    let (header, response) = match law_response.split_once("\r\n\r\n") {
        Some((header, response)) => (header, response),
        None => ("HTTP/1.1 200 OK\r\n\r\n", ""),
    };

    let mut html = String::new();

    if header.contains("Transfer-Encoding: chunked") {
        for line in response.lines() {
            if !is_hex(line) {
                html.push_str(line);
            }
        }
    }

    (html.trim().to_string())
}

fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

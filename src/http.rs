use std::net::{Shutdown, TcpStream};
use std::io::{Read, Write, BufReader};
pub fn fetch(url: &str) -> String {

    let without_scheme = url.strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);

    let (domain, path) = match without_scheme.split_once('/') {
        Some((domain, path)) => (domain, format!("/{}", path)),
        None => (without_scheme, "/".to_string()),
    };

    let request_url = format!("{}:{}", domain, "80");

    let mut stream = TcpStream::connect(request_url).unwrap();

    let request = format!("GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", domain);

    stream.write_all(request.as_bytes()).unwrap();

    let mut law_response = String::new();
    stream.read_to_string(&mut law_response).unwrap();

    stream.shutdown(Shutdown::Both).unwrap();

    let mut is_chunked = false;

    let response = law_response.split("\r\n\r\n").collect::<Vec<&str>>();

    let parsed_headers = response[0].split("\r\n").collect::<Vec<&str>>();
    let parsed_response = response[1].split("\r\n").collect::<Vec<&str>>();

    for (i, header) in parsed_headers.iter().enumerate() {
        if (header.contains("Transfer-Encoding")) {
            if (header.contains("chunked")) {
                is_chunked = true
            }
        }
    }

    let mut html = String::new();

    if (is_chunked) {
        for line in &parsed_response {
            if !is_hex(line) {
                html.push_str(line);
            }
        }
    }

    return html.trim().to_string();
}

fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

use std::{net::TcpStream, io::{Write, Read}, collections::HashMap};

use dns_lookup::lookup_host;

use crate::url::parser::{self};

use super::response_parser::{self, HttpResponse};

// TODO: add typings for headers
pub fn get(url: &str, headers: Option<HashMap<&str, &str>>) -> HttpResponse {
    let url = parser::parse(url);

    let ip = lookup_host(&url.host).unwrap().first().unwrap().to_string();
    let ip_with_port = [&ip, ":", "80"].concat();

    let mut stream = match TcpStream::connect(ip_with_port) {
        Ok(stream) => stream,
        Err(_) => panic!("Problem with connection")
    };

    let mut internal_headers = match headers {
        Some(h) => h,
        None => HashMap::new()
    };

    // Default headers
    internal_headers.insert("Host", &url.host);
    internal_headers.insert("Connection", "close");
    internal_headers.insert("User-Agent", "Light browser");

    let mut request = String::new();

    request += &format!("GET {} HTTP/1.1\r\n", url.path);

    for (key, value) in internal_headers.into_iter() {
        request += &format!("{}: {}\r\n", key, value)
    }
    request += "\r\n";

    stream.write(request.as_bytes()).unwrap();

    let mut str_buffer = String::new();
    stream.read_to_string(&mut str_buffer).unwrap();


    return response_parser::parse(&str_buffer);
}
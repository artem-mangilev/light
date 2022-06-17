
use std::{net::TcpStream, io::{Write, Read}};

use dns_lookup::lookup_host;

use crate::url::parser::{self};

pub fn get(url: &str) -> String {
    let url = parser::parse(url);

    println!("{}", url.host);
    println!("{}", url.path);
    println!("{}", url.protocol);

    let ip = lookup_host(&url.host).unwrap().first().unwrap().to_string();
    let ip_with_port = [&ip, ":", "80"].concat();

    let mut stream = match TcpStream::connect(ip_with_port) {
        Ok(stream) => stream,
        Err(_) => panic!("Problem with connection")
    };

    let method = format!("GET {} HTTP/1.0\r\n", url.path);
    let host = format!("Host: {}\r\n\r\n", url.host);
    
    let mut request = String::new();
    request += &method;
    request += &host;

    stream.write(request.as_bytes()).unwrap();

    let mut str_buffer = String::new();
    stream.read_to_string(&mut str_buffer).unwrap();


    return str_buffer;
}
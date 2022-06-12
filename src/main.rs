mod url;

use url::parser;

use dns_lookup::lookup_host;
use std::{net::{TcpStream}, io::{Write, Read}, fmt::format};

fn main() {
    let url = parser::parse("http://example.org/").unwrap();

    let ip = lookup_host(&url.host).unwrap().first().unwrap().to_string();
    let ip_with_port = [&ip, ":", "80"].concat();

    if let Ok(mut stream) = TcpStream::connect(ip_with_port) {
        println!("Connected to the server!");
        
        let method = format!("GET {} HTTP/1.0\r\n", url.path);
        let host = format!("Host: {}\r\n\r\n", url.host);
        let mut request = String::new();
        request += &method;
        request += &host;

        println!("{}", request);

        let sent = stream.write(request.as_bytes());

        println!("{}", sent.unwrap());

        let mut str_buffer = String::new();

        stream.read_to_string(&mut str_buffer);

        println!("{}", str_buffer);
    } else {
        println!("Couldn't connect to server...");
    }
}

mod url;
pub mod http;

use http::{request, response_parser::parse};

fn main() {
    let response = request::get("http://example.org/");

    let parsed_response = parse(&response);

    println!("{}", parsed_response.headers.get("Connection").unwrap());
    println!("{}", parsed_response.headers.get("Last-Modified").unwrap());
    // println!("{}", response);
}

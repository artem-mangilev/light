mod url;
pub mod http;
mod renderer;

use http::{request, response_parser::parse};

fn main() {
    let response = request::get("http://example.org/");

    let parsed_response = parse(&response);
}

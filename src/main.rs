mod url;
pub mod http;
mod renderer;

use http::{request};

fn main() {
    let response = request::get("http://example.org:8080/", None);

    println!("{}", response.body);
}

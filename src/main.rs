mod url;
pub mod http;

use http::request;

fn main() {
    let response = request::get("http://example.org/");

    println!("{}", response)
}

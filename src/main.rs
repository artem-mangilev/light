mod url;

use url::parser;

fn main() {
    let url = parser::parse("http://example.org/").unwrap();

    println!("Host: {0}", url.host);
    println!("Path: {0}", url.path);
}

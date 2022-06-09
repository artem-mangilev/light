mod url;

use url::parser;

use dns_lookup::lookup_host;

fn main() {
    let url = parser::parse("http://example.org/").unwrap();

    let host = lookup_host(&url.host).unwrap();

    for addr in host.iter() {
        println!("ip address: {}", addr);
    }
}

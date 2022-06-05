pub struct Url {
    pub host: String,
    pub path: String
}

pub fn parse(url: String) -> Option<Url> {
    if url.starts_with("http://") {
        let url_without_protocol = url.strip_prefix("http://");

        let split_result: Vec<&str> = url_without_protocol.unwrap().split("/").collect();

        return Some(Url { 
            host: split_result[0].to_string(), 
            // TODO: handle "http://example.org" case
            path: ["/", split_result[1]].join("")
        }); 
    } 

    return None;
}
pub struct Url {
    pub host: String,
    pub path: String,
    pub protocol: String
}

pub fn parse(url: &str) -> Url {
    let (protocol, url) = url.split_at(url.find("://").unwrap());
    let url = url.strip_prefix("://").unwrap();

    let host_and_path: Vec<&str> = url.split("/").collect();

    return Url {
        protocol: protocol.to_string(),
        host: host_and_path[0].to_string(), 
        // TODO: handle "http://example.org" case
        path: ["/", host_and_path[1]].join("")
    }; 
}
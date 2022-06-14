use std::{collections::{HashMap}, io::{BufReader, BufRead}};

pub struct HttpResponse {
    pub headers: HashMap<String, String>,
    pub status: u16,
    pub status_explanation: String,
    pub http_version: String
}

pub fn parse(response: &String) -> HttpResponse {
    let mut headers: HashMap<String, String> = HashMap::new();

    let mut reader = BufReader::new(response.as_bytes());

    // handle statusline
    let mut statusline = String::new();
    reader.read_line(&mut statusline).unwrap();
    let statusline_split: Vec<&str> = statusline.split(" ").collect();

    // handle headers
    loop {
        let mut headerline = String::new();
        reader.read_line(&mut headerline).unwrap();

        if headerline == "\r\n" {
            break;
        }

        let header_split: Vec<&str> = headerline.splitn(2, ":").collect();
        let header = header_split[0].trim().to_string();
        let value = header_split[1].trim().to_string();

        headers.insert(header, value);
    }

    return HttpResponse {
        http_version: statusline_split[0].to_string(),
        status: statusline_split[1].parse().unwrap(),
        status_explanation: statusline_split[2].to_string(),
        headers
    };
}
use clap::builder::Str;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self, Bytes, Write};
use std::path::Path;
use url::Url;

struct ResponseContent {
    content_type: String,
    content_string: String,
    content_bytes: Vec<u8>,
}

pub struct SpiderData {
    origin_url: String,
    domain: String,
}

/// Function that takes a URL and returns the domain
pub fn get_domain(url: &String) -> String {
    let domain = {
        let parsed_url = Url::parse(url);
        if parsed_url.is_err() {
            return String::new();
        }
        let parsed_url = parsed_url.unwrap();
        let host = parsed_url.host_str();
        if host.is_none() {
            return String::new();
        }
        let host = host.unwrap();
        let port = match parsed_url.port() {
            Some(port) => format!(":{}", port),
            None => String::new(),
        };
        format!("{}{}", host, port)
    };
    domain
}

impl SpiderData {
    pub fn new(url: &String) -> SpiderData {
        let domain = get_domain(url);
        SpiderData {
            origin_url: url.clone(),
            domain: domain,
        }
    }

    pub fn print(&self) {
        println!("Origin URL: {}", self.origin_url);
        println!("Domain: {}", self.domain);
    }

    pub fn is_same_domain(&self, url: &String) -> bool {
        let domain = get_domain(url);
        domain == self.domain
    }
}

/// Function that takes a Response object and returns the status code.
pub fn get_status_code(response: &reqwest::blocking::Response) -> u16 {
    response.status().as_u16()
}

/// Function that takes a Response object and returns the content type.
pub fn get_content_type(response: &reqwest::blocking::Response) -> String {
    let headers = response.headers();
    let content_type = headers.get("Content-Type");
    if content_type.is_none() {
        return String::from("");
    }
    content_type.unwrap().to_str().unwrap_or("").to_string()
}

/// Function that is doing a get request against the given URL and returns the response.
pub fn get_request_url(url: &String) -> Result<ResponseContent, ()> {
    let http_client = Client::new();
    let http_result = http_client.get(url).send();
    if http_result.is_err() {
        return Err(());
    }
    let http_result = http_result.unwrap();

    let status_code = get_status_code(&http_result);
    if status_code == 200 {
        let content = {
            let content_type = get_content_type(&http_result);
            let content_bytes = {
                let response = http_result.bytes();
                if response.is_err() {
                    return Err(());
                }
                response.unwrap().to_vec()
            };

            let content_string = String::from_utf8(content_bytes.clone());
            if content_string.is_err() {
                return Err(());
            }
            let content_string = content_string.unwrap();
            ResponseContent {
                content_type: content_type,
                content_string: content_string,
                content_bytes: content_bytes.to_vec(),
            }
        };
        Ok(content)
    } else {
        Err(())
    }
}

fn write_vec_to_file(data: &Vec<u8>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(Path::new(file_path))?;
    file.write_all(data)?;
    file.flush()?;
    Ok(())
}

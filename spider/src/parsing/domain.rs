use url::Url;

/// Function that takes a URL and returns the domain
pub fn get(url: &str) -> String {
    let domain = {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return String::new();
        }
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
            Some(port) => format!(":{port}"),
            None => String::new(),
        };
        format!("{host}{port}")
    };
    domain
}

/// Function that check if a url has a valid domain
pub fn is_valid(url: &str) -> bool {
    let domain = get(url);
    !domain.is_empty()
}

#[cfg(test)]
use super::domain;

#[test]
fn test_get_domain_with_valid_url() {
    let url = "https://www.example.com".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, "www.example.com".to_string());
}

#[test]
fn test_get_domain_with_invalid_url() {
    let url = "not a url".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, String::new());
}

#[test]
fn test_get_domain_with_subdomain() {
    let url = "https://sub.example.com".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, "sub.example.com".to_string());
}

#[test]
fn test_get_domain_with_no_protocol() {
    let url = "www.example.com".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, String::new());
}

#[test]
fn test_get_domain_with_ip_address() {
    let url = "https://192.168.0.1".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, "192.168.0.1".to_string());
}

#[test]
fn test_get_domain_with_port() {
    let url = "https://www.example.com:8080".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, "www.example.com:8080".to_string());
}

#[test]
fn test_get_domain_with_path() {
    let url = "https://www.example.com/path".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, "www.example.com".to_string());
}

#[test]
fn test_get_domain_with_query() {
    let url = "https://www.example.com?query".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, "www.example.com".to_string());
}

#[test]
fn test_get_domain_with_fragment() {
    let url = "https://www.example.com#fragment".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, "www.example.com".to_string());
}

#[test]
fn test_get_domain_with_query_and_fragment() {
    let url = "https://www.example.com?query#fragment".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, "www.example.com".to_string());
}

#[test]
fn test_get_domain_with_random_string() {
    let url = "http:/random".to_string();
    let domain = domain::get(&url);
    assert_eq!(domain, String::new());
}

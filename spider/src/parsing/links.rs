use regex::Regex;
use std::path::Path;

use super::domain;

pub fn extract_all(html: &str) -> Vec<String> {
    let mut links = Vec::new();
    let a_tag_re = Regex::new(r#"<a[^>]*\s+href=["']([^"']*)["']"#).unwrap();
    let link_tag_re = Regex::new(r#"<link[^>]*\s+href=["']([^"']*)["']"#).unwrap();
    let script_tag_re = Regex::new(r#"<script[^>]*\s+src=["']([^"']*)["']"#).unwrap();
    let img_tag_re = Regex::new(r#"<img[^>]*\s+src=["']([^"']*)["']"#).unwrap();
    let source_tag_re = Regex::new(r#"<source[^>]*\s+srcset=["']([^"']*)["']"#).unwrap();
    let background_image_re =
        Regex::new(r#"background-image\s*:\s*url\(['"]?([^'"\)]+)['"]?\)"#).unwrap();
    let css_url_re = Regex::new(r#"url\(['"]?([^'"\)]+)['"]?\)"#).unwrap();
    for caps in a_tag_re.captures_iter(html) {
        if let Some(link) = caps.get(1) {
            links.push(link.as_str().to_string());
        }
    }
    for caps in link_tag_re.captures_iter(html) {
        if let Some(link) = caps.get(1) {
            links.push(link.as_str().to_string());
        }
    }
    for caps in script_tag_re.captures_iter(html) {
        if let Some(link) = caps.get(1) {
            links.push(link.as_str().to_string());
        }
    }
    for caps in img_tag_re.captures_iter(html) {
        if let Some(link) = caps.get(1) {
            links.push(link.as_str().to_string());
        }
    }
    for caps in source_tag_re.captures_iter(html) {
        if let Some(link) = caps.get(1) {
            links.push(link.as_str().to_string());
        }
    }
    for caps in background_image_re.captures_iter(html) {
        if let Some(link) = caps.get(1) {
            links.push(link.as_str().to_string());
        }
    }
    for caps in css_url_re.captures_iter(html) {
        if let Some(link) = caps.get(1) {
            links.push(link.as_str().to_string());
        }
    }
    links.sort();
    links.dedup();
    links
}

pub fn extract_image(html: &str) -> Vec<String> {
    let mut links = extract_all(html);
    links.retain(|link| {
        Path::new(&link)
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("jpg"))
            || Path::new(&link)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("jpeg"))
            || Path::new(&link)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("png"))
            || Path::new(&link)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("gif"))
            || Path::new(&link)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("bmp"))
    });
    links.sort();
    links.dedup();
    links
}

pub fn extract_not_image(html: &str) -> Vec<String> {
    let mut links = extract_all(html);
    links.retain(|link| {
        !Path::new(&link)
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("jpg"))
            && !Path::new(&link)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("jpeg"))
            && !Path::new(&link)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("png"))
            && !Path::new(&link)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("gif"))
            && !Path::new(&link)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("bmp"))
    });
    links = links
        .iter()
        .map(|link| clean_url(link))
        .collect::<Vec<String>>();
    links.sort();
    links.dedup();
    links
}

/// Function that clean a url by removing the fragment and query
/// and remove index.[extention] from the url
pub fn clean_url(url: &str) -> String {
    let url = url.to_string();
    let url = url.split('#').collect::<Vec<&str>>()[0].to_string();
    let url = url.split('?').collect::<Vec<&str>>()[0].to_string();
    let url = if url.ends_with("index.html") {
        url.split('/').collect::<Vec<&str>>()[0..url.split('/').collect::<Vec<&str>>().len() - 1]
            .join("/")
    } else {
        url
    };
    url
}

/// Function that add the domain to the links using the `base_url`
pub fn add_start_url(base_url: &str, links: &Vec<String>) -> Vec<String> {
    let mut links_full = Vec::new();
    let base_url = clean_url(base_url);
    let base_url = if base_url.ends_with('/') {
        base_url
    } else {
        format!("{base_url}/")
    };
    for link in links {
        let mut link = link.to_string();
        if link.starts_with("http://") || link.starts_with("https://") {
            links_full.push(link);
        } else if link.starts_with("//") {
            link = format!("https:{link}");
            links_full.push(link);
        } else if link.starts_with('/') {
            link = link[1..].to_string();
            let base_url = base_url.split('/').collect::<Vec<&str>>()[0..3].join("/");
            link = format!("{base_url}/{link}");
            links_full.push(link);
        } else {
            link = format!("{base_url}{link}");
            links_full.push(link);
        }
    }
    links_full
}

/// Function that remove all the links that are not from the same domain
pub fn filter_links_by_domain(domain: &str, links: &Vec<String>) -> Vec<String> {
    let mut links_from_domain = Vec::new();
    for link in links {
        if domain::is_valid(link) {
            let link_domain = domain::get(link);
            if link_domain == domain {
                links_from_domain.push(link.to_string());
            }
        }
    }
    links_from_domain
}

/// Function which takes a URL and returns the filename.
/// The filename is the url path without the query string and fragment
/// and / replaced with _
pub fn get_filename(url: &str) -> Result<String, ()> {
    let parsed_url = url::Url::parse(url);
    if parsed_url.is_err() {
        return Err(());
    }
    let parsed_url = parsed_url.unwrap();
    let mut path = parsed_url.path();
    if path.starts_with('/') {
        path = &path[1..];
    }
    let path = path.replace('/', "_");
    Ok(path)
}

/// Function that removes all the links with .. in the path
pub fn remove_double_dots(links: &Vec<String>) -> Vec<String> {
    let mut links_filtered = Vec::new();
    for link in links {
        if !link.contains("..") {
            links_filtered.push(link.clone());
        }
    }
    links_filtered
}

/// Function that print the diffence between two Vec of links
/// the difference represents the links to images that are not from the same domain
pub fn print_diff(links: &Vec<String>, links_domain: &[String]) {
    let mut diff = Vec::new();
    for link in links {
        if !links_domain.contains(link) {
            diff.push(link);
        }
    }
    if diff.is_empty() {
        return;
    }
    println!("🖾 Images found from other domain:");
    for link in diff {
        println!("{link}");
    }
}

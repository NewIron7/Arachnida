use super::parsing::domain;
use super::parsing::links;
use super::utils::{download_images, get_request_url};

use std::collections::HashSet;

pub struct ResponseContent {
    pub content_type: String,
    pub text: String,
    pub bytes: Vec<u8>,
}

pub struct SRequest {
    pub url: String,
    pub recursive: bool,
    pub level: u16,
    pub max_level: u16,
    pub domain: String,
    pub path: String,
}

pub struct SResult {
    pub links: Vec<String>,
    pub image_links: Vec<String>,
}

impl SRequest {
    pub fn new(url: &str, recursive: bool, max_level: u16, path: &str) -> SRequest {
        SRequest {
            url: url.to_owned(),
            recursive,
            level: 0,
            max_level,
            domain: domain::get(url),
            path: path.to_owned(),
        }
    }

    pub fn request(&mut self) -> Result<SResult, ()> {
        let mut result = SResult {
            links: vec![],
            image_links: vec![],
        };
        let mut response = Err(());
        for _ in 0..3 {
            response = get_request_url(&self.url);
            if response.is_ok() {
                break;
            }
        }
        if response.is_err() {
            return Err(());
        }
        println!("🟢 Successfully fetched URL: {}", self.url);
        let response = response.unwrap();
        let links = links::extract_not_image(&response.text);
        let links = links::add_start_url(&self.url, &links);
        let links = links::filter_links_by_domain(&self.domain, &links);
        let links = links::remove_double_dots(&links);
        result.links = links;

        let image_links = links::extract_image(&response.text);
        let image_links = links::remove_double_dots(&image_links);
        let image_links = links::add_start_url(&self.url, &image_links);
        let image_links_domain = links::filter_links_by_domain(&self.domain, &image_links);
        links::print_diff(&image_links, &image_links_domain);
        result.image_links = image_links_domain;

        Ok(result)
    }

    pub fn get_all_image_links(
        &mut self,
        visited_urls: &mut HashSet<String>,
        collected_images: &mut Vec<String>,
    ) {
        if visited_urls.contains(&self.url) {
            return;
        }
        visited_urls.insert(self.url.clone());
        let request_result = self.request();
        if let Ok(spider_result) = request_result {
            let mut new_image_links: Vec<String> = Vec::new();
            for image_link in spider_result.image_links {
                if !collected_images.contains(&image_link) {
                    new_image_links.push(image_link.clone());
                    collected_images.push(image_link);
                }
            }
            if !new_image_links.is_empty() {
                download_images(&new_image_links, &self.path);
            }
            if self.recursive && self.level <= self.max_level {
                for link in spider_result.links {
                    let mut new_request = SRequest {
                        url: link.clone(),
                        recursive: self.recursive,
                        level: self.level + 1,
                        max_level: self.max_level,
                        domain: self.domain.clone(),
                        path: self.path.clone(),
                    };
                    new_request.get_all_image_links(visited_urls, collected_images);
                }
            }
        } else {
            // println!("🔴 Failed to fetch or process URL: {}", self.url);
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("URL: {}", self.url);
        println!("Recursive: {}", self.recursive);
        println!("Level: {}", self.level);
        println!("Max Level: {}", self.max_level);
        println!("Domain: {}", self.domain);
    }
}

use std::io::Write;
use reqwest::blocking::Client;
use regex::Regex;
use colored::Colorize;


pub fn get_content_url(url: &String) -> Result<String, ()> {
    let http_client = Client::new();
    let http_result = http_client.get(url).send();

    if http_result.is_ok() {
        let content = http_result
            .unwrap()
            .text()
            .unwrap_or("FAILED".to_string());
        return Ok(content);
    } else {
        println!("Error occured: {:#?}", http_result);
        return Err(());
    }
}

pub fn extract_links(content: &String) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    let re = Regex::new(r#"<a[^>]+href="([^">]+)"#).unwrap();
    for cap in re.captures_iter(content) {
        links.push(cap[1].to_string());
    }
    links
}

pub fn get_images_from_content(content: &String) -> Vec<String> {
    let mut images: Vec<String> = Vec::new();
    let re = Regex::new(r#"<img[^>]+src="([^">]+)"#).unwrap();
    for cap in re.captures_iter(content) {
        images.push(cap[1].to_string());
    }
    images
}

/// Function that filters all the images with those extensions
/// png, jpg, jpeg, gif, bmp
/// Arguments: images: Vec<String> - the vector of images
/// Returns: Vec<String> - the vector of images filtered
pub fn filter_images_extensions(images: Vec<String>) -> Vec<String> {
    let re = Regex::new(r#".+\.(png|jpg|jpeg|gif|bmp)$"#).unwrap();
    let mut filtered_images: Vec<String> = Vec::new();
    for image in images {
        if re.is_match(&image) {
            filtered_images.push(image);
        }
    }
    filtered_images
}

/// Function that removes all images that start with http or https
/// or //, so the ones that are not relative to the website
/// Arguments: images: Vec<String> - the vector of images
/// Returns: Vec<String> - the vector of images filtered
pub fn filter_absolute_images(images: Vec<String>) -> Vec<String> {
    let re = Regex::new(r#"(http|https)://|//"#).unwrap();
    let mut filtered_images: Vec<String> = Vec::new();
    for image in images {
        if !re.is_match(&image) {
            filtered_images.push(image);
        }
    }
    filtered_images
}

/// Function that returns the full link to the image
/// Arguments: images: Vec<String> - the vector of images
/// and url: &String - the url of the website
/// Returns: Vec<String> - the vector of images with the full url
/// added to the relative links
/// Example: /image.png -> http://example.com/image.png
pub fn add_full_url_to_images(images: Vec<String>, url: &String) -> Vec<String> {
    let mut full_images: Vec<String> = Vec::new();
    for image in images {
        // check if we need to add the / to the url
        // or if we have to remove it
        if url.ends_with("/") && image.starts_with("/") {
            full_images.push(format!("{}{}", url, &image[1..]));
        } else if !url.ends_with("/") && !image.starts_with("/") {
            full_images.push(format!("{}/{}", url, image));
        } else {
            full_images.push(format!("{}{}", url, image));
        }
    }
    full_images
}

/// Function which saves the images from the website
/// in the path specified
/// Arguments: images: Vec<String> - the vector of images
/// path: &String - the path to save the images
/// Returns: ()
pub fn save_images(images: &Vec<String>, path: &String) {
    // create the directory if it does not exist
    std::fs::create_dir_all(path).unwrap();
    // remove the last / if it exists
    let path = if path.ends_with("/") {
        &path[..path.len() - 1]
    } else {
        path
    };
    for image in images {
        let http_client = Client::new();
        let http_result = http_client.get(image).send();
        if http_result.is_ok() {
            // get name of the image
            let image_name = image.split("/").last().unwrap();
            let mut file = std::fs::File::create(format!("{}/{}", path, image_name)).unwrap();
            let bytes = http_result.unwrap().bytes().unwrap();
            file.write_all(&bytes).unwrap();

            println!("{} {}", "✅".green(), image_name);
        }
    }
}

/// Function that filters among the links the ones that are valid
/// a valid link is a relative link that is not a link to an image
/// Arguments: links: Vec<String> - the vector of links
pub fn filter_links(links: Vec<String>) -> Vec<String> {
    let re = Regex::new(r#".+\.(png|jpg|jpeg|gif|bmp)$"#).unwrap();
    let mut filtered_links: Vec<String> = Vec::new();
    for link in links {
        if !re.is_match(&link) && !link.starts_with("#") {
            filtered_links.push(link);
        }
    }
    filtered_links
}

/// Function that filters that starts with a specific url
/// Arguments: links: Vec<String> - the vector of links
/// and url: &String - the url of the website
/// Returns: Vec<String> - the vector of links filtered
pub fn filter_links_starting_with(links: Vec<String>, url: &String) -> Vec<String> {
    let mut filtered_links: Vec<String> = Vec::new();
    for link in links {
        if link.starts_with(url) {
            filtered_links.push(link);
        }
    }
    filtered_links
}

/// Function that merges two vectors of links
/// Arguments: links1: Vec<String> - the first vector of links
/// and links2: Vec<String> - the second vector of links
/// Returns: Vec<String> - the vector of links merged
/// Example: [1, 2, 3] + [4, 5, 6] = [1, 2, 3, 4, 5, 6]
pub fn merge_links(links1: Vec<String>, links2: Vec<String>) -> Vec<String> {
    let mut merged_links: Vec<String> = Vec::new();
    for link in links1 {
        merged_links.push(link);
    }
    for link in links2 {
        merged_links.push(link);
    }
    merged_links
}

/// Function that is used to remove duplicates from a vector
/// Arguments: links: Vec<String> - the vector of links
/// Returns: Vec<String> - the vector of links without duplicates
/// Example: [1, 2, 3, 1, 2, 3] -> [1, 2, 3]
pub fn remove_duplicates(links: Vec<String>) -> Vec<String> {
    let mut unique_links: Vec<String> = Vec::new();
    for link in links {
        if !unique_links.contains(&link) {
            unique_links.push(link);
        }
    }
    unique_links
}

/// Function that remove links that are already visited
/// Arguments: links: Vec<String> - the vector of links
/// and visited: Vec<String> - the vector of visited links
/// Returns: Vec<String> - the vector of links without the visited ones
/// Example: [1, 2, 3, 4, 5] - [2, 4] = [1, 3, 5]
pub fn remove_visited_links(links: Vec<String>, visited: &Vec<String>) -> Vec<String> {
    let mut unvisited_links: Vec<String> = Vec::new();
    for link in links {
        if !visited.contains(&link) {
            unvisited_links.push(link);
        }
    }
    unvisited_links
}
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::parsing::links::get_filename;
use super::spider::ResponseContent;

/// Function that takes a Response object and returns the status code.
pub fn get_status_code(response: &reqwest::blocking::Response) -> u16 {
    response.status().as_u16()
}

/// Function that takes a Response object and returns the content type.
pub fn get_content_type(response: &reqwest::blocking::Response) -> String {
    let headers = response.headers();
    let content_type = headers.get("Content-Type");
    if content_type.is_none() {
        return String::new();
    }
    content_type.unwrap().to_str().unwrap_or("").to_string()
}

/// Function that is doing a get request against the given URL and returns the response.
pub fn get_request_url(url: &String) -> Result<ResponseContent, ()> {
    let http_client = Client::new();
    let http_result = http_client.get(url).send();
    if http_result.is_err() {
        //println!("Error sending request");
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
                    //println!("Error getting response bytes");
                    return Err(());
                }
                response.unwrap().to_vec()
            };
            let content_string = {
                let tmp = String::from_utf8(content_bytes.clone());
                if let Ok(tmp) = tmp {
                    tmp
                } else {
                    String::new()
                }
            };
            ResponseContent {
                content_type,
                text: content_string,
                bytes: content_bytes,
            }
        };
        Ok(content)
    } else {
        //println!("Error getting response: {}", status_code);
        Err(())
    }
}

pub fn write_vec_to_file(data: &[u8], file_path: &str) -> io::Result<()> {
    let mut file = File::create(Path::new(file_path))?;
    file.write_all(data)?;
    file.flush()?;
    Ok(())
}

/// Function that check if the content type is an image jpg jpeg png gif or bmp.
pub fn is_image(content_type: &str) -> bool {
    let content_type = content_type.to_lowercase();
    content_type.contains("image/jpeg")
        || content_type.contains("image/jpg")
        || content_type.contains("image/png")
        || content_type.contains("image/gif")
        || content_type.contains("image/bmp")
}

/// Function that takes the url of an image and downloads it to the given path.
pub fn download_image(url: &String, path: &String) -> Result<(), ()> {
    let response = get_request_url(url);
    if response.is_err() {
        println!("Could not download image {url}");
        return Err(());
    }
    let response = response.unwrap();
    if !is_image(&response.content_type) {
        println!("Not an image: {} -> {}", url, response.content_type);
        return Err(());
    }
    let filename = get_filename(url)?;
    let file_path = format!("{path}/{filename}");
    let result = write_vec_to_file(&response.bytes, &file_path);
    if result.is_err() {
        println!("Could not write image to file: {file_path}");
        return Err(());
    }
    println!("🖼️ Downloaded image: {url} -> {file_path}");
    Ok(())
}

/// Functiont that takes a Vec of links and downloads all the images to the given path.
pub fn download_images(links: &Vec<String>, path: &String) {
    for link in links {
        let result = download_image(link, path);
        if result.is_err() {
            continue;
        }
    }
}

/// Function that check if we can create a folder at the given path.
pub fn can_create_folder(path: &String) -> bool {
    let path = Path::new(path);
    if path.exists() {
        if path.is_dir() {
            let test_file = path.join("test");
            let result = File::create(&test_file);
            if result.is_err() {
                println!("Could not write to directory: {}", path.display());
                return false;
            }
            let result = std::fs::remove_file(&test_file);
            if result.is_err() {
                println!("Could not remove test file from directory: {}", path.display());
                return false;
            }
            return true;
        }
        println!("Path is not a directory: {}", path.display());
        return false;
    }
    let result = std::fs::create_dir_all(path);
    if result.is_err() {
        println!("Could not create directory: {}", path.display());
        return false;
    }
    true
}

/// Function that is making a get request to the given URL to check if we can access it.
pub fn check_url(url: &String) -> bool {
    let response = get_request_url(url);
    if response.is_err() {
        return false;
    }
    true
}

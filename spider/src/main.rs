
use clap::{command, value_parser, Arg};

mod utils;
use utils::*;

fn main() {
    let matches = command!()
        .version("1.0")
        .author("Your Name")
        .about("Downloads images from a website")
        .arg(
            Arg::new("URL")
                .required(true)
                .index(1)
                .help("The URL to download images from"),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .default_value("false")
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value("true")
                .value_parser(value_parser!(bool))
                .help("Recursively download images"),
        )
        .arg(
            Arg::new("level")
                .short('l')
                .long("level")
                .default_value("5")
                .value_parser(value_parser!(u16))
                .help("Maximum depth level for recursive download"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .default_value("./data/")
                .help("Path to save downloaded files"),
        )
        .get_matches();

    let url: &String = matches.get_one::<String>("URL").unwrap();
    let recu: &bool = matches.get_one::<bool>("recursive").unwrap();
    let path: &String = matches.get_one::<String>("path").unwrap();
    let mut level: &u16 = matches.get_one::<u16>("level").unwrap();

    if !recu {
        level = &0;
    }

    // println!("{url}");
    // println!("{recu}");
    // println!("{level}");
    // println!("{path}");

    download_images(url, path, &mut level.clone());
}

/// main function that is used to download images from a website
/// Arguments: url: &String - the url of the website
/// path: &String - the path to save the images
/// recu: &bool - if the download is recursive
/// level: &u16 - the level of the recursive download
/// Returns: ()
fn download_images(url: &String, path: &String, level: &mut u16) {
    let mut links_visited: Vec<String> = Vec::new();
    let mut links_to_visit: Vec<String> = Vec::new();
    let mut images_downloaded: Vec<String> = Vec::new();
    links_to_visit.push(url.clone());

    while links_to_visit.len() != 0 {
        let current_url = links_to_visit.pop().unwrap();
        let content = get_content_url(&current_url);
        if content.is_err() {
            println!("❌ Error occured: {:#?}", current_url);
            links_to_visit.push(current_url.clone());
            continue;
        }
        else {
            println!("🔗 Visited: {}", current_url);
            links_visited.push(current_url.clone());
            let content = content.unwrap();
            let images = get_images_links(&content, &url);
            let clean_images = remove_visited_links(images, &images_downloaded);
            save_images(&clean_images, &path);
            images_downloaded = merge_links(images_downloaded, clean_images);
            images_downloaded = remove_duplicates(images_downloaded);

            if *level > 0 {
                let links = get_links(&content, &url);
                let clean_links = remove_duplicates(links);
                let new_links = remove_visited_links(clean_links, &links_visited);
                links_to_visit = merge_links(links_to_visit, new_links);
                *level -= 1;
            }
        }
    }
}

/// function that is used to get links to images from a website html content
/// with only the valid links to the valid images
/// add the full url to the relative links
fn get_images_links(content: &String, url: &String) -> Vec<String> {
    let images = get_images_from_content(content);
    let filtered_by_extension = filter_images_extensions(images);
    let filtered_by_absolute = filter_absolute_images(filtered_by_extension);
    let full_images = add_full_url_to_images(filtered_by_absolute, &url);
    full_images
}

/// function that is used to get valid links from a website html content
/// with the full url
fn get_links(content: &String, url: &String) -> Vec<String> {
    let links = extract_links(content);
    let filter_links = filter_links(links);
    let starting_with_url = filter_links_starting_with(filter_links.clone(), url);
    let relative_links = filter_absolute_images(filter_links);
    let full_links = add_full_url_to_images(relative_links, &url);
    // merge the relative links with the full links
    let all_links = merge_links(starting_with_url , full_links);
    all_links
}

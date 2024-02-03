use std::{fs::File, io::Write};

use clap::{command, value_parser, Arg};
use reqwest::blocking::Client;

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
    let level: &u16 = matches.get_one::<u16>("level").unwrap();

    println!("{url}");
    println!("{recu}");
    println!("{level}");
    println!("{path}");
    let content = get_content_url(url);

    if content.is_ok() {
        let content = content.unwrap();
        let filename = "index.html".to_string();
        let _ = put_content_in_file(&content, &path, &filename);
    }
}

///Function that is used to put content in a file
fn put_content_in_file(content: &String, path: &String, filename: &String) -> Result<(), ()> {
    //create the directory if it does not exist
    std::fs::create_dir_all(path).unwrap();
    let mut file = File::create(format!("{path}{filename}")).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    Ok(())
}

fn get_content_url(url: &String) -> Result<String, ()> {
    let http_client = Client::new();
    let http_result = http_client.get(url).send();

    if http_result.is_ok() {
        let content = http_result
            .unwrap()
            .text()
            .unwrap_or("FAILED".to_string());
        println!("{:#?}", content);
        return Ok(content);
    } else {
        //println!("Error occured: {:#?}", http_result);
        return Err(());
    }
}

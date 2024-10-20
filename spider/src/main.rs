use clap::{command, value_parser, Arg};

mod utils;
use utils::*;

fn main() {
    let matches = command!()
        .version("1.0")
        .author("hboissel")
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

    let mut spider = SpiderData::new(url);
    spider.print();
}

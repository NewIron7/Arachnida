use exif;

/// Get all arguments from the command line
/// They are stored in a vector of strings
/// They represent filnames
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // remove the first argument which is the name of the program
    let args: Vec<String> = args[1..].to_vec();
    
    print_file_content(args);
}

/// Function that loop through the filenames
/// and print the EXIF data of the pictures
/// It takes a vector of strings as argument
/// It returns nothing
/// It uses the get_picture_content and get_exif_data functions
/// to get the content of the files and the EXIF data
/// and print the EXIF data
fn print_file_content(args: Vec<String>) {
    for filename in args {
        let content = get_picture_content(&filename);
        let exif_data = get_exif_data(content);
        // print the filename with an emoji
        println!("📷 {}", filename);
        println!("{}", exif_data);
    }
}

/// Function that get EXIF data in a vector of bytes
/// It takes a vector of bytes as argument
/// it returns the EXIF data as a string
/// if a error occurs, it returns an empty string
/// and print the error with a emoji
/// It uses the kamadak-exif crate
/// to parse the EXIF data
fn get_exif_data(content: Vec<u8>) -> String {
    match exif::parse_exif(&content) {
        Ok(exif) => exif.to_string(),
        Err(e) => {
            println!("😭 {}", e);
            String::new()
        }
    }
}

/// Function that read the content of a file
/// those files are pictures
/// It takes a filename as argument
/// It returns a vector of bytes
/// if a error occurs, it returns an empty vector
/// and print the error with a emoji
fn get_picture_content(filename: &str) -> Vec<u8> {
    match std::fs::read(filename) {
        Ok(content) => content,
        Err(e) => {
            println!("{}:😭 {}", filename, e);
            Vec::new()
        }
    }
}


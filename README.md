# ARACHNIDA

Two school projects: Spider and Scorpion. Both are simple, yet practical command-line tools created using the Rust programming language.

**Spider** is a tool designed to download images from a website.

**Scorpion** is a utility for viewing the metadata and EXIF data of files.

# 🕷️ Spider

## 🌐 Overview
Spider is a command-line tool written in Rust, designed to download images from a specified website. It allows for both simple and recursive downloads, offering various options to customize the depth of recursion, the path for saving files, and more.

## 🌟 Features
- **Simple Download**: Download all images from a given URL.
- **Recursive Download**: Option to download images recursively from linked pages.
- **Depth Control**: Control the depth of recursion with a specified level.
- **Custom Save Path**: Choose a custom path to save downloaded images.

## 🛠️ Requirements
- Rust Programming Language

## ⚙️ Installation
Clone the repository and build the project using Cargo:
```
git clone <repository-url>
cd spider
cargo build --release
```

## 🚀 Usage
Run the program with the required and optional arguments:
```
./target/release/spider [OPTIONS] <URL>
```

### Arguments
- `URL`: The URL to download images from (required).

### Options
- `-r, --recursive [true/false]`: Enable recursive downloading of images (default is `false`).
- `-l, --level <level>`: Maximum depth level for recursive download (default is `5`).
- `-p, --path <path>`: Path to save downloaded files (default is `./data/`).

## 📚 Examples
- Download images from a URL:
  ```
  ./target/release/spider http://example.com
  ```
- Download images recursively with a depth of 3:
  ```
  ./target/release/spider -r true -l 3 http://example.com
  ```
- Specify a custom path for saving images:
  ```
  ./target/release/spider -p /path/to/save http://example.com
  ```

### List of test target

- https://books.toscrape.com/

# 🦂 Scorpion

## 🌐 Overview
Scorpion is a command-line tool written in Rust, designed for viewing metadata and EXIF data of files. It's a straightforward tool for quick insights into file properties.

## 🌟 Features
- **Metadata Viewing**: Display basic file metadata like size, creation, modification, and access times.
- **EXIF Data Extraction**: Extract and display EXIF data from images.

## 🛠️ Requirements
- Rust Programming Language
- `exif` and `chrono` Rust crates for handling EXIF data and time formats.

## ⚙️ Installation
Clone the repository and build the project using Cargo:
```
git clone <repository-url>
cd scorpion
cargo build --release
```

## 🚀 Usage
Run the program with the file names as arguments:
```
./target/release/scorpion <file1> <file2> ...
```

## 📚 Examples
- View metadata and EXIF data of a single file:
  ```
  ./target/release/scorpion photo.jpg
  ```
- View metadata and EXIF data of multiple files:
  ```
  ./target/release/scorpion image1.jpg image2.png
  ```

## 🙏 Acknowledgements
This project is part of my study at 42 School


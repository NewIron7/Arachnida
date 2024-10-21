#[cfg(test)]
use super::links;

#[test]
fn test_extract_image_links_with_no_images() {
    let html_content = r#"
            <p>This is a text with no images</p>
        "#
    .to_string();

    let image_links = links::extract_image(&html_content);
    assert_eq!(image_links.len(), 0);
}

#[test]
fn test_extract_image_links_with_one_image() {
    let html_content = r#"
            <img src="images/picture.jpg" alt="A picture">
        "#
    .to_string();

    let image_links = links::extract_image(&html_content);
    assert_eq!(image_links.len(), 1);
    assert_eq!(image_links[0], "images/picture.jpg".to_string());
}

#[test]
fn test_extract_image_absolute_links() {
    let html_content = r#"
            <img src="https://www.example.com/images/picture.jpg" alt="A picture">
        "#
    .to_string();

    let image_links = links::extract_image(&html_content);
    assert_eq!(image_links.len(), 1);
    assert_eq!(
        image_links[0],
        "https://www.example.com/images/picture.jpg".to_string()
    );
}

#[test]
fn test_extract_image_links() {
    let html_content = r#"
            <img src="images/picture.jpg" alt="A picture">
            <picture>
                <source srcset="images/image.jpeg" type="image/jpeg">
                <source srcset="images/picture.png" type="image/png">
                <img src="images/fallback.jpg" alt="Fallback Image">
            </picture>
            <div style="background-image: url('images/background.bmp');"></div>
            <style>
                .background { background-image: url('images/inline-background.gif'); }
            </style>
        "#
    .to_string();

    let image_links = links::extract_image(&html_content);
    assert_eq!(image_links.len(), 6);
    assert!(image_links.contains(&"images/picture.jpg".to_string()));
    assert!(image_links.contains(&"images/image.jpeg".to_string()));
    assert!(image_links.contains(&"images/picture.png".to_string()));
    assert!(image_links.contains(&"images/fallback.jpg".to_string()));
    assert!(image_links.contains(&"images/background.bmp".to_string()));
    assert!(image_links.contains(&"images/inline-background.gif".to_string()));
}

#[test]
fn test_extract_image_links_with_other_links() {
    let html_content = r#"
            <img src="images/picture.jpg" alt="A picture">
            <a href="https://example.com">Link</a>
            <link rel="stylesheet" href="styles.css">
            <script src="app.js"></script>
            <img src="images/image.jpg" alt="Image">
            <picture>
                <source srcset="images/image.webp" type="image/webp">
                <img src="images/fallback.jpg" alt="Fallback Image">
            </picture>
            <div style="background-image: url('images/background.jpg');"></div>
            <style>
                .background { background-image: url('images/background2.png'); }
            </style>
        "#
    .to_string();

    let image_links = links::extract_image(&html_content);
    assert_eq!(image_links.len(), 5);
    assert!(image_links.contains(&"images/picture.jpg".to_string()));
    assert!(image_links.contains(&"images/image.jpg".to_string()));
    assert!(image_links.contains(&"images/fallback.jpg".to_string()));
    assert!(image_links.contains(&"images/background.jpg".to_string()));
    assert!(image_links.contains(&"images/background2.png".to_string()));
}

#[test]
/// Test that the function only takes jpg, jpeg, png, gif and bmp images.
fn test_extract_image_links_wrong_extension() {
    let html_content = r#"
            <img src="images/picture.jpg" alt="A picture">
            <img src="images/picture.jpeg" alt="A picture">
            <img src="images/picture.png" alt="A picture">
            <img src="images/picture.gif" alt="A picture">
            <img src="images/picture.bmp" alt="A picture">
            <img src="images/picture.tiff" alt="A picture">
            <img src="images/picture.svg" alt="A picture">
            <img src="images/picture.webp" alt="A picture">
        "#
    .to_string();
    let links = links::extract_image(&html_content);
    assert_eq!(links.len(), 5);
    assert!(links.contains(&"images/picture.jpg".to_string()));
    assert!(links.contains(&"images/picture.jpeg".to_string()));
    assert!(links.contains(&"images/picture.png".to_string()));
    assert!(links.contains(&"images/picture.gif".to_string()));
    assert!(links.contains(&"images/picture.bmp".to_string()));
    assert!(!links.contains(&"images/picture.tiff".to_string()));
    assert!(!links.contains(&"images/picture.svg".to_string()));
    assert!(!links.contains(&"images/picture.webp".to_string()));
}

#[test]
fn test_extract_all_links() {
    let html_content = r#"
        <a href="https://example.com">Link</a>
        <link rel="stylesheet" href="styles.css">
        <script src="app.js"></script>
        <img src="image.jpg" alt="Image">
        <picture>
            <source srcset="image.webp" type="image/webp">
            <img src="fallback.jpg" alt="Fallback Image">
        </picture>
        <div style="background-image: url('background.jpg');"></div>
        <style>
            .background { background-image: url('background2.png'); }
        </style>
    "#
    .to_string();
    let links = links::extract_all(&html_content);
    assert_eq!(links.len(), 8);
    assert!(links.contains(&"https://example.com".to_string()));
    assert!(links.contains(&"styles.css".to_string()));
    assert!(links.contains(&"app.js".to_string()));
    assert!(links.contains(&"image.jpg".to_string()));
    assert!(links.contains(&"image.webp".to_string()));
    assert!(links.contains(&"fallback.jpg".to_string()));
    assert!(links.contains(&"background.jpg".to_string()));
    assert!(links.contains(&"background2.png".to_string()));
}

#[test]
fn test_clean_url() {
    let url = "https://example.com/page?query=1#fragment";
    let cleaned_url = links::clean_url(url);
    assert_eq!(cleaned_url, "https://example.com/page");
}

#[test]
fn test_clean_url_with_index() {
    let url = "https://example.com/index.html";
    let cleaned_url = links::clean_url(url);
    assert_eq!(cleaned_url, "https://example.com");
}

#[test]
fn test_clean_url_with_index_and_query_fragment() {
    let url = "https://example.com/index.html?query=1#fragment";
    let cleaned_url = links::clean_url(url);
    assert_eq!(cleaned_url, "https://example.com");
}

#[test]
fn test_clean_url_with_index_and_path() {
    let url = "https://example.com/path/to/index.html";
    let cleaned_url = links::clean_url(url);
    assert_eq!(cleaned_url, "https://example.com/path/to");
}

#[test]
fn test_filter_links_by_domain() {
    let domain = "example.com";
    let links = vec![
        "https://example.com/page1".to_string(),
        "https://www.example.com/page2".to_string(),
        "http://example.com/page3".to_string(),
    ];
    let filtered_links = links::filter_links_by_domain(domain, &links);
    assert_eq!(filtered_links.len(), 2);
    assert!(filtered_links.contains(&"https://example.com/page1".to_string()));
    assert!(filtered_links.contains(&"http://example.com/page3".to_string()));
}

#[test]
fn test_add_start_url_to_links() {
    let start_url = "https://example.com";
    let links = vec![
        "/page1".to_string(),
        "page2".to_string(),
        "https://example.com/page2".to_string(),
        "http://example.com/page3".to_string(),
        "//example.com/page4".to_string(),
    ];
    let links = links::add_start_url(start_url, &links);
    assert_eq!(links.len(), 5);
    assert!(links.contains(&"https://example.com/page1".to_string()));
    assert!(links.contains(&"https://example.com/page2".to_string()));
    assert!(links.contains(&"http://example.com/page3".to_string()));
    assert!(links.contains(&"https://example.com/page4".to_string()));
}

#[test]
fn test_add_start_url_to_links_with_path_in_url() {
    let start_url = "https://example.com/path/to/page";
    let links = vec![
        "/page1".to_string(),
        "page2".to_string(),
        "https://example.com/page3".to_string(),
        "http://example.com/page4".to_string(),
        "//example.com/page5".to_string(),
    ];
    let links = links::add_start_url(start_url, &links);
    assert_eq!(links.len(), 5);
    assert!(links.contains(&"https://example.com/page1".to_string()));
    assert!(links.contains(&"https://example.com/path/to/page/page2".to_string()));
    assert!(links.contains(&"https://example.com/page3".to_string()));
    assert!(links.contains(&"http://example.com/page4".to_string()));
    assert!(links.contains(&"https://example.com/page5".to_string()));
}

#[test]
fn test_get_filename() {
    let url = "https://example.com/image.jpg".to_string();
    let filename = links::get_filename(&url);
    if filename.is_err() {
        assert!(false);
    }
    let filename = filename.unwrap();
    assert_eq!(filename, "image.jpg".to_string());
}

#[test]
fn test_get_filename_with_query() {
    let url = "https://example.com/image.jpg?query=1".to_string();
    let filename = links::get_filename(&url);
    if filename.is_err() {
        assert!(false);
    }
    let filename = filename.unwrap();
    assert_eq!(filename, "image.jpg".to_string());
}

#[test]
fn test_get_filename_long_path() {
    let url = "https://example.com/path/to/image.jpg".to_string();
    let filename = links::get_filename(&url);
    if filename.is_err() {
        assert!(false);
    }
    let filename = filename.unwrap();
    assert_eq!(filename, "path_to_image.jpg".to_string());
}

use reqwest::StatusCode;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() {
    // Example for HTTP URL
    let url = "http://localhost/static/projects/s4/video/2024-11-12/C1_2024_11_12T15_19_53.mkv";
    match get_http_file_info(url) {
        Ok((length, content_type)) => {
            println!("HTTP File Length: {}", length);
            println!("HTTP Content Type: {}", content_type);
        }
        Err(e) => println!("An error occurred: {}", e),
    }

    // Example for file path
    let file_path =
        Path::new("/media/jiang/f253bd88-c622-438d-8bbe-c0b2471ff26e/KEN英语联系方式.png");
    match get_file_info(file_path) {
        Ok((length, content_type)) => {
            println!("File Length: {}", length);
            println!("Content Type: {}", content_type);
        }
        Err(e) => println!("An error occurred: {}", e),
    }
}

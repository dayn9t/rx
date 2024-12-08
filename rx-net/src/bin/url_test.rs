use reqwest::StatusCode;
use reqwest::blocking::Client;
use std::error::Error;

pub fn url_file_exists(url: &str) -> Result<bool, Box<dyn Error>> {
    let client = Client::new();
    let response = client.head(url).send()?;

    println!("response: {:?}", response);

    Ok(response.status() == StatusCode::OK)
}

fn main() {
    let url = "http://localhost/static/projects/s4/video/2024-11-12/C1_2024_11_12T15_19_53.mkv";
    match url_file_exists(url) {
        Ok(exists) => {
            if exists {
                println!("The file exists.");
            } else {
                println!("The file does not exist.");
            }
        }
        Err(e) => println!("An error occurred: {}", e),
    }
}

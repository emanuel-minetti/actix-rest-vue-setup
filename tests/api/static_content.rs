use crate::helpers::spawn_app;
use rand::Rng;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

#[tokio::test]
async fn favicon_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let file = File::open("src/vue-client/public/favicon.ico")
        .expect("Should be able to open 'favicon.ico'");
    let mut file_reader = BufReader::new(file);
    let mut file_buffer = Vec::new();
    file_reader
        .read_to_end(&mut file_buffer)
        .expect("Should be able to read 'favicon.ico'");
    // Act
    let response = client
        .get(&format!("{}/favicon.ico", address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    let response_bytes = response.bytes().await;
    let response_buffer = response_bytes.expect("Should be able to open response answer");
    assert_eq!(file_buffer, response_buffer);
}

#[tokio::test]
async fn url_root_routes_to_index() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let re = get_index_matching_reg_ex();
    // Act
    let urls = vec!["", "/"];
    for url in urls {
        let response = client
            //.get("http://127.0.0.1:8080".to_owned() + url)
            .get(address.to_owned() + url)
            .send()
            .await
            .expect("Failed to execute request.");
        //Assert
        assert!(response.status().is_success());
        assert_eq!(
            re.captures_iter(
                response
                    .text()
                    .await
                    .expect("Failed to get request body")
                    .as_str()
            )
            .collect::<Vec<_>>()
            .len(),
            1
        );
    }
}

#[tokio::test]
async fn random_url_routes_to_index() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let re = get_index_matching_reg_ex();
    let urls = get_random_urls(10, 15);

    // Act
    for url in urls {
        let response = client
            .get(address.to_owned() + "/" + url.as_str())
            .send()
            .await
            .expect("Failed to execute request.");
        //Assert
        assert!(response.status().is_success());
        let response =
            response
                .text()
                .await
                .expect("Failed to get request body")
                .as_str().to_owned();
        dbg!(&response);
        let matches = re.captures_iter(&response).collect::<Vec<_>>();
        assert_eq!(matches.len(),
            1
        );
    }
}

pub fn get_index_matching_reg_ex() -> Regex {
    Regex::new(
        r#"(?m)^<!DOCTYPE html>\r?$
^<html lang="en">\r?$
^ {2}<head>\r?$
^ {4}<meta charset="UTF-8">\r?$
^ {4}<link rel="icon" href="/favicon\.ico">\r?$
^ {4}<meta name="viewport" content="width=device-width, initial-scale=1\.0">\r?$
^ {4}<title>Vite App</title>\r?$
^ {4}<script type="module" crossorigin src="/assets/index-[0-9a-f]+\.js"></script>\r?$
"#,
    )
    .expect("Could not parse RegEx.")
}

fn get_random_urls(size: usize, url_length: u8) -> Vec<String> {
    let mut res = Vec::new();
    while res.len() < size {
        let candidate = get_random_string(url_length);
        if !candidate.starts_with("/api")
            && !candidate.starts_with("/login")
            && !candidate.starts_with('~')
            && !candidate.chars().next().unwrap().is_ascii_digit()
        {
            res.push(candidate);
        }
    }
    res
}

fn get_random_string(length: u8) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789~";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let index = rng.gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect()
}

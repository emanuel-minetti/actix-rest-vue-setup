use crate::helpers::{get_index_matching_re, spawn_app};
use std::fs::File;
use std::io::{BufReader, Read};

#[tokio::test]
async fn favicon_works() {
    // Arrange
    spawn_app().await;
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
        .get("http://127.0.0.1:8080/favicon.ico")
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
    spawn_app().await;
    let client = reqwest::Client::new();
    let re = get_index_matching_re();
    // Act
    let urls = vec!["", "/"];
    for url in urls {
        let response = client
            .get("http://127.0.0.1:8080".to_owned() + url)
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

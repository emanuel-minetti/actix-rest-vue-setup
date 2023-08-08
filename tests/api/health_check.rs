use crate::helpers::spawn_app;
use std::fs::File;
use std::io::{BufReader, Read};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8080/api/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

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

use std::fs::File;
use std::io::{BufReader, Read};
use crate::helpers::spawn_app;

// #[tokio::test]
// async fn favicon_works() {
//     // Arrange
//     spawn_app().await;
//     let client = reqwest::Client::new();
//     let file = File::open("test.ico").expect("Should be able to open 'favicon.ico'");
//     let mut file_reader = BufReader::new(file);
//     let mut file_buffer = Vec::new();
//     file_reader.read_to_end(&mut file_buffer).expect("Should be able to read 'favicon.ico'");
//     // Act
//     let response = client
//         .get("http://127.0.0.1:8080/favicon.ico")
//         .send()
//         .await
//         .expect("Failed to execute request.");
//     // Assert
//     assert!(response.status().is_success());
//     let response_buffer = response.bytes().await.expect("Should be able to open response answer");
//     //assert_eq!(file_buffer.len(), response.content_length());
//     assert_eq!(file_buffer, response_buffer);
//
// }
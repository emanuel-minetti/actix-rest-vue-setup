use crate::helpers::spawn_app;
use std::fs::OpenOptions;

#[tokio::test]
async fn writing_to_logfile_works() {
    // Arrange
    let test_app = spawn_app();
    let path = test_app.settings.log_settings().path() + ".log";
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open logfile for writing");
    file.set_len(0).expect("Clearing logfile failed");
    let client = reqwest::Client::new();
    // Act
    let _ = client
        .get(format!("{}/", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
}

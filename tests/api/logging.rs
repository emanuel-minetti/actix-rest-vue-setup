use crate::helpers::spawn_app;
use std::fs::{File, OpenOptions};
use std::io::Read;

#[tokio::test]
async fn writing_to_logfile_works() {
    // Arrange
    let test_app = spawn_app();
    let path_string = test_app.settings.log_settings().path() + ".log";
    let path = std::path::Path::new(&path_string);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open logfile for writing");
    file.set_len(0).expect("Clearing logfile failed");
    log::info!("Logfile cleared in test 'writing_to_logfile_works'");
    let client = reqwest::Client::new();
    // Act
    let _ = client
        .get(format!("{}/logtest", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    let mut file = File::open(path).expect("Failed to open logfile for reading");
    let mut file_buffer = String::new();
    let _ = file.read_to_string(&mut file_buffer);
    assert!(file_buffer.contains(r"/logtest"))
}

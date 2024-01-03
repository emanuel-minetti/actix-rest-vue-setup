use crate::helpers::spawn_app;

#[tokio::test]
async fn client_config_works() {
    // Arrange
    let test_app = spawn_app();
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(format!("{}/api/config", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    // TODO Test response contains copyright, version and a message.
}

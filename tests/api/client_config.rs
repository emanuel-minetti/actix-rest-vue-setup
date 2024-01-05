use crate::helpers::spawn_app;
use actix_rest_vue_setup::configuration::ClientSettings;

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
    // test response contains copyright, version and a message.
    let response = response
        .text()
        .await
        .expect("Failed to get request body")
        .as_str()
        .to_owned();
    let _: ClientSettings = serde_json::from_str(&response).expect("Failed to parse answer.");
}

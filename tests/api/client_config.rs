use crate::helpers::spawn_app;
use actix_rest_vue_setup::configuration::ClientSettings;

#[tokio::test]
async fn client_config_delivers_config() {
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
    let response = response
        .text()
        .await
        .expect("Failed to get response body")
        .as_str()
        .to_owned();
    let client_settings: ClientSettings =
        serde_json::from_str(&response).expect("Failed to parse answer.");
    assert_eq!(client_settings, test_app.settings.client_settings)
}

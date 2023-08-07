use crate::helpers::spawn_app;

#[tokio::test]
async fn favicon_is_send() {
    // Arrange
    spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8080/favicon.ico")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    // TODO non testing test!!
}

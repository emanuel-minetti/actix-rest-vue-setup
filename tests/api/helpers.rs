pub async fn spawn_app() {
    let server = actix_rest_vue_setup::run().expect("Failed to bind address.");
    drop(tokio::spawn(server));
}

use actix_rest_vue_setup::configuration::Settings;
use std::net::TcpListener;

pub fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let app = actix_rest_vue_setup::startup_lib::run(listener);
    drop(tokio::spawn(app.server.expect("Failed to bind address.")));

    TestApp {
        settings: app.settings,
        address: format!("http://127.0.0.1:{}", port),
    }
}

pub struct TestApp {
    pub settings: Settings,
    pub address: String,
}

use actix_rest_vue_setup::configuration::{get_configuration, Settings};
use actix_rest_vue_setup::startup_lib;

use std::net::TcpListener;

pub fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let app = startup_lib::run(listener);
    let mut settings = get_configuration().expect("Failed to read configuration");
    settings.application_port = port;
    let mut log_settings = settings.log_settings();
    log_settings.set_path("log/logfile_test".to_string());
    settings.set_log_settings(log_settings);
    startup_lib::apply_config(settings.clone());
    drop(tokio::spawn(app.expect("Failed to bind address.")));

    TestApp {
        settings: settings.clone(),
        address,
    }
}

pub struct TestApp {
    pub settings: Settings,
    pub address: String,
}

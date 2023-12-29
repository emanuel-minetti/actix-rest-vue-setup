use actix_rest_vue_setup::configuration::{get_configuration, Settings};
use actix_rest_vue_setup::startup_lib;

use rand::Rng;
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

/// Returns a `Vec` of random URLs. The returned URLs are not a valid route.
///
/// # Arguments
///
/// * `size`: The size of the returned `Vec`.
/// * `url_length`: The length of the returned URLs.
///
/// returns: Vec<String>
///
/// # Examples
///
/// ```
///
/// ```
pub fn get_random_urls(size: usize, url_length: u8) -> Vec<String> {
    let mut res = Vec::new();
    while res.len() < size {
        let candidate = get_random_string(url_length);
        if !candidate.starts_with("/api")
            && !candidate.starts_with("/login")
            && !candidate.starts_with('~')
            && !candidate.chars().next().unwrap().is_ascii_digit()
        {
            res.push(candidate);
        }
    }
    res
}

fn get_random_string(length: u8) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789~";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let index = rng.gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect()
}

use actix_rest_vue_setup::{configuration, startup_lib};

use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = configuration::get_configuration().expect("Failed to read configuration.");
    startup_lib::apply_config(settings.clone());
    let address = format!("127.0.0.1:{}", settings.application_port);
    let listener = TcpListener::bind(address)?;
    log::debug!("Server about to start");
    startup_lib::run(listener)?.await
}

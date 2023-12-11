use std::net::TcpListener;
use actix_rest_vue_setup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind port 8080");
    run(listener)?.await
}

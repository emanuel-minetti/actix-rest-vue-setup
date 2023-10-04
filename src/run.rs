use actix_rest_vue_setup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
use crate::configuration;
use actix_web::HttpResponse;

pub async fn client_config() -> HttpResponse {
    let config = configuration::get_configuration().unwrap().client_settings;
    let body = serde_json::to_string(&config).unwrap();
    HttpResponse::Ok().body(body)
}

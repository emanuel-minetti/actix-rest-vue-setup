use crate::configuration;
use actix_web::HttpResponse;

pub async fn client_config() -> HttpResponse {
    let config = configuration::get_configuration();
    match config {
        Ok(config) => {
            let body = serde_json::to_string(&config.client_settings);
            match body {
                Ok(body) => HttpResponse::Ok().body(body),
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("Could not parse JSON: {e}",))
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Couldn't read config: {e}")),
    }
}

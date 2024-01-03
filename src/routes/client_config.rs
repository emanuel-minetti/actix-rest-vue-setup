use actix_web::HttpResponse;

pub async fn client_config() -> HttpResponse {
    let body = r#"{"version": "0.0.1", "copyright": "Hallo", "message": ""}"#;
    HttpResponse::Ok().body(body)
}

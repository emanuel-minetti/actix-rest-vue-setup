use actix_web::HttpResponse;
use std::path::Path;

pub async fn health_check() -> HttpResponse {
    if Path::new("./public/").read_dir().is_err() {
        return HttpResponse::ExpectationFailed().body("Static content directory is not readable.");
    }
    HttpResponse::Ok().finish()
}

use actix_web::HttpResponse;
use std::path::Path;

pub async fn health_check() -> HttpResponse {
    // not tested via automated tests
    if Path::new("./src/vue-client/dist/").read_dir().is_err() {
        return HttpResponse::ExpectationFailed().body("Static content directory is not readable.");
    }
    HttpResponse::Ok().finish()
}

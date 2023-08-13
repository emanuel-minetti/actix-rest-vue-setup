use actix_web::{HttpResponse, Responder};

pub async fn hello_from_login() -> impl Responder {
    HttpResponse::Ok().body("Here is the Login API!")
}

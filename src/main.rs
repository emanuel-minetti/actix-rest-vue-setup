use actix_web::{App, HttpResponse, HttpServer, Responder, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

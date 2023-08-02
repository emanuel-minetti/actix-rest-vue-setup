use actix_web::{App, HttpResponse, HttpServer, Responder, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/login")
                    .route("", web::post().to(hello_from_login))
                    .route("", web::delete().to(hello_from_logout))
                    .default_service(
                        web::route()
                            .to(HttpResponse::MethodNotAllowed),
                    )
            )
            .service(
                web::scope("/api")
                    .route("/health_check", web::get().to(heath_check))
            )
            .service(
                // TODO serve static files
                web::scope("")
                    .route("/{route}", web::get().to(hello_from_default))
                    .route("/", web::get().to(hello_from_default))
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn heath_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn hello_from_login() -> impl Responder {
    HttpResponse::Ok().body("Here is the Login API!")
}

async fn hello_from_logout() -> impl Responder {
    HttpResponse::Ok().body("Here is the Logout API!")
}

async fn hello_from_default() -> impl Responder {
    HttpResponse::Ok().body("Here is the Default API!")
}

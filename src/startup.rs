use crate::routes;
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/login")
                    .route("", web::post().to(routes::hello_from_login))
                    .default_service(web::route().to(HttpResponse::MethodNotAllowed)),
            )
            .service(web::scope("/api").route("/health_check", web::get().to(routes::health_check)))
            .service(
                web::scope("")
                    .service(Files::new("/assets", "./src/vue-client/dist/assets"))
                    .route("/favicon.ico", web::get().to(routes::return_favicon))
                    .route("/{route}", web::get().to(routes::return_index))
                    .route("/", web::get().to(routes::return_index)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    Ok(server)
}

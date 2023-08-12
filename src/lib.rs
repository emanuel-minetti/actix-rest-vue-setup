use actix_files::{Files, NamedFile};
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::path::Path;

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/login")
                    .route("", web::post().to(hello_from_login))
                    .route("", web::delete().to(hello_from_logout))
                    .default_service(web::route().to(HttpResponse::MethodNotAllowed)),
            )
            .service(web::scope("/api").route("/health_check", web::get().to(health_check)))
            .service(
                web::scope("")
                    .service(Files::new("/assets", "./src/vue-client/dist/assets"))
                    .route("/favicon.ico", web::get().to(return_favicon))
                    .route("/{route}", web::get().to(return_index))
                    .route("/", web::get().to(return_index)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    Ok(server)
}

async fn health_check() -> HttpResponse {
    // not tested via automated tests
    if Path::new("./src/vue-client/dist/").read_dir().is_err() {
        return HttpResponse::ExpectationFailed().body("Static content directory is not readable.");
    }
    HttpResponse::Ok().finish()
}

async fn hello_from_login() -> impl Responder {
    HttpResponse::Ok().body("Here is the Login API!")
}

async fn hello_from_logout() -> impl Responder {
    HttpResponse::Ok().body("Here is the Logout API!")
}

async fn return_favicon() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("./src/vue-client/dist/favicon.ico")
}

async fn return_index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("./src/vue-client/dist/index.html")
}

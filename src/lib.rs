use actix_files::{Files, NamedFile};
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;

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

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn hello_from_login() -> impl Responder {
    HttpResponse::Ok().body("Here is the Login API!")
}

async fn hello_from_logout() -> impl Responder {
    HttpResponse::Ok().body("Here is the Logout API!")
}

async fn return_favicon() -> Result<NamedFile,std::io::Error> {
    Ok(NamedFile::open("./src/vue-client/dist/favicon.ico")?)
}

async fn return_index() -> Result<NamedFile,std::io::Error> {
    Ok(NamedFile::open("./src/vue-client/dist/index.html")?)
}
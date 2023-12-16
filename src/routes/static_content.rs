use actix_files::NamedFile;

pub async fn return_favicon() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("./public/favicon.ico")
}

pub async fn return_index() -> Result<NamedFile, std::io::Error> {
    for i in 0..250 {
        log::info!("To test rolling file configurations we print this message in a loop. This is loop nr. {}", i);
    }
    NamedFile::open("./public/index.html")
}

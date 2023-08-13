use actix_files::NamedFile;

pub async fn return_favicon() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("./src/vue-client/dist/favicon.ico")
}

pub async fn return_index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("./src/vue-client/dist/index.html")
}

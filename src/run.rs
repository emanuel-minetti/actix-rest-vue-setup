use actix_rest_vue_setup::configuration::get_configuration;
use actix_rest_vue_setup::run;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = get_configuration().expect("Failed to read configuration.");
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)}: [{l}] - {m}\n")))
        .build(&settings.logfile_path)?;
    let log_config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(settings.loglevel()))
        .expect("Could not build logging configuration.");
    log4rs::init_config(log_config).expect("Could not initialize logging.");
    let address = format!("127.0.0.1:{}", settings.application_port);
    let listener = TcpListener::bind(address)?;
    log::debug!("Server about to start");
    run(listener)?.await
}

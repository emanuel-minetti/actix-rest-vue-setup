use actix_rest_vue_setup::configuration::get_configuration;
use actix_rest_vue_setup::run;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = get_configuration().expect("Failed to read configuration.");
    let trigger = SizeTrigger::new(settings.log_settings.size());
    let roll = FixedWindowRoller::builder()
        .base(1)
        .build((settings.log_settings.path().to_owned() + "-{}").as_str(), settings.log_settings.number())
        .expect("Failed to build log file roller");
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roll));
    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)}: [{l}] - {m}{n}",
        )))
        .build(settings.log_settings.path(), Box::new(policy))?;
    let log_config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(settings.log_settings.level()),
        )
        .expect("Could not build logging configuration.");
    log4rs::init_config(log_config).expect("Could not initialize logging.");
    let address = format!("127.0.0.1:{}", settings.application_port);
    let listener = TcpListener::bind(address)?;
    log::debug!("Server about to start");
    run(listener)?.await
}

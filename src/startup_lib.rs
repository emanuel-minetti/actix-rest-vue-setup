use crate::configuration::{LogSettings, Settings};
use crate::routes;

use actix_files::Files;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, HttpResponse, HttpServer};
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::{Command, Stdio};
use sysinfo::{Pid, RefreshKind, System};

const PID_FILE_PATH: &str = "actix-rest-vue-setup.pid";
const PROCESS_NAME: &str = "actix-rest-vue-setup-run";

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let not_running_server = HttpServer::new(|| {
        actix_web::App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/login")
                    .route("", web::post().to(routes::hello_from_login))
                    .default_service(web::route().to(HttpResponse::MethodNotAllowed)),
            )
            .service(
                web::scope("/api")
                    .route("/health_check", web::get().to(routes::health_check))
                    .route("/config", web::get().to(routes::health_check)),
            )
            .service(
                web::scope("")
                    .service(Files::new("/assets", "./public/assets"))
                    .route("/favicon.ico", web::get().to(routes::return_favicon))
                    .route("/{route}", web::get().to(routes::return_index))
                    .route("/", web::get().to(routes::return_index)),
            )
    })
    .listen(listener);

    match not_running_server {
        Ok(server) => Ok(server.run()),
        Err(err) => Err(err),
    }
}

/// Applies the given settings. To be used before using `run()`
///
/// # Arguments
///
/// * `settings`: The settings to be used.
///
/// returns: ()
///
/// # Examples
///
/// ```
/// use actix_rest_vue_setup::configuration::get_configuration;
/// use actix_rest_vue_setup::startup_lib::apply_config;
///
/// let settings = get_configuration().unwrap();
/// apply_config(settings);
/// ```
pub fn apply_config(settings: Settings) {
    config_logs(settings.log_settings())
}

fn config_logs(settings: LogSettings) {
    let trigger = SizeTrigger::new(settings.size());
    let roll = FixedWindowRoller::builder()
        .base(1)
        .build(
            (settings.path().to_owned() + "-{}.log").as_str(),
            settings.number(),
        )
        .expect("Failed to build log file roller");
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roll));
    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)}: [{l}] - {m}{n}",
        )))
        .build(settings.path().to_owned() + ".log", Box::new(policy))
        .expect("Failed to build file appender");
    let log_config = Config::builder()
        .appender(Appender::builder().build(settings.path(), Box::new(logfile)))
        .build(
            Root::builder()
                .appender(settings.path())
                .build(settings.level()),
        )
        .expect("Could not build logging configuration.");
    match log4rs::init_config(log_config) {
        Ok(_) => {}
        Err(e) => {
            println!("error was {e}");
        }
    }
}

pub fn start(forced: bool) {
    println!("Starting");
    if forced {
        stop(true);
    }
    if is_running() {
        println!("Seems to be running. Consider stopping.");
        std::process::exit(0)
    }
    let mut pid_file = File::create(PID_FILE_PATH).expect("Could not create pid file.");
    let child = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(PROCESS_NAME)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to run the server.");
    let pid = child.id().to_string();
    pid_file
        .write_all(pid.as_ref())
        .expect("Could not write to pid file.");
    println!("Started pid: {}", pid);
    std::process::exit(0)
}

pub fn stop(forced: bool) {
    if !forced {
        println!("Stopping");
        if !is_running() {
            println!("Seems to be not running. Try starting.");
            std::process::exit(0)
        }
        let mut pid_file = File::open(PID_FILE_PATH).expect("Failed to open pid file.");
        let pid_file_content = &mut "".to_string();
        pid_file
            .read_to_string(pid_file_content)
            .expect("Failed to read pid file.");
        let pid = pid_file_content
            .parse::<usize>()
            .expect("Pin file content should parse to a number");
        let binding = System::new_with_specifics(RefreshKind::with_processes(
            RefreshKind::default(),
            Default::default(),
        ));
        let pid = Pid::from(pid);
        let process = binding.process(pid);
        match process {
            None => {
                println!("No process with pid {}", pid);
                std::fs::remove_file(PID_FILE_PATH).expect("Couldn't remove pid file");
                println!("Stopped.");
            }
            Some(process) => {
                process.kill();
                std::fs::remove_file(PID_FILE_PATH).expect("Couldn't remove pid file");
                println!("Stopped pid: {}", pid);
            }
        }
    } else {
        println!("Force stopping");
        if is_running() {
            stop(false)
        } else {
            let system_binding = System::new_with_specifics(RefreshKind::with_processes(
                RefreshKind::default(),
                Default::default(),
            ));
            let processes = system_binding.processes_by_name(PROCESS_NAME);
            processes.for_each(|process| {
                process.kill();
                println!("Stopped {} with pid: {}", process.name(), process.pid());
            })
        }
    }
}

pub fn status() {
    println!(
        "Status: {}",
        if is_running() { "Running" } else { "Stopped" }
    );
}

pub fn restart() {
    println!("Restarting");
    if !is_running() {
        println!("Seems not to be running. Consider starting.");
        std::process::exit(0)
    }
    stop(false);
    start(false);
}

pub fn print_usage(called: &str) {
    println!(
        "Usage: {} (start | status | stop | restart) [options]\n\
    The only option supported is:\n\
      \t--force\n\
      \t\tWhen used with `start` forces a restart even if the server is running.
      \t\tWhen used with `stops` forces a stop. If pid file is missing, kills all running instances.",
        called
    );
}

fn is_running() -> bool {
    let pid_file_path = Path::new(PID_FILE_PATH);
    pid_file_path.exists()
}

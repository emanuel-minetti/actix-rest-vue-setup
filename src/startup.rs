use crate::routes;
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use sysinfo::{Pid, ProcessExt, RefreshKind, System, SystemExt};

const PID_FILE_PATH: &str = "actix-rest-vue-setup.pid";
const PROCESS_NAME: &str = "actix-rest-vue-setup-run";

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
                    .service(Files::new("/assets", "./public/assets"))
                    .route("/favicon.ico", web::get().to(routes::return_favicon))
                    .route("/{route}", web::get().to(routes::return_index))
                    .route("/", web::get().to(routes::return_index)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    Ok(server)
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
        .arg("actix-rest-vue-setup-run")
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

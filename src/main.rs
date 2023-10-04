use std::env;
use std::fs::{File, remove_file};
use std::io::Write;
use std::path::Path;
use std::process::{Command, exit, Stdio};

const PID_FILE_PATH: &str = "actix-rest-vue-setup.pid";

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_usage(&args[0]);
        exit(0);
    }
    match args[1].as_ref() {
        "start" => {start()},
        "status" => {status()},
        "stop" => {stop()},
        "restart" => {println!("Restarting")},
        _ => print_usage(&args[0])
    }
}

fn start() -> () {
    println!("Starting");
    let mut pid_file = File::create(PID_FILE_PATH)
        .expect("Could not create pid file.");
    let child = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("actix-rest-vue-setup-run")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to run the server.");
    let pid = child.id().to_string();
    pid_file.write(pid.as_ref()).expect("Could not write to pid file.");
    exit(0)
}

fn stop() -> () {
    println!("Stopping");
    remove_file(PID_FILE_PATH).expect("Couldn't remove pid file");
}

fn status() -> () {
    println!("Status: {}", if is_running() {"Running"} else {"Stopped"});
}

fn print_usage(called: &str) -> () {
    println!("Usage: {} (start | status | stop | restart)", called);
}

fn is_running() -> bool {
    let pid_file_path = Path::new(PID_FILE_PATH);
    pid_file_path.exists()
}
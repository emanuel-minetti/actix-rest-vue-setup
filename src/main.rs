use std::env;
use std::fs::{File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use sysinfo::{System, SystemExt, Pid, ProcessExt, RefreshKind};

const PID_FILE_PATH: &str = "actix-rest-vue-setup.pid";

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_usage(&args[0]);
        std::process::exit(0);
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
    if is_running() {
        println!("Seems to be running. Consider stopping.");
        std::process::exit(0)
    }
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
    println!("Started pid: {}", pid);
    std::process::exit(0)
}

fn stop() -> () {
    println!("Stopping");
    if !is_running() {
        println!("Seems to be not running. Try starting.");
        std::process::exit(0)
    }
    let mut pid_file = File::open(PID_FILE_PATH).expect("Failed to open pid file.");
    let pid_file_content = &mut "".to_string();
    pid_file.read_to_string(pid_file_content).expect("Failed to read pid file.");
    let pid = pid_file_content.parse::<usize>()
        .expect("Pin file content should parse to a number");
    let binding = System::new_with_specifics(
        RefreshKind::with_processes(
            RefreshKind::default(), Default::default()));
    let pid = Pid::from(pid);
    let process = binding.process(pid).expect("Pid process should exist.");
    process.kill();
    std::fs::remove_file(PID_FILE_PATH).expect("Couldn't remove pid file");
    println!("Stopped pid: {}", pid);
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
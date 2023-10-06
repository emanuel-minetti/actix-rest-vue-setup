use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use sysinfo::{Pid, ProcessExt, RefreshKind, System, SystemExt};

const PID_FILE_PATH: &str = "actix-rest-vue-setup.pid";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => print_usage(&args[0]),
        2 => match args[1].as_ref() {
            "start" => start(false),
            "status" => status(),
            "stop" => stop(),
            "restart" => restart(),
            _ => print_usage(&args[0]),
        },
        3 => {
            if &args[1] == "start" && &args[2] == "--force" {
                start(true);
            } else {
                print_usage(&args[0])
            }
        }
        _ => print_usage(&args[0]),
    }
}

fn start(forced: bool) {
    println!("Starting");
    if forced {
        stop();
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

fn stop() {
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
    let process = binding.process(pid).expect("Pid process should exist.");
    process.kill();
    std::fs::remove_file(PID_FILE_PATH).expect("Couldn't remove pid file");
    println!("Stopped pid: {}", pid);
}

fn status() {
    println!(
        "Status: {}",
        if is_running() { "Running" } else { "Stopped" }
    );
}

fn restart() {
    println!("Restarting");
    if !is_running() {
        println!("Seems not to be running. Consider starting.");
        std::process::exit(0)
    }
    stop();
    start(false);
}

fn print_usage(called: &str) {
    println!(
        "Usage: {} (start | status | stop | restart) [options]\n\
    The only option supported is:\n\
      \t--force\n\
      \t\tWhen used with `start` forces a restart even if the server is running.",
        called
    );
}

fn is_running() -> bool {
    let pid_file_path = Path::new(PID_FILE_PATH);
    pid_file_path.exists()
}

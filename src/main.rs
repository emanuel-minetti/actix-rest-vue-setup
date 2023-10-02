use std::env;
use std::fs::{File, remove_file};
use std::path::Path;
use std::process::exit;
//use actix_rest_vue_setup::run;

const PID_FILE_PATH: &str = "actix-rest-vue-setup.pid";

#[tokio::main]
async fn main() -> () {

//async fn main() -> Result<(), std::io::Error> {
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
    let mut _pid_file = File::create(PID_FILE_PATH);
    //run()?.await
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
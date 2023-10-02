use std::env;
use std::fs::{File, remove_file};
use std::process::exit;
//use actix_rest_vue_setup::run;

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
        "status" => {println!("Status")},
        "stop" => {stop()},
        "restart" => {println!("Restarting")},
        _ => print_usage(&args[0])
    }
    //run()?.await
}

fn start() -> () {
    println!("Starting");
    let mut _pid_file = File::create("actix-rest-vue-setup.pid");
    //let temp_file = temp_dir().push("actix-rest-vue-setup.pid");
}

fn stop() -> () {
    println!("Stopping");
    remove_file("actix-rest-vue-setup.pid").expect("Couldn't remove pid file");
}

fn print_usage(called: &str) -> () {
    println!("Usage: {} (start | status | stop | restart)", called);
}

use std::env;
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
        "start" => {println!("Starting")},
        "status" => {println!("Status")},
        "stop" => {println!("Stopping")},
        "restart" => {println!("Restarting")},
        _ => print_usage(&args[0])
    }
    //run()?.await
}

fn print_usage(called: &str) -> () {
    println!("Usage: {} (start | status | stop | restart)", called);
}

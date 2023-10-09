use actix_rest_vue_setup::startup;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => startup::print_usage(&args[0]),
        2 => match args[1].as_ref() {
            "start" => startup::start(false),
            "status" => startup::status(),
            "stop" => startup::stop(false),
            "restart" => startup::restart(),
            _ => startup::print_usage(&args[0]),
        },
        3 => {
            if &args[2] == "--force" {
                match args[1].as_ref() {
                    "start" => startup::start(true),
                    "stop" => startup::stop(true),
                    _ => startup::print_usage(&args[0]),
                }
            } else {
                startup::print_usage(&args[0])
            }
        }
        _ => startup::print_usage(&args[0]),
    }
}

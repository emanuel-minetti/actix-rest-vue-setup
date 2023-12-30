use actix_rest_vue_setup::startup_lib;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => startup_lib::print_usage(&args[0]),
        2 => match args[1].as_ref() {
            "start" => startup_lib::start(false),
            "status" => startup_lib::status(),
            "stop" => startup_lib::stop(false),
            "restart" => startup_lib::restart(),
            _ => startup_lib::print_usage(&args[0]),
        },
        3 => {
            if &args[2] == "--force" {
                match args[1].as_ref() {
                    "start" => startup_lib::start(true),
                    "stop" => startup_lib::stop(true),
                    _ => startup_lib::print_usage(&args[0]),
                }
            } else {
                startup_lib::print_usage(&args[0])
            }
        }
        _ => startup_lib::print_usage(&args[0]),
    }
}

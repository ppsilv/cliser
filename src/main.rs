use std::env;
use syslog::{Facility, Formatter3164, BasicLogger};
use std::process;

pub mod server;
pub mod client;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} <servidor|cliente>", args[0]);
        process::exit(1);
    }

    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "cliser".into(),
        pid: std::process::id() as u32,
    };

    // Initialize the logger
    let logger = syslog::unix(formatter).expect("could not connect to syslog");

    // Set the logger as the global logger
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .expect("could not set logger");
    
     if args.len() < 2 {
         eprintln!("Usage: {} <name> [server|client]", args[0]);
         std::process::exit(1);
     }

    match args[1].as_str() {
        "server" => server::server(),
        "client" => client::client(),
        _ => {
            eprintln!("Uso: {} <servidor|cliente>", args[0]);
            process::exit(1);
        }
    }
}
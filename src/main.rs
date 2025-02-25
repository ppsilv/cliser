use std::env;
use syslog::{Facility, Formatter3164, BasicLogger};
use std::process;
use std::thread;
use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use std::io::{Read,Write}; // Import the `Read` trait

mod servermod;
mod server;
mod client;


fn main() {
    let args: Vec<String> = env::args().collect();

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
        "server" => {
            thread::spawn(move || {
                server::conection_manager();
            });
        },
        "client" => {
            thread::spawn(move || {
                client::client();
            });
        },
        _ => {
            eprintln!("Usage: {} <server|client>", args[0]);
            process::exit(1);
        }
    }
    loop{
        let ip_port: String = servermod::config::get_hostip() + ":" + &servermod::config::get_port3();
        println!("Listening on {}...", ip_port);

        let listener = TcpListener::bind(ip_port).unwrap();

        // Accept a TCP connection
        let (stream, _) = listener.accept().unwrap();

        thread::spawn(move || {
            handle_backdoor_client_port3(stream );
        });
    }
}


fn handle_backdoor_client_port3(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    log::info!("Backdoor port connected: {}", stream.peer_addr().unwrap());

    loop {
        // Read data from the client
        stream.write("#>".as_bytes()).unwrap();
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Client disconnected
                log::error!("Client backdoor disconnected .");
            break;
            }
            Ok(n) => {
                // Echo the data back to the client
                let received_data = String::from_utf8_lossy(&buffer[..n]);
                log::info!("handle_read_client_port3:Server: Received: {}", received_data);
                // Handle the LISTAR command
                if received_data.trim() == "L" || received_data.trim() == "l" {
                    let clientes: String = servermod::clientdata::list_all_clients2();                   
                    stream.write(clientes.as_bytes()).unwrap();
                    continue;
                }else if received_data.trim() == "E" || received_data.trim() == "E" {
                    log::info!("E: exiting...");               
                    break
                }else{
                    stream.write("Invalid command\n".as_bytes()).unwrap();
                    continue;
                }         
                
                
            }
            Err(e) => {
                log::error!("Server: Failed to read from client: {}", e);
                break;
            }
        }

        // Simulate some processing time
        thread::sleep(Duration::from_secs(1));
    }
    println!("client backdoor desconecting...");
}

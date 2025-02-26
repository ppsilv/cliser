use std::sync::mpsc;
use std::thread;
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::time::Duration;
//use std::env;
use syslog::{Facility, Formatter3164, BasicLogger};
use std::process;
mod configser;
mod clientdata;
// Define the server function in the server module
pub fn conection_manager() {
    log::info!("Server: Server running...");

    loop{
        // Create a TCP listener for main connection
        let ip_port: String = configser::get_hostip() + ":" + &configser::get_port1();
        log::info!("Listening on {}...", ip_port);
        let listener = TcpListener::bind(ip_port).unwrap();
    
        // Accept a TCP connection
        let (stream, _) = listener.accept().unwrap();
        log::info!("Client connected!");
        
        let stream_clone = match stream.try_clone() {
            Ok(clone) => clone,
            Err(e) => {
                log::error!("Failed to clone TcpStream: {}", e);
                continue;
            }
        };
        let stream_clone_ger_msg = match stream.try_clone() {
            Ok(clone) => clone,
            Err(e) => {
                log::error!("Failed to clone TcpStream: {}", e);
                continue;
            }
        };

        // Create a channel for communication
        let (sender_tcp_reader, receiver_from_tcp_reader) = mpsc::channel();
        let (sender_to_tcp_writer, receiver_tcp_writer) = mpsc::channel();

        
        // Spawn a thread to read from the TCP stream
        thread::spawn(move || {
            tcp_reader(stream, sender_tcp_reader);
        });
    
        // Spawn a thread to write to the TCP stream
        thread::spawn(move || {
            tcp_writer(stream_clone, receiver_tcp_writer);
        });

        let (sender_to_ger_client, receiver_from_backdoor) = mpsc::channel();
        // Spawn a thread to read from the TCP stream
        thread::spawn(move || {
            ger_client(stream_clone_ger_msg,sender_to_tcp_writer, receiver_from_tcp_reader, receiver_from_backdoor);
        });

        thread::spawn(move || {
            handle_backdoor_client_port3(sender_to_ger_client);
        });    
    }
}

fn ger_client(mut stream: TcpStream, sender: mpsc::Sender<String>, receiver: mpsc::Receiver<String>, receiver_from_backdoor: mpsc::Receiver<String>) {
    //1.1.3.1 - Envia para a  msgqueue tcp_writer.receiver um comando para pedir a senha para o cliente.
    let msg: String = "110: qual a senha".to_string();
    sender.send(msg).unwrap();

    //1.1.3.2 - Aguarda na msgqueue tcp_reader.receiver a senha. Valida a senha e se for inválida 
    //          desconecta o cliente e volta a ouvir o stream tcp/ip.    
    for password in receiver.iter() {
        if password == configser::get_password() {
            log::info!("ger_client: Senha correta");
            break;
        } else {
            log::info!("ger_client: Senha incorreta");
            stream.write_all("Senha incorreta desconnecting client".as_bytes()).unwrap();
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            return;
        }
    }
    log::info!("ger_client: Password validated OK");

    let client_id: String = "120: qual o ID".to_string();
    sender.send(client_id).unwrap();

    //1.1.3.2 - Aguarda na msgqueue tcp_reader.receiver ID.   
    let client_id = receiver.recv().unwrap();
    log::info!("ger_client: Received client ID {}", client_id);

    //Spliting IP and port from ...
    // Store the result of `split_once` in a temporary variable
    let clientip = stream.peer_addr().unwrap().to_string();
    let (ip, port) = if let Some((ip, port)) = clientip.split_once(':') {
        (ip, port)
    } else {
        eprintln!("Invalid client IP format");
        return; // Or handle the error appropriately
    };

    let u16client_id = client_id.parse::<u16>().unwrap();
    
    match clientdata::find_client_by_id(u16client_id) {
        Some(client) => {
            println!("Server: Client found: {:?} disconecting", client);
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            return;
        }
        None => {
            // Now you can safely mutate `client_ip` and `client_port`
            let client_ip = ip.to_string();
            let client_port = port.to_string();    

            let client1 = clientdata::ClientData {
                id: u16client_id,
                ip: client_ip,
                status: "active".to_string(),
                port: client_port,
                cid: client_id.to_string(),
            };
            log::info!("ger_client:  Saving client data: {:?}", client1);
            clientdata::save_client(client1);
        }
    }
    //🌞
    let connected: String = "140: Connected".to_string();
    sender.send(connected).unwrap();
    let timeout: u128 = 10000;
    let start = std::time::Instant::now();
    let elapsed = start.elapsed();
    let mut timerovrflow: u128= timeout + elapsed.as_millis();
    loop{
        if elapsed.as_millis() > timerovrflow {
            timerovrflow = timeout + elapsed.as_millis();
            log::info!("ger_client:  Timeout, closing connection");
            let keep_alive: String = "100: keep alive".to_string();
            sender.send(keep_alive).unwrap();
        }else{
            let keep_alive: String = "1".to_string();
            sender.send(keep_alive).unwrap();
        }
        
        match receiver.recv_timeout(Duration::from_millis(500)) {
            Ok(message) => {
                log::info!("ger_client:  Received message: {}", message);
                if message == "exit" {
                    break;
                }            
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                //println!("No data available yet, waiting...");
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("ger_client:  has disconnected, exiting...");
                break;
            }
        }
        match receiver_from_backdoor.recv_timeout(Duration::from_millis(500)) {
            Ok(message) => {
                log::info!("ger_client:  Received from backdoor: {}", message);
                if message == "201:" {
                    let shutdown: String = "999: shutdown".to_string();
                    sender.send(shutdown).unwrap();
                    thread::sleep(Duration::from_millis(5000));
                    process::exit(0);
                    return;
                }      
                if message == "200:" {
                    let keep_alive: String = "100: keep alive".to_string();
                    sender.send(keep_alive).unwrap();
                    continue;
                }      
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                //println!("No data available yet, waiting...");
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("ger_client: backdoor has disconnected, exiting...");
                break;
            }
        }
    }
}

// Function to read from a TCP stream and send data to a channel
fn tcp_reader(mut stream: TcpStream, sender: mpsc::Sender<String>) {
    let mut buffer = [0; 512];
    loop {
        // Read data from the TCP stream
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            log::info!("tcp_reader: Client has closed the connection...");
            break; // Connection closed
        }

        // Convert the data to a String and send it through the channel
        let data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        sender.send(data).unwrap();
    }
}

// Function to write data received from a channel to a TCP stream
fn tcp_writer(mut stream: TcpStream, receiver: mpsc::Receiver<String>) {
    for received_data in receiver {
        log::info!("tcp_writer: Sending data: {}", received_data);
        // Write the received data to the TCP stream
        stream.write_all(received_data.as_bytes()).unwrap();
    }
}

fn main() {
   // let args: Vec<String> = env::args().collect();

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

    println!("path: {:?}",configser::print_path()) ;   

    thread::spawn(move || {
        conection_manager();
    });
    loop{
        thread::sleep(Duration::from_millis(500000));
        /*
        let ip_port: String = configser::get_hostip() + ":" + &configser::get_port3();
        println!("Listening on {}...", ip_port);

        let listener = TcpListener::bind(ip_port).unwrap();

        // Accept a TCP connection
        let (stream, _) = listener.accept().unwrap();

        thread::spawn(move || {
            handle_backdoor_client_port3(stream );
        });
        */
    }
}


fn handle_backdoor_client_port3(sender_to_ger_client:  mpsc::Sender<String>) {
    let mut buffer = [0; 512];
    let ip_port: String = configser::get_hostip() + ":" + &configser::get_port3();
    println!("Listening on {}...", ip_port);

    let listener = TcpListener::bind(ip_port).unwrap();

    // Accept a TCP connection
    let (mut stream, _) = listener.accept().unwrap();


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
                log::info!("Backdoor: Received: {}", received_data);
                let mut cmdtotcp: &str = "";
                if let Some(cmdtotcp1) = received_data.get(0..4) {
                    println!("Backdoor: cmdtotcp {}", cmdtotcp); // Output: "Hello"
                    cmdtotcp = cmdtotcp1;
                } else {
                    log::info!("Backdoor: No command to tcp/ip");
                }                
                
                if received_data.trim() == "200:" {
                    sender_to_ger_client.send(cmdtotcp.to_string()).unwrap();
                    continue;
                }
                if received_data.trim() == "201:" {
                    sender_to_ger_client.send(cmdtotcp.to_string()).unwrap();
                    continue;
                }// Handle the LISTAR command
                if received_data.trim() == "L" || received_data.trim() == "l" {
                        let clientes: String = clientdata::list_all_clients2();                   
                    stream.write(clientes.as_bytes()).unwrap();
                    continue;
                }else if received_data.trim() == "E" || received_data.trim() == "E" {
                    log::info!("Backdoor: E cmd exiting...");               
                    break
                }else{
                    stream.write("Invalid command\n".as_bytes()).unwrap();
                    continue;
                }         
            }
            Err(e) => {
                log::error!("Backdoor: Failed to read from client: {}", e);
                break;
            }
        }

        // Simulate some processing time
      //  thread::sleep(Duration::from_secs(1));
    }
    println!("client backdoor desconecting...");
}

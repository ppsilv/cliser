use std::sync::mpsc;
use std::thread;
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::time::Duration;

pub mod config;
pub mod clientdata;
//use config::Config;



// Define the server function in the server module
pub fn conection_manager() {
    log::info!("Server: Server running...");

    loop{
        // Create a TCP listener
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        println!("Listening on 127.0.0.1:8080...");
    
        // Accept a TCP connection
        let (stream, _) = listener.accept().unwrap();
        println!("Client connected!");
        
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

        // Spawn a thread to read from the TCP stream
        thread::spawn(move || {
            ger_client(stream_clone_ger_msg,sender_to_tcp_writer, receiver_from_tcp_reader);
        });
    
    }
}

fn ger_client(mut stream: TcpStream, sender: mpsc::Sender<String>, receiver: mpsc::Receiver<String>) {
    //1.1.3.1 - Envia para a  msgqueue tcp_writer.receiver um comando para pedir a senha para o cliente.
    let msg: String = "110: qual a senha".to_string();
    sender.send(msg).unwrap();

    //1.1.3.2 - Aguarda na msgqueue tcp_reader.receiver a senha. Valida a senha e se for inválida 
    //          desconecta o cliente e volta a ouvir o stream tcp/ip.    
    for password in receiver.iter() {
        if password == config::get_password() {
            println!("Senha correta");
            break;
        } else {
            println!("Senha incorreta");
            stream.write_all("Senha incorreta desconnecting client".as_bytes()).unwrap();
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            return;
        }
    }
    log::info!("Server: Password validated OK");

    let client_id: String = "120: qual o ID".to_string();
    sender.send(client_id).unwrap();

    //1.1.3.2 - Aguarda na msgqueue tcp_reader.receiver ID.   
    let client_id = receiver.recv().unwrap();
    log::info!("Server: Received client ID {}", client_id);

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
            log::info!("Server: Saving client data: {:?}", client1);
            clientdata::save_client(client1);
        }
    }
    
    let connected: String = "140: Connected".to_string();
    sender.send(connected).unwrap();
    loop{
        match receiver.recv_timeout(Duration::from_millis(500)) {
            Ok(message) => {
                log::info!("Server: Received message: {}", message);
                if message == "exit" {
                    break;
                }            
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                //println!("No data available yet, waiting...");
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("Sender has disconnected, exiting...");
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
        // Write the received data to the TCP stream
        stream.write_all(received_data.as_bytes()).unwrap();
    }
}
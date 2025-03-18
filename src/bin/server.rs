use std::sync::mpsc::{self};
use std::thread;
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::time::Duration;
use syslog::{Facility, Formatter3164, BasicLogger};
use std::process;
use log::{info, error};

mod configser;

mod clientdata;
//😊😊😊😊😊😊

// Define the server function in the server module
pub fn conection_manager() {
    log::info!("Server: Server running...");
    
    thread::spawn(move || {
        handle_backdoor_client_port3();
    });    

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

        // Spawn a thread to read from the TCP stream
        thread::spawn(move || {
            auth_manager(stream_clone_ger_msg,sender_to_tcp_writer, receiver_from_tcp_reader);
        });

    }
}

fn auth_manager(mut stream: TcpStream, sender: mpsc::Sender<String>, receiver: mpsc::Receiver<String>) {
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

    let u16client_id = match client_id.parse::<u16>() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Invalid client ID format");
            stream.write_all("Invalid client ID format".as_bytes()).unwrap();
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            return;
        }
    };
    
    match clientdata::ClientData::find_client_by_id(u16client_id) {
        Some(client) => {
            println!("Server: Client found: {:?} disconecting", client);
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            return;
        }
        None => {
            // Now you can safely mutate `client_ip` and `client_port`
            let client_ip = ip.to_string();
            let client_port = port.to_string();  

            let (_sender_tcp_reader,  _receiver_tcp_reader): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

            let client_data = clientdata::ClientData {
                    id: u16client_id,
                    ip: client_ip,
                    status: "active".to_string(),
                    port: client_port,
                    cid: client_id.to_string(),
                    sender_tcp_writer: sender.clone(),
            };            
            log::info!("ger_client:  Saving client data: {:?}", client_data);
            clientdata::ClientData::save(client_data);
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
        let elapsed = start.elapsed(); // Update elapsed time inside the loop
        //println!("ger_client:  Verificando se timer over flow...{} timerovrflow: {}",elapsed.as_millis(),timerovrflow);
        if elapsed.as_millis() > timerovrflow {
            timerovrflow = timeout + elapsed.as_millis();
            //log::info!("ger_client:  Timeout, closing connection");
            let keep_alive: String = "100: keep alive".to_string();
            clientdata::ClientData::round_robin(keep_alive);
        }else{
            let keep_alive: String = "1".to_string();
            //clientdata::ClientData::round_robin(keep_alive);
            //or
            //clientdata::ClientData::send_client_msg_by_id(u16client_id,keep_alive);
        }
        //println!("ger_client:  Waiting for message from client...");
        match receiver.recv_timeout(Duration::from_millis(500)) {
            Ok(message) => {
                //TODO: Filtrar qual mensagem chegou, nesse momento só temos a mensagem de desligar o cliente.
                log::info!("ger_client:  Received message: {} for u16client_id [{}]", message, u16client_id);
                //clientdata::ClientData::update_status(u16client_id, "inactive".to_string());
                if clientdata::ClientData::delete_client_by_id(u16client_id) {
                    log::info!("ger_client:  Client deleted: {}", u16client_id);
                }else{
                    log::info!("ger_client:  Client not deleted: {}", u16client_id);
                }
                if message.contains("999:") {
                    log::info!("ger_client:  Thread exiting");
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
    }
}

// Function to read from a TCP stream and send data to a channel
fn tcp_reader(mut stream: TcpStream, sender: mpsc::Sender<String>) {
    let mut buffer = [0; 512];
    loop {
        // Read data from the TCP stream
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            sender.send("999: shutdown".to_string()).unwrap();
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
        if received_data.contains("999:") {
            log::info!("tcp_writer: Client will close the connection...");
            break;
        }
    }
}

fn main() {
   // let args: Vec<String> = env::args().collect();

    info!("Iniciando servidor...");
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "cliser".into(),
        pid: std::process::id() as u32,
    };

    // Initialize the logger
    //let logger = syslog::unix(formatter).expect("could not connect to syslog");
        let logger = syslog::unix_custom(formatter, "/run/systemd/journal/dev-log")
        .expect("could not connect to syslog");

    // Set the logger as the global logger
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .expect("could not set logger");

   // println!("path: {:?}",configser::print_path()) ;   

    thread::spawn(move || {
        conection_manager();
    });
    loop{
        thread::sleep(Duration::from_millis(500000));
        /*
            Aqui eu vou por o código que busca a informação de se tem energia ou não.
            Caso a energia esteja desligada, eu vou mandar uma mensagem para todos os clientes
            para que eles se desliguem.
        */
        //clientdata::ClientData::round_robin( "100: keep alive".to_string());

    }
}

use std::io::{self};
use std::time::Instant;

fn handle_backdoor_client_port3() {
loop{
    let ip_port: String = configser::get_hostip() + ":" + &configser::get_port3();
    println!("Listening on {}...", ip_port);

    let listener = TcpListener::bind(ip_port).unwrap();

    // Accept a TCP connection
    let (mut stream, _) = listener.accept().unwrap();


    log::info!("Backdoor port connected: {}", stream.peer_addr().unwrap());

    // Define um timeout de 5 segundos
    let timeout1 = Duration::from_secs(5);

    let mut buffer = [0; 1024]; // Buffer para armazenar os dados lidos
    let start_time = Instant::now(); // Marca o início do timeout

    // Configura o stream para modo não bloqueante
    stream.set_nonblocking(true).unwrap();

    let timeout: u128 = 1000;
    let start = std::time::Instant::now();
    let elapsed = start.elapsed();
    let mut timerovrflow: u128= timeout + elapsed.as_millis();
    
    stream.write_all("#> ".as_bytes()).unwrap();
    loop {
            // Tenta ler dados do stream
            match stream.read(&mut buffer) {
            Ok(0) => {
                // Fim da leitura (conexão fechada pelo outro lado)
                break;
            }
            Ok(n) => {
                // Adiciona os dados lidos ao vetor de resultado
                let received_data = String::from_utf8_lossy(&buffer[..n]);
                let mut cmdtotcp: &str = "none";
                if let Some(cmdtotcp1) = received_data.get(0..4) {
                    println!("Backdoor: cmdtotcp {}", cmdtotcp); // Output: "Hello"
                    cmdtotcp = cmdtotcp1;
                } else {
                    log::info!("Backdoor: No command to tcp/ip");
                }                
                
                if received_data.trim() == "200:" {
                    //sender_to_ger_client.send(cmdtotcp.to_string()).unwrap();
                    stream.write_all("#> ".as_bytes()).unwrap();
                    continue;
                }
                if received_data.contains("202:") {
                    if received_data.trim().len() < 8 {
                        log::info!("Backdoor: You forgot client ID\n");
                        stream.write_all("#> ".as_bytes()).unwrap();
                        continue;
                    }
                    let client_id = &received_data[4..8];
                    if client_id.is_empty() {
                        log::info!("Backdoor: Invalid client ID\n");
                    }else{
                        let mut u16client_id: u16 = 0;
                        match client_id.parse::<u16>() {
                            Ok(valor) => {
                                u16client_id = valor;
                                println!("Conversão bem-sucedida: {}", u16client_id);
                            }
                            Err(e) => println!("Falha na conversão: {}", e),
                            
                        }
                        let shutdown: String = "999: shutdown".to_string();
                        println!("Convertido: {}", u16client_id);
                        clientdata::ClientData::send_client_msg_by_id(u16client_id,shutdown);
                        //thread::sleep(Duration::from_millis(5000));
                        //process::exit(0);
                        stream.write_all("#> ".as_bytes()).unwrap();
                    }
                    continue;
                }
                if received_data.trim() == "201:" {
                    let shutdown: String = "999: shutdown".to_string();
                    clientdata::ClientData::round_robin(shutdown);
                    //thread::sleep(Duration::from_millis(5000));
                    //process::exit(0);
                    stream.write_all("#> ".as_bytes()).unwrap();
                    continue;
                }// Handle the LISTAR command
                if received_data.trim() == "L" || received_data.trim() == "l" {
                    let clients = clientdata::ClientData::list_clients();
                    if clients.is_empty() {
                        stream.write("No clients connected\n".as_bytes()).unwrap();
                        stream.write_all("#> ".as_bytes()).unwrap();
                        continue;
                    }   
                    for client in clients {
                        let one_client = format!("Client ID: {} Client IP: {} Client Status: {} Client Port: {} Client CID: {}\n", client.id, client.ip, client.status, client.port, client.cid);
                        stream.write(one_client.as_bytes()).unwrap();
                    }                                 
                    stream.write_all("#> ".as_bytes()).unwrap();
                }else if received_data.trim() == "E" || received_data.trim() == "E" {
                    log::info!("Backdoor: E cmd exiting...");               
                    stream.write_all("#> ".as_bytes()).unwrap();
                    break
                }else{                
                    stream.write("Invalid command\n".as_bytes()).unwrap();
                    stream.write_all("#> ".as_bytes()).unwrap();
                }               
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Não há dados disponíveis no momento
                if start_time.elapsed() >= timeout1 {
                    // Timeout atingido
                   // log::error!("Read timed out");
                    //break; //nao quero sair do loop
                }
                // Espera um pouco antes de tentar novamente
                //std::thread::sleep(Duration::from_millis(100));
            }
                Err(e) => {
                // Outro erro ocorreu
                log::error!("Read error: {}", e);
                break;
            }
            }
            if elapsed.as_millis() > timerovrflow {
                timerovrflow = timeout + elapsed.as_millis();
                log::info!("ger_client:  Timeout, closing connection");
                let keep_alive: String = "100: keep alive".to_string();
                clientdata::ClientData::round_robin(keep_alive);
            }else{
                let keep_alive: String = "1".to_string();
                //clientdata::ClientData::round_robin(keep_alive);
            }
        }
    }
}


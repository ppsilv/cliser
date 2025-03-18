use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use regex::Regex;
use std::thread;

// Struct para armazenar os dados do cliente
#[derive(Debug, Clone)]
struct Client {
    id: u32,
    ip: String,
    status: String,
    port: u16,
    cid: u32,
}

// Função para fazer o parsing da string e criar um objeto Client
fn parse_client_data(data: &str) -> Option<Client> {
    let re = Regex::new(r"Client ID: (\d+) Client IP: ([0-9.]+) Client Status: (\w+) Client Port: (\d+) Client CID: (\d+)").unwrap();
    if let Some(caps) = re.captures(data) {
        // Validação do Client ID e CID
        let id = match caps[1].parse::<u32>() {
            Ok(id) => id,
            Err(_) => return None, // Retorna None se o parsing falhar
        };
        let ip = caps[2].to_string();
        let status = caps[3].to_string();

        // Validação do Client Port (u16)
        let port = match caps[4].parse::<u16>() {
            Ok(port) => port,
            Err(_) => return None, // Retorna None se o parsing falhar
        };

        // Validação do Client CID
        let cid = match caps[5].parse::<u32>() {
            Ok(cid) => cid,
            Err(_) => return None, // Retorna None se o parsing falhar
        };

        Some(Client {
            id,
            ip,
            status,
            port,
            cid,
        })
    } else {
        None
    }
}

// Função para lidar com a conexão do cliente
fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<HashMap<u32, Client>>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = String::new();

    println!("New connection: {}", stream.peer_addr().unwrap());

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                // Conexão fechada pelo cliente
                println!("Client disconnected: {}", stream.peer_addr().unwrap());
                break;
            }
            Ok(_) => {
                let data = buffer.trim(); // Remove espaços em branco e novas linhas
                println!("Received command: {}", data);

                // Processa o comando recebido
                let response = process_command(data, &clients);
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

// Função para processar os comandos recebidos
fn process_command(command: &str, clients: &Arc<Mutex<HashMap<u32, Client>>>) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();

    // Imprime o primeiro slice (parte) do comando
    if let Some(first_part) = parts.first() {
        println!("First slice: {}", first_part);
    }

    match parts.as_slice() {
        ["LIST"] => {
            let clients = clients.lock().unwrap();
            if clients.is_empty() {
                "No clients registered.\n".to_string()
            } else {
                let mut response = String::new();
                for client in clients.values() {
                    response.push_str(&format!("{:?}\n", client));
                }
                response
            }
        }
        ["SEARCH", id] => {
            let clients = clients.lock().unwrap();
            if let Ok(id) = id.parse::<u32>() {
                if let Some(client) = clients.get(&id) {
                    format!("{:?}\n", client)
                } else {
                    format!("Client ID: {} not found.\n", id)
                }
            } else {
                "Invalid Client ID.\n".to_string()
            }
        }
        ["ADD", ..] => {
            if let Some(client) = parse_client_data(command) {
                let mut clients = clients.lock().unwrap();
                if clients.contains_key(&client.id) {
                    format!("Client ID: {} already exists.\n", client.id)
                } else {
                    clients.insert(client.id, client.clone());
                    format!("Client ID: {} added successfully.\n", client.id)
                }
            } else {
                "Failed to parse client data. Check the format and values.\n".to_string()
            }
        }
        ["REMOVE", id] => {
            let mut clients = clients.lock().unwrap();
            if let Ok(id) = id.parse::<u32>() {
                if clients.remove(&id).is_some() {
                    format!("Client ID: {} removed successfully.\n", id)
                } else {
                    format!("Client ID: {} not found.\n", id)
                }
            } else {
                "Invalid Client ID.\n".to_string()
            }
        }
        ["UPDATE", id, ..] => {
            if let Ok(id) = id.parse::<u32>() {
                if let Some(new_client) = parse_client_data(command) {
                    let mut clients = clients.lock().unwrap();
                    if clients.contains_key(&id) {
                        clients.insert(id, new_client);
                        format!("Client ID: {} updated successfully.\n", id)
                    } else {
                        format!("Client ID: {} not found.\n", id)
                    }
                } else {
                    "Failed to parse client data. Check the format and values.\n".to_string()
                }
            } else {
                "Invalid Client ID.\n".to_string()
            }
        }
        _ => "Invalid command. Use LIST, SEARCH <ID>, ADD <data>, REMOVE <ID>, or UPDATE <ID> <data>.\n".to_string(),
    }
}

// Função principal
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878...");

    // Usamos um HashMap protegido por Mutex para armazenar os clientes
    let clients = Arc::new(Mutex::new(HashMap::<u32, Client>::new()));

    for stream in listener.incoming() {
        let clients = Arc::clone(&clients);
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream, clients);
                });
            }
            Err(e) => {
                println!("Failed to establish connection: {}", e);
            }
        }
    }
}
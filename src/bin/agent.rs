use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use regex::Regex;
use std::thread;
use serde::{Deserialize, Serialize};  // <-- Esta linha está faltando!

// Struct para armazenar os dados do cliente
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Client {
    id: u32,
    ip: String,
    status: String,
    port: u16,
    cid: u32,
}

#[derive(Debug, Deserialize, Serialize)]
enum Command {
    List,
    Search { id: u32 },
    Add { client: Client },
    Remove { id: u32 },
    Update { id: u32, client: Client },
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<Client>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<Client>,
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

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<HashMap<u32, Client>>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = String::new();

    println!("New connection: {}", stream.peer_addr().unwrap());

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected: {}", stream.peer_addr().unwrap());
                break;
            }
            Ok(_) => {
                let response = match serde_json::from_str(&buffer) {
                    Ok(command) => process_command(command, &clients),
                    Err(e) => Response {
                        success: false,
                        message: format!("Invalid JSON: {}", e),
                        data: None,
                        client: None,
                    },
                };

                let response_json = serde_json::to_string(&response).unwrap();
                stream.write_all(response_json.as_bytes()).unwrap();
                stream.write_all(b"\n").unwrap(); // Delimitador de fim de mensagem
            }
            Err(e) => {
                println!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

// Função para processar os comandos recebidos
fn process_command(command: Command, clients: &Arc<Mutex<HashMap<u32, Client>>>) -> Response {
    match command {
        Command::List => {
            let clients_map = clients.lock().unwrap();
            let clients_list: Vec<Client> = clients_map.values().cloned().collect();
            Response {
                success: true,
                message: "OK".to_string(),
                data: Some(clients_list),
                client: None,
            }
        }
        Command::Search { id } => {
            let clients_map = clients.lock().unwrap();
            match clients_map.get(&id) {
                Some(client) => Response {
                    success: true,
                    message: "Client found".to_string(),
                    data: None,
                    client: Some(client.clone()),
                },
                None => Response {
                    success: false,
                    message: format!("Client ID {} not found", id),
                    data: None,
                    client: None,
                },
            }
        }
        Command::Add { client } => {
            let mut clients_map = clients.lock().unwrap();
            if clients_map.contains_key(&client.id) {
                Response {
                    success: false,
                    message: format!("Client ID {} already exists", client.id),
                    data: None,
                    client: None,
                }
            } else {
                clients_map.insert(client.id, client.clone());
                Response {
                    success: true,
                    message: format!("Client ID {} added", client.id),
                    data: None,
                    client: None,
                }
            }
        }
        Command::Remove { id } => {
            let mut clients_map = clients.lock().unwrap();
            if clients_map.remove(&id).is_some() {
                Response {
                    success: true,
                    message: format!("Client ID {} removed", id),
                    data: None,
                    client: None,
                }
            } else {
                Response {
                    success: false,
                    message: format!("Client ID {} not found", id),
                    data: None,
                    client: None,
                }
            }
        }
        Command::Update { id, client } => {
            let mut clients_map = clients.lock().unwrap();
            if clients_map.contains_key(&id) {
                clients_map.insert(id, client.clone());
                Response {
                    success: true,
                    message: format!("Client ID {} updated", id),
                    data: None,
                    client: None,
                }
            } else {
                Response {
                    success: false,
                    message: format!("Client ID {} not found", id),
                    data: None,
                    client: None,
                }
            }
        }
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
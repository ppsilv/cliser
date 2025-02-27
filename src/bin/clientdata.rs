use std::sync::{mpsc, MutexGuard};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::collections::HashMap;

// Estrutura para os dados do cliente (sem o receiver)
#[derive(Debug, Clone)]
pub struct ClientData {
    pub id: u16,
    pub ip: String,
    pub status: String, // "active" or "inactive"
    pub port: String, // Porta do cliente
    pub cid: String, // ID do cliente
    pub sender_tcp_writer: mpsc::Sender<String>, // Sender para enviar mensagens ao cliente
}

// Singleton para armazenar os clientes
lazy_static! {
    static ref CLIENTS: Arc<Mutex<HashMap<u16, Arc<Mutex<ClientData>>>>> = Arc::new(Mutex::new(HashMap::new()));
}

impl ClientData {
    //use super::*;

    // Salva ou atualiza um cliente
    pub fn save(client: ClientData) {
        let mut clients = CLIENTS.lock().unwrap();
        clients.insert(client.id, Arc::new(Mutex::new(client)));
    }
/*
    Ao invéz de usar o status para usar um cadastro ja existente para cadastrar
    um novo cliente eu preferi deletar o registro antigo e criar um novo registro.
    Se algum dia precisar de um histórico de clientes, eu posso criar um campo
    para armazenar o histórico de clientes.
    Se precisar do status eu reativo essa função.
    // Atualiza o status de um cliente por ID
    pub fn update_status(id: u16, new_status: String) {
        let clients = CLIENTS.lock().unwrap();
        if let Some(client) = clients.get(&id) {
            let mut client = client.lock().unwrap();
            client.status = new_status;
        }
    }
*/
    // Lê o ID de um cliente por ID
    pub fn read_id(id: u16) -> Option<u16> {
        let clients = CLIENTS.lock().unwrap();
        clients.get(&id).map(|client| client.lock().unwrap().id)
    }

    // Lê o IP de um cliente por ID
    pub fn read_ip(id: u16) -> Option<String> {
        let clients = CLIENTS.lock().unwrap();
        clients.get(&id).map(|client| client.lock().unwrap().ip.clone())
    }

    // Lê o status de um cliente por ID
    pub fn read_status(id: u16) -> Option<String> {
        let clients = CLIENTS.lock().unwrap();
        clients.get(&id).map(|client| client.lock().unwrap().status.clone())
    }

    // Lê o CID de um cliente por ID
    pub fn read_cid(id: u16) -> Option<String> {
        let clients = CLIENTS.lock().unwrap();
        clients.get(&id).map(|client| client.lock().unwrap().cid.clone())
    }

    // Lista todos os clientes (apenas informações, sem canais de comunicação)
    pub fn list_clients() -> Vec<ClientData> {
        let clients = CLIENTS.lock().unwrap();
        clients.values()
            .map(|client| {
                let client = client.lock().unwrap();
                ClientData {
                    id: client.id,
                    ip: client.ip.clone(),
                    status: client.status.clone(),
                    port: client.port.clone(),
                    cid: client.cid.clone(),
                    sender_tcp_writer: client.sender_tcp_writer.clone(),
                }
            })
            .collect()
    }

    // Encontra um cliente por ID
    pub fn find_client_by_id(id: u16) -> Option<ClientData> {
        let clients = CLIENTS.lock().unwrap();
        clients.get(&id).map(|client| {
            let client = client.lock().unwrap();
            ClientData {
                id: client.id,
                ip: client.ip.clone(),
                status: client.status.clone(),
                port: client.port.clone(),
                cid: client.cid.clone(),
                sender_tcp_writer: client.sender_tcp_writer.clone(),
            }
        })
    }
    pub fn send_client_msg_by_id(id: u16, message: String) {
        let clients = CLIENTS.lock().unwrap();
        clients.get(&id).map(|client| {
            let client = client.lock().unwrap();
            let result = client.sender_tcp_writer.send(message.clone())
                .map_err(|e| format!("Failed to send message to client {}: {}", client.id, e));
        });
    }

    // Lê o sender_tcp_writer por ID
    pub fn read_sender_tcp_writer(id: u16) -> Option<mpsc::Sender<String>> {
        let clients = CLIENTS.lock().unwrap();
        clients.get(&id).map(|client| client.lock().unwrap().sender_tcp_writer.clone())
    }

    // Envia uma mensagem para todos os clientes (round-robin)
    pub fn round_robin(message: String) -> Vec<Result<(), String>> {
        let clients = CLIENTS.lock().unwrap();
        let mut results = Vec::new();

        // Itera sobre todos os clientes
        for client in clients.values() {
            let client = client.lock().unwrap();
            // Envia a mensagem para o cliente
            let result = client.sender_tcp_writer.send(message.clone())
                .map_err(|e| format!("Failed to send message to client {}: {}", client.id, e));
            results.push(result);
        }
        results
    }

    pub fn delete_client_by_id( id: u16) -> bool {
        let mut clients = CLIENTS.lock().unwrap();
        clients.remove(&id).is_some() // Retorna true se o cliente foi deletado
    }    
}
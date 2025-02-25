use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::client;

#[derive(Debug, Clone)]
pub struct ClientData {
    pub id: u16,
    pub ip: String,
    pub status: String, // "active" or "inactive"
    pub port: String, // Port of client
    pub cid: String,
}

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<u16, ClientData>> = Mutex::new(HashMap::new());
}

pub fn save_client(client: ClientData) {
    let mut clients = CLIENTS.lock().unwrap();
    clients.insert(client.id, client);
}

pub fn find_client_by_id(id: u16) -> Option<ClientData> {
    let clients = CLIENTS.lock().unwrap();
    clients.get(&id).cloned()
}

pub fn update_status_by_id(id: u16, new_status: String) -> bool {
    let mut clients = CLIENTS.lock().unwrap();
    if let Some(client) = clients.get_mut(&id) {
        client.status = new_status;
        true
    } else {
        false
    }
}

pub fn update_cid_by_id(id: u16, new_cid: String) -> bool {
    let mut clients = CLIENTS.lock().unwrap();
    if let Some(client) = clients.get_mut(&id) {
        client.cid = new_cid;
        true
    } else {
        false
    }
}

pub fn list_all_clients() {
    let clients = CLIENTS.lock().unwrap();
    let all_clients: Vec<ClientData>  = clients.values().cloned().collect();
    if clients.is_empty() {
        log::info!("No clients connected.");
    }
    for client in all_clients {
        println!("Client: {:?}", client);
    } 
}
pub fn list_all_clients2()->String {
    let clients = CLIENTS.lock().unwrap();
    let all_clients: Vec<ClientData>  = clients.values().cloned().collect();
    let mut result = String::from("Connected clients:\n");
    if clients.is_empty() {
        log::info!("No clients connected.");
        result = String::from("Connected clients: No clients connected.\n");
    }

    for client in clients.iter() {        
        result.push_str(&format!("ID: {}, IP: {}, Status: {} Port2: {} cid {}\n", client.1.id, client.1.ip, client.1.status, client.1.port, client.1.cid));
    }
    result
}

fn _testemain() {
    // Example usage
    let client1 = ClientData {
        id: 1,
        ip: "192.168.1.1".to_string(),
        status: "active".to_string(),
        port: "8080".to_string(),
        cid: "cid1".to_string(),
    };

    let client2 = ClientData {
        id: 2,
        ip: "192.168.1.2".to_string(),
        status: "inactive".to_string(),
        port: "8081".to_string(),
        cid: "cid2".to_string(),
    };

    save_client(client1);
    save_client(client2);

    if let Some(client) = find_client_by_id(1) {
        println!("Found client: {:?}", client);
    }

    update_status_by_id(1, "inactive".to_string());
    update_cid_by_id(1, "new_cid1".to_string());

    if let Some(client) = find_client_by_id(1) {
        println!("Updated client: {:?}", client);
    }
}
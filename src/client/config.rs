use serde::Deserialize;
use std::env;
use std::fs::{self, File};
use lazy_static::lazy_static;
use std::io::{self, Read};
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    server: ServerConfig,
}
#[derive(Debug, Deserialize, Clone)]
struct ServerConfig {
    clientid: String,
    password: String,
    host: String,
    port1: String,
    port2: String,
}
fn read_file() -> io::Result<String> {
    let mut file = File::open("configcli.json")?; // Use `?` to propagate the error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
// Define a global static variable for the configuration
lazy_static! {
    static ref CONFIG: Config = {
        // Read the configuration file
        //let config_file = fs::read_to_string("configcli.json").unwrap();
        
        let contents = match read_file() {
            Ok(contents) => {
                println!("File contents: {}", contents);
                contents
            },
            Err(e) => {
                eprintln!("Failed to read file: {}", e);
                String::new()
            },
        };


        // Parse the JSON into the Config struct
        serde_json::from_str(&contents).unwrap()
    };
}

// Function to access the configuration
pub fn get_configuration() -> &'static Config {
    &CONFIG
}

pub fn print_path() {
    println!("Current directory: {:?}", env::current_dir().unwrap());
    let config_file = fs::read_to_string("config.json").expect("Failed to read config.json. Please ensure the file exists.");
    println!("Config file content: {}", config_file);
}

pub fn get_clientid() ->String{
    self::CONFIG.server.clientid.clone()
}

pub fn get_password() ->String{
    self::CONFIG.server.password.clone()
}

pub fn get_hostip() ->String{
    CONFIG.server.host.clone()
}

pub fn get_port1() ->String{
    CONFIG.server.port1.clone()
}

pub fn get_port2() ->String{
    CONFIG.server.port2.clone()
}


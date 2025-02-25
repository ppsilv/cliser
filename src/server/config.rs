use serde::Deserialize;
use std::env;
use std::fs;
use lazy_static::lazy_static;

   
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    server: ServerConfig,
}

#[derive(Debug, Deserialize, Clone)]
struct ServerConfig {
    password: String,
    host: String,
    port1: String,
    port2: String,
    port3: String
}

// Define a global static variable for the configuration
lazy_static! {
    static ref CONFIG: Config = {
        // Read the configuration file
        let config_file = fs::read_to_string("serv_config.json").unwrap();
        // Parse the JSON into the Config struct
        serde_json::from_str(&config_file).unwrap()
    };
}

// Function to access the configuration
pub fn get_configuration() -> &'static Config {
    &CONFIG
}

/*
pub fn get_configuration() ->Config { //} Result<(), Box<dyn std::error::Error>> {
    // Read the JSON file
    let config_file = fs::read_to_string("serv_config.json").unwrap();

    // Deserialize the JSON into the Config struct
    let config: Config = serde_json::from_str(&config_file).unwrap();
   // println!("Configuration: {:?}", config);
    config
}
*/

pub fn print_path() {
    println!("Current directory: {:?}", env::current_dir().unwrap());
    let config_file = fs::read_to_string("config.json").expect("Failed to read config.json. Please ensure the file exists.");
    println!("Config file content: {}", config_file);
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

pub fn get_port3() ->String{
    CONFIG.server.port3.clone()
}
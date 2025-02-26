use serde::Deserialize;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use lazy_static::lazy_static;

   
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
//    port2: String,
//    port3: String
}
/*
fn get_binary_dir() -> io::Result<PathBuf> {
    let exe_path = env::current_exe()?; // Get the path of the binary
    Ok(exe_path.parent().unwrap().to_path_buf()) // Get the directory
}

// Define a global static variable for the configuration
lazy_static! {
    
    static ref CONFIG: Config = {
        let binary_dir = get_binary_dir().unwrap();
        let config_path = binary_dir.join("configcli.json"); // Construct the full path
        // Read the configuration file
        println!("Config file path: {:?}", config_path);
        let config_file = fs::read_to_string(config_path).unwrap();
        // Parse the JSON into the Config struct
        serde_json::from_str(&config_file).unwrap()
    };
}
*/
fn get_binary_dir() -> io::Result<PathBuf> {
    let exe_path = env::current_exe()?; // Get the path of the binary
    println!("Binary path: {:?}", exe_path); // Debug print
    Ok(exe_path.parent().unwrap().to_path_buf()) // Get the directory
}

lazy_static! {
    static ref CONFIG: Config = {
        let binary_dir = get_binary_dir().unwrap();
        let config_path = binary_dir.join("configcli.json"); // Construct the full path
        println!("Config file path: {:?}", config_path); // Debug print
        let config_file = fs::read_to_string(config_path).unwrap();
        serde_json::from_str(&config_file).unwrap()
    };
}

/*
// Function to access the configuration
pub fn get_configuration() -> &'static Config {
    &CONFIG
}

pub fn print_path() {
    println!("Current directory: {:?}", env::current_dir().unwrap());
//    let config_file = fs::read_to_string("config.json").expect("Failed to read config.json. Please ensure the file exists.");
//    println!("Config file content: {}", config_file);
}
*/
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
/* 
pub fn get_port2() ->String{
    CONFIG.server.port2.clone()
}

pub fn get_port3() ->String{
    CONFIG.server.port3.clone()
}
*/
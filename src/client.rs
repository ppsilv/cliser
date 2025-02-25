use std::io::{self, Write, Read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::net::{TcpStream, Shutdown};
//use serde::Deserialize;
pub mod config;
use config::Config;

static mut CTRL_SIGNAL: u8 = 0;

pub fn client() {
   
    let password = config::get_password() ;
    // Create a TCP stream
    let mut stream = match TcpStream::connect(format!("{}:{}", config::get_hostip(), config::get_port1())) {
        Ok(stream) => stream,
        Err(e) => {
            log::error!("Failed to connect: {}", e);
            return;
        }
    };
}

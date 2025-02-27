use std::io::{self, Write, Read};
use syslog::{Facility, BasicLogger, Formatter3164};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::net::{TcpStream, Shutdown};
use std::process;
//use serde::Deserialize;
mod configcli;


static mut CTRL_SIGNAL: u8 = 0;

pub fn main() -> io::Result<()> {

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "cliser".into(),
        pid: std::process::id() as u32,
    };
    // Initialize the logger
    let logger = syslog::unix(formatter).expect("could not connect to syslog");

    // Set the logger as the global logger
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .expect("could not set logger");

    // Set up the Ctrl+C handler
    ctrlc::set_handler(move || {
        log::info!("Client: Ctrl+C received! Shutting down...");
        r.store(false, Ordering::SeqCst); // Set the flag to false
        unsafe{
            CTRL_SIGNAL = 255;
        }
    }).expect("Error setting Ctrl+C handler");

    // Connect to the server
  
    let hostip_port1: String = configcli::get_hostip(  )+":"+ &configcli::get_port1(  ) ;
    log::info!("Connected to server at {}",hostip_port1);
    let mut stream = TcpStream::connect(hostip_port1)?;

    let mut buffer = [0; 512];

    // Read the server's prompt for the password
    log::info!("Client: Reading the server's prompt for the password");
    let n = stream.read(&mut buffer)?;
    let prompt = String::from_utf8_lossy(&buffer[..n]);

    // If the password was incorrect, exit
    if prompt.contains("110: qual a senha") {
        // Send the password to the server
        let password =  configcli::get_password(  );
        //io::stdin().read_line(&mut password)?;
        stream.write_all(password.trim().as_bytes())?;       
        log::info!("Client: Password sent.");
    }

    // Read the server's response to the password
    log::info!("Client: Reading the server's response to the id");
    let n = stream.read(&mut buffer)?;
    let response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..n]);

    if response.contains("120: qual o ID") {
        // Send the password to the server
        let clientid =  configcli::get_clientid(  );
        stream.write_all(clientid.trim().as_bytes())?;       
        log::info!("Client: clientid sent.");
    }

    // If the password was incorrect, exit
    if response.contains("Invalid password") {
        return Ok(());
    }

    // Read the server's response
    log::info!("Client: Reading the server's Connected message");
    let n = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    //stream.shutdown(Shutdown::Both)?;

    if response.contains("140: Connected") {
        log::info!("Client: The server autorized my connection");
    }else{
        log::info!("Client: Erro verificando se connected");
        return Ok(());
    }

    
    loop{        
        //TODO: 🌞 Put a timeout if sever does not send a message in time, shutdown client.
        let n = stream.read(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        log::info!("Client:{} msg {}",configcli::get_clientid(  ),response);
        unsafe{
            if CTRL_SIGNAL == 255 {
                stream.shutdown(Shutdown::Both).unwrap();
                log::info!("Client: Finishing the client Id: {}",configcli::get_clientid(  )); 
            }
        }
        if response.contains("999:") {
            log::info!("Client:{} msg {}",configcli::get_clientid(  ),response);
            stream.shutdown(Shutdown::Both).unwrap();
            process::exit(0);
        }
        if response.contains("100:") {
            log::info!("Client:{} msg {}",configcli::get_clientid(  ),response);
        }
        if response.contains("110:") {
            log::info!("Client:{} msg {}",configcli::get_clientid(  ),response);
            stream.write_all(configcli::get_clientid(  ).trim().as_bytes())?;       
            log::info!("Client: id sent.");            
        }
    }    
    //Ok(())
}
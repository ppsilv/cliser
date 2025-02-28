use std::fs;
use std::io::Error;
use csv::Writer;
use reqwest::blocking::Client; // Agora funcionará corretamente⏎
use serde_json;
use std::fs::OpenOptions;
use chrono;



///    Busca os sensores de temperatura conectados ao Raspberry Pi.
///    Os sensores de temperatura são conectados ao Raspberry Pi através do barramento 1-Wire.
///    O barramento 1-Wire é um barramento serial que permite a comunicação de dados com dispositivos
///    utilizando apenas um fio de dados.
///    Os sensores de temperatura DS18B20 são dispositivos que utilizam o barramento 1-Wire para
///    medir a temperatura ambiente.
///    Cada sensor DS18B20 possui um identificador único de 64 bits que é utilizado para identificar
///    o sensor no barramento 1-Wire.
///    O identificador do sensor é utilizado para acessar os dados de temperatura do sensor.
///    O identificador do sensor é armazenado em um arquivo no diretório /sys/bus/w1/devices.
///    O identificador do sensor é um diretório que contém um arquivo chamado w1_slave que contém os
///    dados de temperatura do sensor.
///    O arquivo w1_slave contém os dados de temperatura em graus Celsius.


// Função que retorna uma lista com os identificadores dos sensores de temperatura conectados ao Raspberry Pi.
pub fn get_sensor_ids() -> Result<Vec<String>, Error> {
    let dir = "/sys/bus/w1/devices";
    let mut sensor_ids = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            if dir_name.starts_with("28") {
                sensor_ids.push(path.to_str().unwrap().to_string());
            }
        }
    }

    if sensor_ids.is_empty() {
        Err(Error::new(std::io::ErrorKind::NotFound, "No sensors found"))
    } else {
        Ok(sensor_ids)
    }
}

// Função que retorna o identificador do primeiro sensor de temperatura conectado ao Raspberry Pi.
pub fn get_sensor_id() -> Result<String, Error> {
    let dir = "/sys/bus/w1/devices";
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() { // Verifica se é um diretório
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            if dir_name.starts_with("28") { // Verifica se o nome começa com "28"
                return Ok(path.to_str().unwrap().to_string());
            }
        }
    }
    Err(Error::new(std::io::ErrorKind::NotFound, "Sensor not found"))
}

// Função que lê a temperatura do sensor de temperatura com o identificador especificado.    
fn read_temperature(sensor_id: &str) -> Result<f32, Error> {
    let temp_file = format!("{}/w1_slave", sensor_id);
    let data = fs::read_to_string(temp_file)?;
    let parts: Vec<&str> = data.split("t=").collect();
    if parts.len() >= 2 {
        let raw_temp = parts[1].trim().parse::<i32>().unwrap();
        Ok(raw_temp as f32 / 1000.0) // Converte para Celsius
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid temperature data"))
    }
}

// Função que registra a temperatura do sensor de temperatura com o identificador especificado em um arquivo CSV.
fn _log_temperature(sensor_id: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let temperature = read_temperature(sensor_id)?;

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&[chrono::Local::now().to_rfc3339(), temperature.to_string()])?;
    wtr.flush()?;

    println!("Temperature logged: {:.2}°C", temperature);
    Ok(())
}

pub fn send_temperature_to_server(sensor_id: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let temperature = read_temperature(sensor_id)?;

    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json") // Adiciona o cabeçalho JSON
        .body(serde_json::to_string(&serde_json::json!({ "temperature": temperature }))?) // Converte o JSON para string
        .send()?;

    if response.status().is_success() {
        println!("Temperature sent to server: {:.2}°C", temperature);
    } else {
        eprintln!("Failed to send temperature to server");
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let _sensor_id = match get_sensor_id() {
        Ok(sensor_id) => {
            println!("Sensor ID: {}", sensor_id);
            let temperature = read_temperature(&sensor_id)?;
            println!("Temperature: {:.2}°C", temperature);
            sensor_id
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e);
        },
    };
    Ok(())
}
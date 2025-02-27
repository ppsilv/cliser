/*
To read a DS18B20 temperature sensor from a Raspberry Pi 4 using Rust, you need to access 
the underlying Linux system's One-Wire interface through the /sys/bus/w1/devices directory, 
read the sensor's unique device ID, and then read the temperature data from the corresponding 
file within that directory using the std::fs module in Rust. 
Key steps:

    Hardware Setup:
        Connect the DS18B20 sensor to a GPIO pin on the Raspberry Pi, ensuring you have a 
        4.7kΩ pull-up resistor between the data pin and the 3.3V power supply.
        Enable the One-Wire kernel module on your Raspberry Pi. 

Explanation:

    get_sensor_id function:
        Iterates through files in the /sys/bus/w1/devices directory.
        Checks if each file name starts with "28" (a pattern for DS18B20 sensors).
        Returns the full path of the sensor file if found. 
    read_temperature function:
        Constructs the path to the sensor's "w1_slave" file.
        Reads the file content.
        Parses the data to extract the raw temperature value.
        Converts the raw value to Celsius and returns it. 

Important Considerations:

    Permissions: Ensure your Rust program has the necessary permissions to access the /sys/bus/w1/devices directory. 

Error Handling: Always handle potential errors during file operations. 
Kernel Module: Make sure the One-Wire kernel module is loaded on your Raspberry Pi. 
Sensor Connection: Ensure the DS18B20 sensor is properly connected to the Raspberry Pi.     
*/
use std::fs;
use std::io::Error;


pub fn get_sensor_id() -> Result<String, Error> {
    let dir = "/sys/bus/w1/devices"; 
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.to_str().unwrap().contains("28") { // Check for DS18B20 pattern
            return Ok(path.to_str().unwrap().to_string());
        }
    }
    Err(Error::new(std::io::ErrorKind::NotFound, "Sensor not found"))
}

fn read_temperature(sensor_id: &str) -> Result<f32, Error> {
    let temp_file = format!("{}/w1_slave", sensor_id);
    let data = fs::read_to_string(temp_file)?;
    let parts: Vec<&str> = data.split("t=").collect();
    if parts.len() >= 2 {
        let raw_temp = parts[1].parse::<i32>().unwrap();
        Ok(raw_temp as f32 / 1000.0) // Convert to Celsius
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid temperature data"))
    }
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
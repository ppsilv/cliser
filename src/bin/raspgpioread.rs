mod gpio;
use gpio::GpioReader;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the GPIO reader for pin 17 (BCM numbering)
    let gpio_reader = GpioReader::new(17)?;

    // Read the GPIO pin state
    let pin_state = gpio_reader.read();
    println!("GPIO pin state: {:?}", pin_state);

    Ok(())
}
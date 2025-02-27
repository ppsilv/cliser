use rppal::gpio::{Gpio, InputPin, Level};
use std::error::Error;

pub struct GpioReader {
    pin: InputPin,
}

impl GpioReader {
    // Initialize the GPIO pin
    pub fn new(pin_number: u8) -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(pin_number)?.into_input();
        Ok(GpioReader { pin })
    }

    // Read the current state of the GPIO pin
    pub fn read(&self) -> Level {
        self.pin.read()
    }
}
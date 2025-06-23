use crate::notifier::{Notifier, NotifierError};
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct RaspberryPiNotifier {}

impl RaspberryPiNotifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Notifier for RaspberryPiNotifier {
    fn sign(&self, _: String) -> Result<(), NotifierError> {
        // Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
        const GPIO_LED: u8 = 23;
        println!(
            "Blinking an LED on a {}.",
            DeviceInfo::new().unwrap().model()
        );

        let mut pin = Gpio::new().unwrap().get(GPIO_LED).unwrap().into_output();
        thread::spawn(|| {
            for i in 1..10 {
                // Blink the LED by setting the pin's logic level high for 500 ms.
                pin.set_high();
                thread::sleep(Duration::from_millis(500));
                pin.set_low();
            }
        });
        Ok(())
    }
}

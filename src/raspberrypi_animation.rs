use crate::animation::Animation;
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct RaspberryPiAnimation {}

impl RaspberryPiAnimation {
    pub fn new() -> Self {
        info!(
            "Blinking an LED on a {}.",
            DeviceInfo::new().unwrap().model()
        );
        Self {}
    }
}

impl Animation for RaspberryPiAnimation {
    fn animate(&self) {
        // Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
        const GPIO_LED: u8 = 23;

        let mut pin = Gpio::new().unwrap().get(GPIO_LED).unwrap().into_output();
        thread::spawn(move || {
            for i in 1..20 {
                // Blink the LED by setting the pin's logic level high for 500 ms.
                pin.set_high();
                thread::sleep(Duration::from_millis(500));
                pin.set_low();
            }
        });
    }
}

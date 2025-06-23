use std::thread;
use std::time::Duration;
#[cfg(target_arch = "arm")]
use rppal::gpio::Gpio;
#[cfg(target_arch = "arm")]
use rppal::system::DeviceInfo;
use crate::notifier::{Notifier, NotifierError};

#[cfg(target_arch = "arm")]
#[derive(Debug)]
pub struct RaspberryPiNotifier {}

impl RaspberryPiNotifier {
    pub fn new() -> Self {
        RaspberryPiNotifier {}
    }
}

#[cfg(target_arch = "arm")]
impl Notifier for RaspberryPiNotifier {
    fn sign(&self, _: String) -> Result<(), NotifierError> {
        println!("RaspberryPiNotifier::sign");

        // Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
        const GPIO_LED: u8 = 23;
        println!("Blinking an LED on a {}.", DeviceInfo::new().unwrap().model());

        let mut pin = Gpio::new().unwrap().get(GPIO_LED).unwrap().into_output();

        // Blink the LED by setting the pin's logic level high for 500 ms.
        pin.set_high();
        thread::sleep(Duration::from_secs(5));
        pin.set_low();

        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(500));
            }
        });
        Ok(())
    }
}

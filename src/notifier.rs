use std::error::Error;
use std::fmt::Debug;
use std::thread;
use std::time::Duration;
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

pub trait Notifier: Send + Sync + Debug + 'static {
    fn sign(&self, event: String) -> Result<(), dyn Error>;
}

#[derive(Debug)]
pub struct EmptyNotifier {}

impl EmptyNotifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Notifier for EmptyNotifier {
    fn sign(&self, event: String) -> Result<(), dyn Error> {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(500));
        }
        
        Ok(())
    }
}

#[cfg(target_arch = "arm")]
#[derive(Debug)]
pub struct RaspberryPiNotifier {}

#[cfg(target_arch = "arm")]
impl RaspberryPiNotifier {
    pub fn new() -> Self {
        RaspberryPiNotifier {}
    }
}

#[cfg(target_arch = "arm")]
impl Notifier for RaspberryPiNotifier {
    fn sign(&self, _: String) -> Result<(), dyn Error> {
        println!("RaspberryPiNotifier::sign");

        // Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
        const GPIO_LED: u8 = 23;
        println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

        let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

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

use std::fmt::Debug;
use std::thread;
use std::time::Duration;

pub trait Notifier: Send + Sync + Debug + 'static {
    fn sign(&self, event: String);
}

#[derive(Debug)]
pub struct EmptyNotifier {}

impl EmptyNotifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Notifier for EmptyNotifier {
    fn sign(&self, event: String) {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(500));
        }
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
    fn sign(&self, _: String) {
        println!("RaspberryPiNotifier::sign");
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(500));
            }
        });
    }
}

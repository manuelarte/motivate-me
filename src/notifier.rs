use std::fmt::Debug;
use std::thread;
use std::time::Duration;

pub trait Notifier: Send + Sync + Debug + 'static {
    fn sign(&self, event: String) -> Result<(), NotifierError>;
}

#[derive(Debug)]
pub struct EmptyNotifier {}

impl EmptyNotifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Notifier for EmptyNotifier {
    fn sign(&self, event: String) -> Result<(), NotifierError> {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(500));
        }

        Ok(())
    }
}

pub enum NotifierError {
    Failed,
}

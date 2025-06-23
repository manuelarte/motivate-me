use std::fmt::Debug;

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
    fn sign(&self, _: String) -> Result<(), NotifierError> {
        Ok(())
    }
}

#[derive(Debug)]
pub enum NotifierError {
    Failed,
}

#[cfg(target_arch = "aarch64")]
use crate::raspberrypi_animation::RaspberryPiAnimation;
use std::fmt::Debug;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::info;

pub trait Animation: Send + Sync + Debug {
    fn animate(&self);
}

#[derive(Clone, Debug)]
pub struct MockAnimation {}

impl Animation for MockAnimation {
    fn animate(&self) {
        info!("MockAnimation animate");
        thread::sleep(Duration::from_millis(500));
        info!("MockAnimation finished");
    }
}

pub fn get_animation(environment: &str) -> Arc<dyn Animation> {
    match environment {
        #[cfg(target_arch = "aarch64")]
        "production" => Arc::new(RaspberryPiAnimation::new()),
        _ => Arc::new(MockAnimation {}),
    }
}

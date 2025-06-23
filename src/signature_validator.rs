use std::fmt::Debug;
pub trait SignatureValidator: Send + Sync + Debug + 'static {
    fn validate(&self, secret: &str) -> bool;
}

#[derive(Debug, Clone)]
pub struct AlwaysTrueValidator {}

impl AlwaysTrueValidator {
    pub fn new() -> AlwaysTrueValidator {
        AlwaysTrueValidator {}
    }
}

impl SignatureValidator for AlwaysTrueValidator {
    fn validate(&self, _: &str) -> bool {
        true
    }
}

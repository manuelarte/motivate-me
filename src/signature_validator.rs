use crate::AppConfig;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fmt::Debug;
use std::sync::Arc;

pub trait SignatureValidator: Send + Sync + Debug + 'static {
    fn validate(&self, payload: &[u8], expected_signature: &str) -> bool;
}

#[derive(Debug, Clone)]
pub struct AlwaysTrueValidator {}

impl AlwaysTrueValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl SignatureValidator for AlwaysTrueValidator {
    fn validate(&self, _: &[u8], _: &str) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct Rsa256SignatureValidator {
    secret: String,
}

impl Rsa256SignatureValidator {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
}

impl SignatureValidator for Rsa256SignatureValidator {
    fn validate(&self, payload: &[u8], expected_signature: &str) -> bool {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes()).unwrap();
        mac.update(payload);

        let result = mac.finalize();
        let expected = result.into_bytes();
        let expected_hex = hex::encode(expected);
        expected_hex.eq(expected_signature)
    }
}

pub fn get_signature_validator(cfg: &AppConfig) -> Arc<dyn SignatureValidator> {
    match cfg.environment.as_str() {
        "production" => Arc::new(AlwaysTrueValidator::new()),
        _ => Arc::new(Rsa256SignatureValidator::new(cfg.secret.as_str())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_test() {
        let signature_validator = Rsa256SignatureValidator::new("It's a Secret to Everybody");
        let expected_signature = "757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17";
        let result = signature_validator.validate("Hello, World!".as_bytes(), expected_signature);
        assert_eq!(result, true);
    }
}

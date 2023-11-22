use regex::Regex;

use crate::{MpesaError, MpesaResult};

pub trait PhoneNumberValidator {
    fn validate(&self) -> MpesaResult<()>;
}

impl PhoneNumberValidator for &str {
    fn validate(&self) -> MpesaResult<()> {
        let phone_regex =
            Regex::new(r"^(254\d{9}|07\d{8}|011\d{7}|7\d{8}|1\d{8})$").map_err(|_| {
                MpesaError::Message(
                "Invalid phone number, must be in the format 2547XXXXXXXX, 07XXXXXXXX, 011XXXXXXX",
            )
            })?;

        if phone_regex.is_match(self) {
            Ok(())
        } else {
            Err(MpesaError::Message(
                "Invalid phone number, must be in the format 2547XXXXXXXX, 07XXXXXXXX, 011XXXXXXX",
            ))
        }
    }
}

impl PhoneNumberValidator for String {
    fn validate(&self) -> MpesaResult<()> {
        self.as_str().validate()
    }
}

impl PhoneNumberValidator for u64 {
    fn validate(&self) -> MpesaResult<()> {
        self.to_string().validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_phone() {
        assert!("254712345678".validate().is_ok());
        assert!("254012345678".validate().is_ok());
        assert!("0712345678".validate().is_ok());
        assert!("712345678".validate().is_ok());
        assert!("112345678".validate().is_ok());
        assert!("0112345678".validate().is_ok());
        assert!("07987654321".validate().is_err());
        assert!("011987654321".validate().is_err());
        assert!("254712345678900".validate().is_err());
        assert!("25471234567".validate().is_err());
        assert!("2547".validate().is_err());
        assert!("2547a".validate().is_err());
        assert!("254".validate().is_err());
        assert!("254a".validate().is_err());
        assert!("25".validate().is_err());
        assert!("25a".validate().is_err());
        assert!("2".validate().is_err());
        assert!("2a".validate().is_err());
        assert!("".validate().is_err());
        assert!("a".validate().is_err());
    }

    #[test]
    fn test_validate_phone_string() {
        assert!("254712345678".to_string().validate().is_ok());
        assert!("254012345678".to_string().validate().is_ok());
        assert!("254712345678900".to_string().validate().is_err());
        assert!("25471234567".to_string().validate().is_err());
        assert!("2547".to_string().validate().is_err());
        assert!("2547a".to_string().validate().is_err());
        assert!("254".to_string().validate().is_err());
        assert!("254a".to_string().validate().is_err());
        assert!("25".to_string().validate().is_err());
        assert!("25a".to_string().validate().is_err());
        assert!("2".to_string().validate().is_err());
        assert!("2a".to_string().validate().is_err());
        assert!("".to_string().validate().is_err());
        assert!("a".to_string().validate().is_err());
    }

    #[test]
    fn test_validate_phone_u64() {
        assert!(254712345678u64.validate().is_ok());
        assert!(254012345678u64.validate().is_ok());
        assert!(712345678u64.validate().is_ok());
        assert!(112345678u64.validate().is_ok());
        assert!(254712345678900u64.validate().is_err());
        assert!(25471234567u64.validate().is_err());
        assert!(2547u64.validate().is_err());
        assert!(2547u64.validate().is_err());
        assert!(254u64.validate().is_err());
        assert!(254u64.validate().is_err());
        assert!(25u64.validate().is_err());
        assert!(25u64.validate().is_err());
        assert!(2u64.validate().is_err());
        assert!(2u64.validate().is_err());
        assert!(0u64.validate().is_err());
    }
}

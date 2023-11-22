use crate::{MpesaError, MpesaResult};

pub trait PhoneNumberValidator {
    fn validate(&self) -> MpesaResult<()>;
}

impl PhoneNumberValidator for &str {
    fn validate(&self) -> MpesaResult<()> {
        if self.starts_with("254")
            && self.len() == 12
            && self.chars().skip(3).all(|c| c.is_ascii_digit())
        {
            Ok(())
        } else {
            Err(MpesaError::Message(
                "Invalid phone number, must be in the format 2547XXXXXXXX",
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

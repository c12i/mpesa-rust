//!# MPESA Environment
//!
//! Code related to setting up the desired Safaricom API environment. Environment can be either
//! sandbox or production.
//! you will need environment specific credentials (`CLIENT_KEY` AND `CLIENT_SECRET`) when creating
//! an instance of the `Mpesa` client struct. Note that you cannot use sandbox credentials in
//! production and vice versa.
//!
//! Based on selected environment. You are able to access environment specific data such as the `base_url`
//! and the `public key` an X509 certificate used for encrypting initiator passwords. You can read more about that from
//! the Safaricom API [docs](https://developer.safaricom.co.ke/docs?javascript#security-credentials).

use std::convert::TryFrom;
use std::str::FromStr;

use crate::MpesaError;

#[derive(Debug, Clone)]
/// Enum to map to desired environment so as to access certificate
/// and the base url
/// Required to construct a new `Mpesa` struct
pub enum Environment {
    /// Production environment
    Production,
    /// Sandbox environment: for testing and development purposes
    Sandbox,
}

/// Expected behavior of an `Mpesa` client environment
/// This abstraction exists to make it possible to mock the MPESA api server for tests
pub trait ApiEnvironment: Clone {
    fn base_url(&self) -> &str;
    fn get_certificate(&self) -> &str;
}

impl FromStr for Environment {
    type Err = MpesaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl TryFrom<&str> for Environment {
    type Error = MpesaError;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let v = v.to_lowercase();
        match v.as_str() {
            "production" => Ok(Self::Production),
            "sandbox" => Ok(Self::Sandbox),
            _ => Err(MpesaError::Message(
                "Could not parse the provided environment name",
            )),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = MpesaError;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        v.as_str().try_into()
    }
}

impl ApiEnvironment for Environment {
    /// Matches to base_url based on `Environment` variant
    fn base_url(&self) -> &str {
        match self {
            Environment::Production => "https://api.safaricom.co.ke",
            Environment::Sandbox => "https://sandbox.safaricom.co.ke",
        }
    }

    /// Match to X509 public key certificate based on `Environment`
    fn get_certificate(&self) -> &str {
        match self {
            Environment::Production => include_str!("./certificates/production"),
            Environment::Sandbox => include_str!("./certificates/sandbox"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn test_valid_string_is_parsed_as_environment() {
        let accepted_production_values =
            vec!["production", "Production", "PRODUCTION", "prODUctIoN"];
        let accepted_sandbox_values = vec!["sandbox", "Sandbox", "SANDBOX", "sanDBoX"];
        accepted_production_values.into_iter().for_each(|v| {
            let environment: Environment = v.parse().unwrap();
            assert_eq!(environment.base_url(), "https://api.safaricom.co.ke");
            assert_eq!(
                environment.get_certificate(),
                include_str!("./certificates/production")
            )
        });
        accepted_sandbox_values.into_iter().for_each(|v| {
            let environment: Environment = v.try_into().unwrap();
            assert_eq!(environment.base_url(), "https://sandbox.safaricom.co.ke");
            assert_eq!(
                environment.get_certificate(),
                include_str!("./certificates/sandbox")
            )
        })
    }

    #[test]
    #[should_panic]
    fn test_invalid_string_panics() {
        let _: Environment = "foo_bar".try_into().unwrap();
    }
}

use failure_derive::*;
use std::env::VarError;

#[derive(Debug, Fail)]
/// Mpesa error stack
pub enum MpesaError {
    #[fail(display = "Error Authenticating: {}", 0)]
    AuthenticationError(serde_json::Value),
    #[fail(display = "Error performing B2B transaction: {}", 0)]
    B2bError(serde_json::Value),
    #[fail(display = "Error performing B2C transaction: {}", 0)]
    B2cError(serde_json::Value),
    #[fail(display = "Error performing C2B registration: {}", 0)]
    C2bRegisterError(serde_json::Value),
    #[fail(display = "Error performing C2B simulation: {}", 0)]
    C2bSimulateError(serde_json::Value),
    #[fail(display = "Error getting account balance: {}", 0)]
    AccountBalanceError(serde_json::Value),
    #[fail(display = "Error making mpesa express request: {}", 0)]
    MpesaExpressRequestError(serde_json::Value),
    #[fail(display = "Network Error: {}", 0)]
    NetworkError(reqwest::Error),
    #[fail(display = "Error parsing JSON data: {}", 0)]
    ParseError(serde_json::Error),
    #[fail(display = "Error getting environmental variables: {}", 0)]
    EnvironmentalVariableError(VarError),
    #[fail(display = "Error extracting X509 from pem: {}", 0)]
    EncryptionError(openssl::error::ErrorStack),
    #[fail(display = "Error: {}", 0)]
    Message(&'static str),
    #[fail(display = "Error: {:#?}", 0)]
    ErrorResponse(serde_json::Value),
}

impl From<serde_json::Error> for MpesaError {
    fn from(e: serde_json::Error) -> Self {
        MpesaError::ParseError(e)
    }
}

impl From<reqwest::Error> for MpesaError {
    fn from(e: reqwest::Error) -> Self {
        MpesaError::NetworkError(e)
    }
}

impl From<VarError> for MpesaError {
    fn from(e: VarError) -> Self {
        MpesaError::EnvironmentalVariableError(e)
    }
}

impl From<&'static str> for MpesaError {
    fn from(e: &'static str) -> Self {
        MpesaError::Message(e)
    }
}

impl From<openssl::error::ErrorStack> for MpesaError {
    fn from(e: openssl::error::ErrorStack) -> Self {
        MpesaError::EncryptionError(e)
    }
}

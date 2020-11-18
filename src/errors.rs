use failure_derive::*;
use reqwest;
use serde_json;
use std::env::VarError;

#[derive(Debug, Fail)]
/// Mpesa error stack
pub enum MpesaError {
    #[fail(display = "Error Authenticating: {}", 0)]
    AuthenticationError(&'static str),
    #[fail(display = "Error performing B2B transaction: {}", 0)]
    B2BError(&'static str),
    #[fail(display = "Error performing B2C transaction: {}", 0)]
    B2CError(&'static str),
    #[fail(display = "Error performing C2B simulation: {}", 0)]
    C2BSimulateError(&'static str),
    #[fail(display = "Error getting account balance: {}", 0)]
    AccountBalanceError(&'static str),
    #[fail(display = "Network Error: {}", 0)]
    NetworkError(reqwest::Error),
    #[fail(display = "Error parsing JSON data: {}", 0)]
    ParseError(serde_json::Error),
    #[fail(display = "Error getting environmental variables: {}", 0)]
    EnvironmentalVariableError(VarError),
    #[fail(display = "Error: {}", 0)]
    Message(&'static str),
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

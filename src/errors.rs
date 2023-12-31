use std::env::VarError;
use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Mpesa error stack
#[derive(Error, Debug)]
pub enum MpesaError {
    #[error("Service error: {0}")]
    Service(ResponseError),
    #[error("An error has occured while performing the http request")]
    NetworkError(#[from] reqwest::Error),
    #[error("An error has occured while serializing/ deserializing")]
    ParseError(#[from] serde_json::Error),
    #[error("An error has occured while retrieving an environmental variable")]
    EnvironmentalVariableError(#[from] VarError),
    #[error("An error has occurred while generating security credentials")]
    EncryptionError(#[from] openssl::error::ErrorStack),
    #[error("{0}")]
    Message(&'static str),
    #[error("An error has occurred while building the request: {0}")]
    BuilderError(BuilderError),
}

/// `Result` enum type alias
pub type MpesaResult<T> = Result<T, MpesaError>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ResponseError {
    pub request_id: String,
    pub error_code: String,
    pub error_message: String,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "requestID: {}, errorCode:{}, errorMessage:{}",
            self.request_id, self.error_code, self.error_message
        )
    }
}

#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("Field [{0}] is required")]
    UninitializedField(&'static str),
    #[error("Field [{0}] is invalid")]
    ValidationError(String),
}

impl From<String> for BuilderError {
    fn from(s: String) -> Self {
        Self::ValidationError(s)
    }
}

impl From<derive_builder::UninitializedFieldError> for MpesaError {
    fn from(e: derive_builder::UninitializedFieldError) -> Self {
        Self::BuilderError(BuilderError::UninitializedField(e.field_name()))
    }
}

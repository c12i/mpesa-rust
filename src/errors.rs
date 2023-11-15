use std::env::VarError;
use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Mpesa error stack
#[derive(Error, Debug)]
pub enum MpesaError {
    #[error("{0}")]
    AuthenticationError(ApiError),
    #[error("B2B request failed: {0}")]
    B2bError(ApiError),
    #[error("B2C request failed: {0}")]
    B2cError(ApiError),
    #[error("C2B register request failed: {0}")]
    C2bRegisterError(ApiError),
    #[error("C2B simulate request failed: {0}")]
    C2bSimulateError(ApiError),
    #[error("Account Balance request failed: {0}")]
    AccountBalanceError(ApiError),
    #[error("Bill manager onboarding failed: {0}")]
    OnboardError(ApiError),
    #[error("Bill manager onboarding modify failed: {0}")]
    OnboardModifyError(ApiError),
    #[error("Bill manager bulk invoice failed: {0}")]
    BulkInvoiceError(ApiError),
    #[error("Bill manager reconciliation failed: {0}")]
    ReconciliationError(ApiError),
    #[error("Bill manager single invoice failed: {0}")]
    SingleInvoiceError(ApiError),
    #[error("Bill manager cancel invoice failed: {0}")]
    CancelInvoiceError(ApiError),
    #[error("Mpesa Express request/ STK push failed: {0}")]
    MpesaExpressRequestError(ApiError),
    #[error("Mpesa Transaction reversal failed: {0}")]
    MpesaTransactionReversalError(ApiError),
    #[error("Mpesa Transaction status failed: {0}")]
    MpesaTransactionStatusError(ApiError),
    #[error("Mpesa Dynamic QR failed: {0}")]
    MpesaDynamicQrError(ApiError),
    #[error("An error has occured while performing the http request")]
    NetworkError(#[from] reqwest::Error),
    #[error("An error has occured while serializig/ deserializing")]
    ParseError(#[from] serde_json::Error),
    #[error("An error has occured while retreiving an environmental variable")]
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
pub struct ApiError {
    pub request_id: String,
    pub error_code: String,
    pub error_message: String,
}

impl fmt::Display for ApiError {
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

impl From<url::ParseError> for MpesaError {
    fn from(e: url::ParseError) -> Self {
        Self::BuilderError(BuilderError::ValidationError(e.to_string()))
    }
}

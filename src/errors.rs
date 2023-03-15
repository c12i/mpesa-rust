use std::env::VarError;

/// Mpesa error stack
#[derive(thiserror::Error, Debug)]
pub enum MpesaError {
    #[error("Authentication request failed: {0}")]
    AuthenticationError(serde_json::Value),
    #[error("B2B request failed: {0}")]
    B2bError(serde_json::Value),
    #[error("B2C request failed: {0}")]
    B2cError(serde_json::Value),
    #[error("C2B register request failed: {0}")]
    C2bRegisterError(serde_json::Value),
    #[error("C2B simulate request failed: {0}")]
    C2bSimulateError(serde_json::Value),
    #[error("Account Balance request failed: {0}")]
    AccountBalanceError(serde_json::Value),
    #[error("Bill manager onboarding failed: {0}")]
    BillManagerOnboardError(serde_json::Value),
    #[error("Mpesa Express request/ STK push failed: {0}")]
    MpesaExpressRequestError(serde_json::Value),
    #[error("Mpesa Transaction reversal failed: {0}")]
    MpesaTransactionReversalError(serde_json::Value),
    #[error("Mpesa Transaction status failed: {0}")]
    MpesaTransactionStatusError(serde_json::Value),
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
}

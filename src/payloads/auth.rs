use serde::Deserialize;

#[derive(Debug,Deserialize)]
/// Response from calling the Safaricom OAuth endpoint
pub struct AuthResponse {
    pub access_token: String,
    expires_in: String,
}
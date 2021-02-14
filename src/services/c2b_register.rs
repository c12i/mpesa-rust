use crate::client::{Mpesa, MpesaResult};
use crate::constants::ResponseType;
use crate::errors::MpesaError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
/// Payload to register the 3rd party’s confirmation and validation URLs to M-Pesa
struct C2bRegisterPayload<'a> {
    ValidationURL: &'a str,
    ConfirmationURL: &'a str,
    ResponseType: ResponseType,
    ShortCode: &'a str,
}

#[derive(Debug, Deserialize, Clone)]
pub struct C2bRegisterResponse {
    ConversationID: String,
    OriginatorCoversationID: String,
    ResponseDescription: String,
}

#[allow(dead_code)]
impl<'a> C2bRegisterResponse {
    pub fn conversation_id(&'a self) -> &'a String {
        &self.ConversationID
    }

    pub fn originator_conversation_id(&'a self) -> &'a String {
        &self.OriginatorCoversationID
    }

    pub fn response_description(&'a self) -> &'a String {
        &self.ResponseDescription
    }
}

#[derive(Debug)]
/// C2B Register builder
pub struct C2bRegisterBuilder<'a> {
    client: &'a Mpesa,
    validation_url: Option<&'a str>,
    confirmation_url: Option<&'a str>,
    response_type: Option<ResponseType>,
    short_code: Option<&'a str>,
}

impl<'a> C2bRegisterBuilder<'a> {
    /// Creates a new C2B Builder
    pub fn new(client: &'a Mpesa) -> C2bRegisterBuilder<'a> {
        C2bRegisterBuilder {
            client,
            validation_url: None,
            confirmation_url: None,
            response_type: None,
            short_code: None,
        }
    }

    /// Adds `ValidationURL` for the client. This is a required field
    ///
    /// # Error
    /// If `ValidationURL` is invalid or not provided
    pub fn validation_url(mut self, validation_url: &'a str) -> C2bRegisterBuilder<'a> {
        self.validation_url = Some(validation_url);
        self
    }

    /// Adds `ConfirmationUrl` for the client. This is a required field
    ///
    /// # Error
    /// If `ConfirmationUrl` is invalid or not provided
    pub fn confirmation_url(mut self, confirmation_url: &'a str) -> C2bRegisterBuilder<'a> {
        self.confirmation_url = Some(confirmation_url);
        self
    }

    /// Adds `ResponseType` for timeout. Will default to `ResponseType::Complete` if not explicitly provided
    pub fn response_type(mut self, response_type: ResponseType) -> C2bRegisterBuilder<'a> {
        self.response_type = Some(response_type);
        self
    }

    /// Adds `ShortCode` for the organization. This is a required field.
    ///
    /// # Error
    /// If `ShortCode` is invalid
    pub fn short_code(mut self, short_code: &'a str) -> C2bRegisterBuilder<'a> {
        self.short_code = Some(short_code);
        self
    }

    /// **C2B Register API**
    ///
    /// Registers the the 3rd party’s confirmation and validation URLs to M-Pesa
    ///
    /// Registering maps these URLs to the 3rd party shortcode.
    /// Whenever M-Pesa receives a transaction on the shortcode,
    /// M-Pesa triggers a validation request against the validation URL and
    /// the 3rd party system responds to M-Pesa with a validation response (either a success or an error code).
    /// See more [here](https://developer.safaricom.co.ke/docs?shell#c2b-api)
    ///
    /// The response expected is the success code the 3rd party
    ///
    /// A successful request returns a `serde_json::Value` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub fn send(self) -> MpesaResult<C2bRegisterResponse> {
        let url = format!(
            "{}/mpesa/c2b/v1/registerurl",
            self.client.environment().base_url()
        );

        let payload = C2bRegisterPayload {
            ValidationURL: self.validation_url.unwrap_or("None"),
            ConfirmationURL: self.confirmation_url.unwrap_or("None"),
            ResponseType: self.response_type.unwrap_or(ResponseType::Complete),
            ShortCode: self.short_code.unwrap_or("None"),
        };

        let response = Client::new()
            .post(&url)
            .bearer_auth(self.client.auth()?)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            let value: C2bRegisterResponse = response.json()?;
            return Ok(value);
        }

        let value: Value = response.json()?;
        Err(MpesaError::C2bRegisterError(value))
    }
}

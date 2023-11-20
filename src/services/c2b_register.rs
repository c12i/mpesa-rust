use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::constants::ResponseType;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

const C2B_REGISTER_URL: &str = "mpesa/c2b/v1/registerurl";

#[derive(Debug, Serialize)]
/// Payload to register the 3rd party’s confirmation and validation URLs to M-Pesa
struct C2bRegisterPayload<'mpesa> {
    #[serde(rename(serialize = "ValidationURL"))]
    validation_url: &'mpesa str,
    #[serde(rename(serialize = "ConfirmationURL"))]
    confirmation_url: &'mpesa str,
    #[serde(rename(serialize = "ResponseType"))]
    response_type: ResponseType,
    #[serde(rename(serialize = "ShortCode"))]
    short_code: &'mpesa str,
}

#[derive(Debug, Deserialize, Clone)]
pub struct C2bRegisterResponse {
    #[serde(rename(deserialize = "OriginatorCoversationID"))]
    pub originator_conversation_id: String,
    #[serde(rename(deserialize = "ResponseCode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    pub response_description: String,
}

#[derive(Debug)]
/// C2B Register builder
pub struct C2bRegisterBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    validation_url: Option<&'mpesa str>,
    confirmation_url: Option<&'mpesa str>,
    response_type: Option<ResponseType>,
    short_code: Option<&'mpesa str>,
}

impl<'mpesa, Env: ApiEnvironment> C2bRegisterBuilder<'mpesa, Env> {
    /// Creates a new C2B Builder
    pub fn new(client: &'mpesa Mpesa<Env>) -> C2bRegisterBuilder<'mpesa, Env> {
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
    pub fn validation_url(
        mut self,
        validation_url: &'mpesa str,
    ) -> C2bRegisterBuilder<'mpesa, Env> {
        self.validation_url = Some(validation_url);
        self
    }

    /// Adds `ConfirmationUrl` for the client. This is a required field
    ///
    /// # Error
    /// If `ConfirmationUrl` is invalid or not provided
    pub fn confirmation_url(
        mut self,
        confirmation_url: &'mpesa str,
    ) -> C2bRegisterBuilder<'mpesa, Env> {
        self.confirmation_url = Some(confirmation_url);
        self
    }

    /// Adds `ResponseType` for timeout. Will default to `ResponseType::Complete` if not explicitly provided
    pub fn response_type(mut self, response_type: ResponseType) -> C2bRegisterBuilder<'mpesa, Env> {
        self.response_type = Some(response_type);
        self
    }

    /// Adds `ShortCode` for the organization. This is a required field.
    ///
    /// # Error
    /// If `ShortCode` is invalid or not provided
    pub fn short_code(mut self, short_code: &'mpesa str) -> C2bRegisterBuilder<'mpesa, Env> {
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
    /// A successful request returns a `C2bRegisterResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure

    pub async fn send(self) -> MpesaResult<C2bRegisterResponse> {
        let payload = C2bRegisterPayload {
            validation_url: self
                .validation_url
                .ok_or(MpesaError::Message("validation_url is required"))?,
            confirmation_url: self
                .confirmation_url
                .ok_or(MpesaError::Message("confirmation_url is required"))?,
            response_type: self.response_type.unwrap_or(ResponseType::Completed),
            short_code: self
                .short_code
                .ok_or(MpesaError::Message("short_code is required"))?,
        };

        self.client
            .send(crate::client::Request {
                method: reqwest::Method::POST,
                path: C2B_REGISTER_URL,
                body: payload,
            })
            .await
    }
}

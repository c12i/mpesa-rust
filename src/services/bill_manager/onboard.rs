use serde::{Deserialize, Serialize};

use crate::client::{Mpesa, MpesaResult};
use crate::constants::SendRemindersTypes;
use crate::environment::ApiEnvironment;
use crate::errors::MpesaError;

#[derive(Debug, Serialize)]
/// Payload to opt you in as a biller to the bill manager features.
struct OnboardPayload<'mpesa> {
    #[serde(rename(serialize = "callbackUrl"))]
    callback_url: &'mpesa str,
    #[serde(rename(serialize = "email"))]
    email: &'mpesa str,
    #[serde(rename(serialize = "logo"))]
    logo: &'mpesa str,
    #[serde(rename(serialize = "officialContact"))]
    official_contact: &'mpesa str,
    #[serde(rename(serialize = "sendReminders"))]
    send_reminders: SendRemindersTypes,
    #[serde(rename(serialize = "shortcode"))]
    short_code: &'mpesa str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OnboardResponse {
    #[serde(rename(deserialize = "app_key"))]
    pub app_key: String,
    #[serde(rename(deserialize = "rescode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub response_message: String,
}

#[derive(Debug)]
pub struct OnboardBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    callback_url: Option<&'mpesa str>,
    email: Option<&'mpesa str>,
    logo: Option<&'mpesa str>,
    official_contact: Option<&'mpesa str>,
    send_reminders: Option<SendRemindersTypes>,
    short_code: Option<&'mpesa str>,
}

impl<'mpesa, Env: ApiEnvironment> OnboardBuilder<'mpesa, Env> {
    /// Creates a new Bill Manager Onboard builder
    pub fn new(client: &'mpesa Mpesa<Env>) -> OnboardBuilder<'mpesa, Env> {
        OnboardBuilder {
            client,
            callback_url: None,
            email: None,
            logo: None,
            official_contact: None,
            send_reminders: None,
            short_code: None,
        }
    }

    /// Adds `callbackUrl`.
    ///
    /// # Errors
    /// If 'callbackUrl` is not provided.
    pub fn callback_url(mut self, callback_url: &'mpesa str) -> OnboardBuilder<'mpesa, Env> {
        self.callback_url = Some(callback_url);
        self
    }

    /// Adds an `email` address to the request.
    ///
    /// # Errors
    /// If `email` is not provided.
    pub fn email(mut self, email: &'mpesa str) -> OnboardBuilder<'mpesa, Env> {
        self.email = Some(email);
        self
    }

    /// Adds `logo`; a file with your organizions's logo.
    ///
    /// # Errors
    /// If `logo` is not provided.
    pub fn logo(mut self, logo: &'mpesa str) -> OnboardBuilder<'mpesa, Env> {
        self.logo = Some(logo);
        self
    }

    /// Adds `officialContact` to the request; must be in the format `07XXXXXXXX`
    ///
    /// # Errors
    /// If `officialContact` is invalid or not provided.
    pub fn official_contact(
        mut self,
        official_contact: &'mpesa str,
    ) -> OnboardBuilder<'mpesa, Env> {
        self.official_contact = Some(official_contact);
        self
    }

    /// Adds `sendReminders`. Defaults to `SendRemindersTypes::Disable` if no value is explicitely passed.
    ///
    /// # Errors
    /// If `sendReminders` is not valid.
    pub fn send_reminders(
        mut self,
        send_reminders: SendRemindersTypes,
    ) -> OnboardBuilder<'mpesa, Env> {
        self.send_reminders = Some(send_reminders);
        self
    }

    /// Adds `ShortCode`; the 6 digit MPESA Till Number or PayBill Number
    ///
    /// # Errors
    /// If Till or PayBill number is invalid or not provided
    pub fn short_code(mut self, short_code: &'mpesa str) -> OnboardBuilder<'mpesa, Env> {
        self.short_code = Some(short_code);
        self
    }

    /// # Bill Manager Onboarding API
    ///
    /// Opt in as a biller to mpesa's bill manager features.
    ///
    /// A successful request returns a `OnboardResponse` type
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<OnboardResponse> {
        let url = format!(
            "{}/v1/billmanager-invoice/optin",
            self.client.environment.base_url()
        );

        let payload = OnboardPayload {
            callback_url: self
                .callback_url
                .ok_or(MpesaError::Message("callback_url is required"))?,
            email: self.email.ok_or(MpesaError::Message("email is required"))?,
            logo: self.logo.ok_or(MpesaError::Message("logo is required"))?,
            official_contact: self
                .official_contact
                .ok_or(MpesaError::Message("official_contact is required"))?,
            send_reminders: self
                .send_reminders
                .unwrap_or_else(|| SendRemindersTypes::Disable),
            short_code: self
                .short_code
                .ok_or(MpesaError::Message("short_code is required"))?,
        };

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json().await?;
            return Ok(value);
        }

        let value = response.json().await?;
        Err(MpesaError::OnboardError(value))
    }
}

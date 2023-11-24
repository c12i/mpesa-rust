#![doc = include_str!("../../../docs/client/bill_manager/onboard_modify.md")]

use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::constants::SendRemindersTypes;
use crate::environment::ApiEnvironment;
use crate::errors::MpesaResult;

const BILL_MANAGER_ONBOARD_MODIFY_API_URL: &str = "v1/billmanager-invoice/change-optin-details";

#[derive(Debug, Serialize)]
/// Payload to modify opt-in details to the bill manager api.
struct OnboardModifyPayload<'mpesa> {
    #[serde(
        rename(serialize = "callbackUrl"),
        skip_serializing_if = "Option::is_none"
    )]
    callback_url: Option<&'mpesa str>,
    #[serde(rename(serialize = "email"), skip_serializing_if = "Option::is_none")]
    email: Option<&'mpesa str>,
    #[serde(rename(serialize = "logo"), skip_serializing_if = "Option::is_none")]
    logo: Option<&'mpesa str>,
    #[serde(
        rename(serialize = "officialContact"),
        skip_serializing_if = "Option::is_none"
    )]
    official_contact: Option<&'mpesa str>,
    #[serde(
        rename(serialize = "sendReminders"),
        skip_serializing_if = "Option::is_none"
    )]
    send_reminders: Option<SendRemindersTypes>,
    #[serde(
        rename(serialize = "shortcode"),
        skip_serializing_if = "Option::is_none"
    )]
    short_code: Option<&'mpesa str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OnboardModifyResponse {
    #[serde(rename(deserialize = "rescode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub response_message: String,
}

#[derive(Debug)]
pub struct OnboardModifyBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    callback_url: Option<&'mpesa str>,
    email: Option<&'mpesa str>,
    logo: Option<&'mpesa str>,
    official_contact: Option<&'mpesa str>,
    send_reminders: Option<SendRemindersTypes>,
    short_code: Option<&'mpesa str>,
}

impl<'mpesa, Env: ApiEnvironment> OnboardModifyBuilder<'mpesa, Env> {
    /// Creates a new Bill Manager Onboard Modify builder
    pub fn new(client: &'mpesa Mpesa<Env>) -> OnboardModifyBuilder<'mpesa, Env> {
        OnboardModifyBuilder {
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
    pub fn callback_url(mut self, callback_url: &'mpesa str) -> OnboardModifyBuilder<'mpesa, Env> {
        self.callback_url = Some(callback_url);
        self
    }

    /// Adds an `email` address to the request.
    pub fn email(mut self, email: &'mpesa str) -> OnboardModifyBuilder<'mpesa, Env> {
        self.email = Some(email);
        self
    }

    /// Adds `logo`; a file with your organizions's logo.
    pub fn logo(mut self, logo: &'mpesa str) -> OnboardModifyBuilder<'mpesa, Env> {
        self.logo = Some(logo);
        self
    }

    /// Adds `officialContact` to the request; must be in the format `07XXXXXXXX`
    pub fn official_contact(
        mut self,
        official_contact: &'mpesa str,
    ) -> OnboardModifyBuilder<'mpesa, Env> {
        self.official_contact = Some(official_contact);
        self
    }

    /// Adds `sendReminders`.
    pub fn send_reminders(
        mut self,
        send_reminders: SendRemindersTypes,
    ) -> OnboardModifyBuilder<'mpesa, Env> {
        self.send_reminders = Some(send_reminders);
        self
    }

    /// Adds `ShortCode`; the 6 digit MPESA Till Number or PayBill Number
    pub fn short_code(mut self, short_code: &'mpesa str) -> OnboardModifyBuilder<'mpesa, Env> {
        self.short_code = Some(short_code);
        self
    }

    /// # Bill Manager Onboarding Modify API
    ///
    /// Modifies opt-in details to the bill manager api.
    ///
    /// A successful request returns a `OnboardModifyResponse` type
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<OnboardModifyResponse> {
        let payload = OnboardModifyPayload {
            callback_url: self.callback_url,
            email: self.email,
            logo: self.logo,
            official_contact: self.official_contact,
            send_reminders: self.send_reminders,
            short_code: self.short_code,
        };

        self.client
            .send(crate::client::Request {
                method: reqwest::Method::POST,
                path: BILL_MANAGER_ONBOARD_MODIFY_API_URL,
                body: payload,
            })
            .await
    }
}

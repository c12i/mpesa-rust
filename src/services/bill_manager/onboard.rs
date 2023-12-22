use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::constants::SendRemindersTypes;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

#[derive(Debug, Serialize)]
/// Payload to opt you in as a biller to the bill manager features.
pub struct OnboardRequest<'mpesa> {
    /// Callback url that will be invoked by our payments API in order to
    /// push payments done to your paybill.
    #[serde(rename(serialize = "callbackUrl"))]
    callback_url: &'mpesa str,

    /// Official contact email address for the organization signing up to
    /// bill manager.
    #[serde(rename(serialize = "email"))]
    email: &'mpesa str,

    /// Image to be embedded in the invoices and receipts sent to your customer.
    #[serde(rename(serialize = "logo"))]
    logo: &'mpesa str,

    /// Official contact phone number will appear in features sent to the customer such
    /// as invoices and payment receipts for customers to reach out to you as a business.
    #[serde(rename(serialize = "officialContact"))]
    official_contact: &'mpesa str,

    /// Allows you to enable or disable sms payment reminders for invoices sent.
    #[serde(rename(serialize = "sendReminders"))]
    send_reminders: SendRemindersTypes,

    /// A shortcode (5 to 6 digit account number) used to identify the organization
    /// and receive the transaction.
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

#[derive(Builder, Clone, Debug)]
#[builder(build_fn(error = "MpesaError"))]
pub struct Onboard<'mpesa, Env: ApiEnvironment> {
    #[builder(pattern = "immutable", private)]
    client: &'mpesa Mpesa<Env>,

    /// Callback url that will be invoked by our payments API in order to
    /// push payments done to your paybill.
    #[builder(setter(into))]
    callback_url: &'mpesa str,

    /// Official contact email address for the organization signing up to
    /// bill manager.
    #[builder(setter(into))]
    email: &'mpesa str,

    /// Image to be embedded in the invoices and receipts sent to your customer.
    #[builder(setter(into))]
    logo: &'mpesa str,

    /// Official contact phone number will appear in features sent to the customer such
    /// as invoices and payment receipts for customers to reach out to you as a business.
    #[builder(setter(into))]
    official_contact: &'mpesa str,

    /// Allows you to enable or disable sms payment reminders for invoices sent.
    #[builder(default = "SendRemindersTypes::Disable", setter(into), try_setter)]
    send_reminders: SendRemindersTypes,

    /// A shortcode (5 to 6 digit account number) used to identify the organization
    /// and receive the transaction.
    #[builder(setter(into))]
    short_code: &'mpesa str,
}

impl<'mpesa, Env: ApiEnvironment> From<Onboard<'mpesa, Env>> for OnboardRequest<'mpesa> {
    fn from(builder: Onboard<'mpesa, Env>) -> Self {
        OnboardRequest {
            callback_url: builder.callback_url,
            email: builder.email,
            logo: builder.logo,
            official_contact: builder.official_contact,
            send_reminders: builder.send_reminders,
            short_code: builder.short_code,
        }
    }
}

impl<'mpesa, Env: ApiEnvironment> Onboard<'mpesa, Env> {
    pub(crate) fn builder(client: &'mpesa Mpesa<Env>) -> OnboardBuilder<'mpesa, Env> {
        OnboardBuilder::default().client(client)
    }

    /// Builds Onboard
    ///
    /// Returns an `Onboard` which can be used to send a request.
    pub fn from_request(client: &'mpesa Mpesa<Env>, request: OnboardRequest<'mpesa>) -> Self {
        Onboard {
            client,
            callback_url: request.callback_url,
            email: request.email,
            logo: request.logo,
            official_contact: request.official_contact,
            send_reminders: request.send_reminders,
            short_code: request.short_code,
        }
    }

    /// # Bill Manager Onboarding API
    ///
    /// Opt in as a biller to mpesa's bill manager features.
    ///
    /// A successful request returns a `OnboardResponse` type
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<OnboardResponse> {
        let url = format!(
            "{}/v1/billmanager-invoice/optin",
            self.client.environment.base_url()
        );

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json::<OnboardRequest>(&self.into())
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

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::constants::SendRemindersTypes;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

#[derive(Debug, Serialize)]
/// Payload to modify opt-in details to the bill manager api.
pub struct OnboardModifyRequest<'mpesa> {
    /// Callback url that will be invoked by our payments API in order to
    /// push payments done to your paybill.
    #[serde(
        rename(serialize = "callbackUrl"),
        skip_serializing_if = "Option::is_none"
    )]
    callback_url: Option<&'mpesa str>,
    /// Official contact email address for the organization signing up to
    /// bill manager.
    #[serde(rename(serialize = "email"), skip_serializing_if = "Option::is_none")]
    email: Option<&'mpesa str>,
    /// Image to be embedded in the invoices and receipts sent to your customer.
    #[serde(rename(serialize = "logo"), skip_serializing_if = "Option::is_none")]
    logo: Option<&'mpesa str>,
    /// Official contact phone number will appear in features sent to the customer such
    /// as invoices and payment receipts for customers to reach out to you as a business.
    #[serde(
        rename(serialize = "officialContact"),
        skip_serializing_if = "Option::is_none"
    )]
    official_contact: Option<&'mpesa str>,
    /// Allows you to enable or disable sms payment reminders for invoices sent.
    #[serde(
        rename(serialize = "sendReminders"),
        skip_serializing_if = "Option::is_none"
    )]
    send_reminders: Option<SendRemindersTypes>,
    /// A shortcode (5 to 6 digit account number) used to identify the organization
    /// and receive the transaction.
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

#[derive(Builder, Clone, Debug)]
#[builder(build_fn(error = "MpesaError"))]
pub struct OnboardModify<'mpesa, Env: ApiEnvironment> {
    #[builder(pattern = "immutable", private)]
    client: &'mpesa Mpesa<Env>,
    /// Callback url that will be invoked by our payments API in order to
    /// push payments done to your paybill.
    #[builder(default = "None", setter(into, strip_option))]
    callback_url: Option<&'mpesa str>,
    /// Official contact email address for the organization signing up to
    /// bill manager.
    #[builder(default = "None", setter(into, strip_option))]
    email: Option<&'mpesa str>,
    /// Image to be embedded in the invoices and receipts sent to your customer.
    #[builder(default = "None", setter(into, strip_option))]
    logo: Option<&'mpesa str>,
    /// Official contact phone number will appear in features sent to the customer such
    /// as invoices and payment receipts for customers to reach out to you as a business.
    #[builder(default = "None", setter(into, strip_option))]
    official_contact: Option<&'mpesa str>,
    /// Allows you to enable or disable sms payment reminders for invoices sent.
    #[builder(default = "None", setter(into, strip_option))]
    send_reminders: Option<SendRemindersTypes>,
    /// A shortcode (5 to 6 digit account number) used to identify the organization
    /// and receive the transaction.
    #[builder(default = "None", setter(into, strip_option))]
    short_code: Option<&'mpesa str>,
}

impl<'mpesa, Env: ApiEnvironment> From<OnboardModify<'mpesa, Env>>
    for OnboardModifyRequest<'mpesa>
{
    fn from(builder: OnboardModify<'mpesa, Env>) -> Self {
        OnboardModifyRequest {
            callback_url: builder.callback_url,
            email: builder.email,
            logo: builder.logo,
            official_contact: builder.official_contact,
            send_reminders: builder.send_reminders,
            short_code: builder.short_code,
        }
    }
}

impl<'mpesa, Env: ApiEnvironment> OnboardModify<'mpesa, Env> {
    pub(crate) fn builder(client: &'mpesa Mpesa<Env>) -> OnboardModifyBuilder<'mpesa, Env> {
        OnboardModifyBuilder::default().client(client)
    }

    /// Builds OnboardModify
    ///
    /// Returns an `OnboardModify` which can be used to build a request.
    pub fn from_request(client: &'mpesa Mpesa<Env>, request: OnboardModifyRequest<'mpesa>) -> Self {
        OnboardModify {
            client,
            callback_url: request.callback_url,
            email: request.email,
            logo: request.logo,
            official_contact: request.official_contact,
            send_reminders: request.send_reminders,
            short_code: request.short_code,
        }
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
        let url = format!(
            "{}/v1/billmanager-invoice/change-optin-details",
            self.client.environment.base_url()
        );

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json::<OnboardModifyRequest>(&self.into())
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json().await?;
            return Ok(value);
        }

        let value = response.json().await?;
        Err(MpesaError::OnboardModifyError(value))
    }
}

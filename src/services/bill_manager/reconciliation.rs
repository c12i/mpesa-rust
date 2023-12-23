use chrono::prelude::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconciliationRequest<'mpesa> {
    /// An account number being invoiced that uniquely identifies a customer.
    account_reference: &'mpesa str,

    /// The date the payment was done and recorded in the bill manager system.
    date_created: DateTime<Utc>,

    /// The customer's phone number, in the format 2547XXXXXXXX
    msisdn: &'mpesa str,

    /// Amount Paid In KES
    paid_amount: f64,

    /// A shortcode (5 to 6 digit account number) used to identify the organization
    /// and receive the transaction.
    short_code: &'mpesa str,

    /// The M-PESA generated reference
    transaction_id: &'mpesa str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReconciliationResponse {
    #[serde(rename(deserialize = "rescode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub response_message: String,
}

#[derive(Builder, Clone, Debug)]
#[builder(build_fn(error = "MpesaError"))]
pub struct Reconciliation<'mpesa, Env: ApiEnvironment> {
    #[builder(pattern = "immutable", private)]
    client: &'mpesa Mpesa<Env>,

    /// An account number being invoiced that uniquely identifies a customer.
    #[builder(setter(into))]
    account_reference: &'mpesa str,

    /// The date the payment was done and recorded in the bill manager system.
    #[builder(setter(into), try_setter)]
    date_created: DateTime<Utc>,

    /// Amount Paid In KES
    #[builder(setter(into))]
    paid_amount: f64,

    /// The customer's phone number, in the format 2547XXXXXXXX
    #[builder(setter(into))]
    msisdn: &'mpesa str,

    /// A shortcode (5 to 6 digit account number) used to identify the organization
    /// and receive the transaction.
    #[builder(setter(into))]
    short_code: &'mpesa str,

    /// The M-PESA generated reference
    #[builder(setter(into))]
    transaction_id: &'mpesa str,
}

impl<'mpesa, Env: ApiEnvironment> From<Reconciliation<'mpesa, Env>>
    for ReconciliationRequest<'mpesa>
{
    fn from(value: Reconciliation<'mpesa, Env>) -> Self {
        ReconciliationRequest {
            account_reference: value.account_reference,
            date_created: value.date_created,
            msisdn: value.msisdn,
            paid_amount: value.paid_amount,
            short_code: value.short_code,
            transaction_id: value.transaction_id,
        }
    }
}

impl<'mpesa, Env: ApiEnvironment> Reconciliation<'mpesa, Env> {
    pub(crate) fn builder(client: &'mpesa Mpesa<Env>) -> ReconciliationBuilder<'mpesa, Env> {
        ReconciliationBuilder::default().client(client)
    }

    /// Builds Reconciliation
    ///
    /// Returns a `Reconciliation` which can be used to send a request.
    pub fn from_request(
        client: &'mpesa Mpesa<Env>,
        request: ReconciliationRequest<'mpesa>,
    ) -> Self {
        Reconciliation {
            client,
            account_reference: request.account_reference,
            date_created: request.date_created,
            msisdn: request.msisdn,
            paid_amount: request.paid_amount,
            short_code: request.short_code,
            transaction_id: request.transaction_id,
        }
    }

    /// Bill Manager Reconciliation API
    ///
    /// Enables your customers to receive e-receipts for payments made to your paybill account
    ///
    /// A successful request returns a `ReconciliationResponse` type.
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure.
    pub async fn send(self) -> MpesaResult<ReconciliationResponse> {
        let url = format!(
            "{}/v1/billmanager-invoice/reconciliation",
            self.client.environment.base_url()
        );

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json::<ReconciliationRequest>(&self.into())
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json().await?;
            return Ok(value);
        }

        let value = response.json().await?;
        Err(MpesaError::ReconciliationError(value))
    }
}

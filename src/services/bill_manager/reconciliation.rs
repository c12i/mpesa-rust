use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReconciliationPayload<'mpesa> {
    account_reference: &'mpesa str,
    external_reference: &'mpesa str,
    full_name: &'mpesa str,
    invoice_name: &'mpesa str,
    paid_amount: f64,
    payment_date: DateTime<Utc>,
    phone_number: &'mpesa str,
    transaction_id: &'mpesa str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReconciliationResponse {
    #[serde(rename(deserialize = "rescode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub response_message: String,
}

#[derive(Debug)]
pub struct ReconciliationBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    account_reference: Option<&'mpesa str>,
    external_reference: Option<&'mpesa str>,
    full_name: Option<&'mpesa str>,
    invoice_name: Option<&'mpesa str>,
    paid_amount: Option<f64>,
    payment_date: Option<DateTime<Utc>>,
    phone_number: Option<&'mpesa str>,
    transaction_id: Option<&'mpesa str>,
}

impl<'mpesa, Env: ApiEnvironment> ReconciliationBuilder<'mpesa, Env> {
    /// Creates a new Bill Manager Reconciliation Builder
    pub fn new(client: &'mpesa Mpesa<Env>) -> ReconciliationBuilder<'mpesa, Env> {
        ReconciliationBuilder {
            client,
            account_reference: None,
            external_reference: None,
            full_name: None,
            invoice_name: None,
            paid_amount: None,
            payment_date: None,
            phone_number: None,
            transaction_id: None,
        }
    }

    /// Adds `account_reference`
    pub fn account_reference(
        mut self,
        account_reference: &'mpesa str,
    ) -> ReconciliationBuilder<'mpesa, Env> {
        self.account_reference = Some(account_reference);
        self
    }

    /// Adds `external_reference`
    pub fn external_reference(
        mut self,
        external_reference: &'mpesa str,
    ) -> ReconciliationBuilder<'mpesa, Env> {
        self.external_reference = Some(external_reference);
        self
    }

    /// Adds `full_name`
    pub fn full_name(mut self, full_name: &'mpesa str) -> ReconciliationBuilder<'mpesa, Env> {
        self.full_name = Some(full_name);
        self
    }

    /// Adds `invoice_name`
    pub fn invoice_name(mut self, invoice_name: &'mpesa str) -> ReconciliationBuilder<'mpesa, Env> {
        self.invoice_name = Some(invoice_name);
        self
    }

    /// Adds `paid_amount`
    pub fn paid_amount<Number: Into<f64>>(
        mut self,
        paid_amount: Number,
    ) -> ReconciliationBuilder<'mpesa, Env> {
        self.paid_amount = Some(paid_amount.into());
        self
    }

    /// Adds `payment_date`
    pub fn payment_date(
        mut self,
        payment_date: DateTime<Utc>,
    ) -> ReconciliationBuilder<'mpesa, Env> {
        self.payment_date = Some(payment_date);
        self
    }

    /// Adds `phone_number`
    pub fn phone_number(mut self, phone_number: &'mpesa str) -> ReconciliationBuilder<'mpesa, Env> {
        self.phone_number = Some(phone_number);
        self
    }

    /// Adds `transaction_id`
    pub fn transaction_id(
        mut self,
        transaction_id: &'mpesa str,
    ) -> ReconciliationBuilder<'mpesa, Env> {
        self.transaction_id = Some(transaction_id);
        self
    }

    /// Bill Manager Reconciliation API
    ///
    /// Enables your customers to receive e-receipts for payments made to your paybill account
    ///
    /// A successful request returns a `ReconciliationResponse` type.
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure.
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<ReconciliationResponse> {
        let url = format!(
            "{}/v1/billmanager-invoice/reconciliation",
            self.client.environment.base_url()
        );

        let payload = ReconciliationPayload {
            account_reference: self
                .account_reference
                .ok_or(MpesaError::Message("account_reference is required"))?,
            external_reference: self
                .external_reference
                .ok_or(MpesaError::Message("external_reference is required"))?,
            full_name: self
                .full_name
                .ok_or(MpesaError::Message("full_name is required"))?,
            invoice_name: self
                .invoice_name
                .ok_or(MpesaError::Message("invoice_name is required"))?,
            paid_amount: self
                .paid_amount
                .ok_or(MpesaError::Message("paid_amount is required"))?,
            payment_date: self
                .payment_date
                .ok_or(MpesaError::Message("payment_date is required"))?,
            phone_number: self
                .phone_number
                .ok_or(MpesaError::Message("phone_number is required"))?,
            transaction_id: self
                .transaction_id
                .ok_or(MpesaError::Message("transaction_id is required"))?,
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
        Err(MpesaError::ReconciliationError(value))
    }
}

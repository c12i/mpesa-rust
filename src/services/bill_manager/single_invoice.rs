use crate::client::{Mpesa, MpesaResult};
use crate::constants::{Invoice, InvoiceItem};
use crate::environment::ApiEnvironment;
use crate::errors::MpesaError;
use chrono::prelude::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SingleInvoiceResponse {
    #[serde(rename(deserialize = "rescode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub response_message: String,
    #[serde(rename(deserialize = "Status_Message"))]
    pub status_message: String,
}

#[derive(Debug)]
pub struct SingleInvoiceBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    amount: Option<f64>,
    account_reference: Option<&'mpesa str>,
    billed_full_name: Option<&'mpesa str>,
    billed_period: Option<&'mpesa str>,
    billed_phone_number: Option<&'mpesa str>,
    due_date: Option<DateTime<Utc>>,
    external_reference: Option<&'mpesa str>,
    invoice_items: Option<Vec<InvoiceItem<'mpesa>>>,
    invoice_name: Option<&'mpesa str>,
}

impl<'mpesa, Env: ApiEnvironment> SingleInvoiceBuilder<'mpesa, Env> {
    /// Creates a new Bill Manager Single Invoice Builder
    pub fn new(client: &'mpesa Mpesa<Env>) -> SingleInvoiceBuilder<'mpesa, Env> {
        SingleInvoiceBuilder {
            client,
            amount: None,
            account_reference: None,
            billed_full_name: None,
            billed_period: None,
            billed_phone_number: None,
            due_date: None,
            external_reference: None,
            invoice_items: None,
            invoice_name: None,
        }
    }

    /// Adds `amount`
    pub fn amount<Number: Into<f64>>(
        mut self,
        amount: Number,
    ) -> SingleInvoiceBuilder<'mpesa, Env> {
        self.amount = Some(amount.into());
        self
    }

    /// Adds `account_reference`
    pub fn account_reference(
        mut self,
        account_refernce: &'mpesa str,
    ) -> SingleInvoiceBuilder<'mpesa, Env> {
        self.account_reference = Some(account_refernce);
        self
    }

    /// Adds `billed_full_name`
    pub fn billed_full_name(
        mut self,
        billed_full_name: &'mpesa str,
    ) -> SingleInvoiceBuilder<'mpesa, Env> {
        self.billed_full_name = Some(billed_full_name);
        self
    }

    /// Adds `billed_period`; must be in the format `"Month Year"` e.g. `"March 2023"`
    pub fn billed_period(
        mut self,
        billed_period: &'mpesa str,
    ) -> SingleInvoiceBuilder<'mpesa, Env> {
        self.billed_period = Some(billed_period);
        self
    }

    /// Adds `billed_phone_number`; must be in the format `0722XXXXXX`
    pub fn billed_phone_number(
        mut self,
        billed_phone_number: &'mpesa str,
    ) -> SingleInvoiceBuilder<'mpesa, Env> {
        self.billed_phone_number = Some(billed_phone_number);
        self
    }

    /// Adds `due_date`
    pub fn due_date(mut self, due_date: DateTime<Utc>) -> SingleInvoiceBuilder<'mpesa, Env> {
        self.due_date = Some(due_date);
        self
    }

    /// Adds `external_reference`
    pub fn external_reference(
        mut self,
        external_reference: &'mpesa str,
    ) -> SingleInvoiceBuilder<'mpesa, Env> {
        self.external_reference = Some(external_reference);
        self
    }

    /// Adds `invoice_items`
    pub fn invoice_items(
        mut self,
        invoice_items: Vec<InvoiceItem<'mpesa>>,
    ) -> SingleInvoiceBuilder<'mpesa, Env> {
        self.invoice_items = Some(invoice_items);
        self
    }

    /// Adds `invoice_name`
    pub fn invoice_name(mut self, invoice_name: &'mpesa str) -> SingleInvoiceBuilder<'mpesa, Env> {
        self.invoice_name = Some(invoice_name);
        self
    }

    /// Bill Manager Single Invoice API
    ///
    /// Creates and sends invoices to your customers
    ///
    /// A successful request returns a `SingleInvoiceResponse` type
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<SingleInvoiceResponse> {
        let url = format!(
            "{}/v1/billmanager-invoice/single-invoicing",
            self.client.environment.base_url()
        );

        let payload = Invoice {
            amount: self
                .amount
                .ok_or(MpesaError::Message("amount is required"))?,
            account_reference: self
                .account_reference
                .ok_or(MpesaError::Message("account_reference is required"))?,
            billed_full_name: self
                .billed_full_name
                .ok_or(MpesaError::Message("billed_full_name is required"))?,
            billed_period: self
                .billed_period
                .ok_or(MpesaError::Message("billed_period is required"))?,
            billed_phone_number: self
                .billed_phone_number
                .ok_or(MpesaError::Message("billed_phone_number is required"))?,
            due_date: self
                .due_date
                .ok_or(MpesaError::Message("due_date is required"))?,
            external_reference: self
                .external_reference
                .ok_or(MpesaError::Message("external_reference is required"))?,
            invoice_items: self.invoice_items,
            invoice_name: self
                .invoice_name
                .ok_or(MpesaError::Message("invoice_name is required"))?,
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
        Err(MpesaError::SingleInvoiceError(value))
    }
}

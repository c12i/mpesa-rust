use crate::client::{Mpesa, MpesaResult};
use crate::constants::Invoice;
use crate::environment::ApiEnvironment;
use crate::errors::MpesaError;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BillManagerBulkInvoiceResponse {
    #[serde(rename(deserialize = "rescode"))]
    pub res_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub res_msg: String,
    #[serde(rename(deserialize = "Status_Message"))]
    pub status_message: String,
}

#[derive(Debug)]
pub struct BillManagerBulkInvoiceBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    invoices: Vec<Invoice<'mpesa>>,
}

impl<'mpesa, Env: ApiEnvironment> BillManagerBulkInvoiceBuilder<'mpesa, Env> {
    /// Creates a new Bill Manager Bulk Invoice builder
    pub fn new(client: &'mpesa Mpesa<Env>) -> BillManagerBulkInvoiceBuilder<'mpesa, Env> {
        BillManagerBulkInvoiceBuilder {
            client,
            invoices: vec![],
        }
    }

    /// Adds an invoice to the bulk of invoices to be sent
    pub fn add_invoice(
        mut self,
        invoice: Invoice<'mpesa>,
    ) -> BillManagerBulkInvoiceBuilder<'mpesa, Env> {
        self.invoices.push(invoice);
        self
    }

    /// Bill Manager Bulk Invoice API
    ///
    /// Sends invoices to your customers in bulk
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure.
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<BillManagerBulkInvoiceResponse> {
        let url = format!(
            "{}/v1/billmanager-invoice/bulk-invoicing",
            self.client.environment.base_url()
        );

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json(&self.invoices)
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json().await?;
            return Ok(value);
        }

        let value = response.json().await?;
        Err(MpesaError::BillManagerBulkInvoiceError(value))
    }
}

#![doc = include_str!("../../../docs/client/bill_manager/bulk_invoice.md")]

use serde::Deserialize;

use crate::client::Mpesa;
use crate::constants::Invoice;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

const BILL_MANAGER_BULK_INVOICE_API_URL: &str = "v1/billmanager-invoice/bulk-invoicing";

#[derive(Clone, Debug, Deserialize)]
pub struct BulkInvoiceResponse {
    #[serde(rename(deserialize = "rescode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub response_message: String,
    #[serde(rename(deserialize = "Status_Message"))]
    pub status_message: String,
}

#[derive(Debug)]
pub struct BulkInvoiceBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    invoices: Vec<Invoice<'mpesa>>,
}

impl<'mpesa, Env: ApiEnvironment> BulkInvoiceBuilder<'mpesa, Env> {
    /// Creates a new Bill Manager Bulk Invoice builder
    pub fn new(client: &'mpesa Mpesa<Env>) -> BulkInvoiceBuilder<'mpesa, Env> {
        BulkInvoiceBuilder {
            client,
            invoices: vec![],
        }
    }

    /// Adds a single `invoice`
    pub fn invoice(mut self, invoice: Invoice<'mpesa>) -> BulkInvoiceBuilder<'mpesa, Env> {
        self.invoices.push(invoice);
        self
    }

    /// Adds multiple `invoices`
    pub fn invoices(
        mut self,
        mut invoices: Vec<Invoice<'mpesa>>,
    ) -> BulkInvoiceBuilder<'mpesa, Env> {
        self.invoices.append(&mut invoices);
        self
    }

    /// Bill Manager Bulk Invoice API
    ///
    /// Sends invoices to your customers in bulk
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure.
    pub async fn send(self) -> MpesaResult<BulkInvoiceResponse> {
        if self.invoices.is_empty() {
            return Err(MpesaError::Message("invoices cannot be empty"));
        }

        self.client
            .send(crate::client::Request {
                method: reqwest::Method::POST,
                path: BILL_MANAGER_BULK_INVOICE_API_URL,
                body: self.invoices,
            })
            .await
    }
}

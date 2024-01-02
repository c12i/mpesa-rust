#![doc = include_str!("../../../docs/client/bill_manager/cancel_invoice.md")]

use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::errors::MpesaResult;

const BILL_MANAGER_CANCEL_INVOICE_API_URL: &str = "v1/billmanager-invoice/cancel-single-invoice";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CancelInvoicePayload<'mpesa> {
    external_reference: &'mpesa str,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CancelInvoiceResponse {
    #[serde(rename(deserialize = "rescode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub response_message: String,
    #[serde(rename(deserialize = "Status_Message"))]
    pub status_message: String,
}

#[derive(Debug)]
pub struct CancelInvoiceBuilder<'mpesa> {
    client: &'mpesa Mpesa,
    external_references: Vec<CancelInvoicePayload<'mpesa>>,
}

impl<'mpesa> CancelInvoiceBuilder<'mpesa> {
    /// Creates a new Bill Manager Cancel invoice builder
    pub fn new(client: &'mpesa Mpesa) -> CancelInvoiceBuilder<'mpesa> {
        CancelInvoiceBuilder {
            client,
            external_references: vec![],
        }
    }

    /// Adds an `external_reference`
    pub fn external_reference(
        mut self,
        external_reference: &'mpesa str,
    ) -> CancelInvoiceBuilder<'mpesa> {
        self.external_references
            .push(CancelInvoicePayload { external_reference });
        self
    }

    /// Adds `external_references`
    pub fn external_references(
        mut self,
        external_references: Vec<&'mpesa str>,
    ) -> CancelInvoiceBuilder<'mpesa> {
        self.external_references.append(
            &mut external_references
                .into_iter()
                .map(|external_reference| CancelInvoicePayload { external_reference })
                .collect(),
        );
        self
    }

    /// Bill Manager Cancel Invoice API
    ///
    /// Cancels a list of invoices by their `external_reference`
    ///
    /// A successful request returns a `CancelInvoiceResponse` type
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<CancelInvoiceResponse> {
        self.client
            .send(crate::client::Request {
                method: reqwest::Method::POST,
                path: BILL_MANAGER_CANCEL_INVOICE_API_URL,
                body: self.external_references,
            })
            .await
    }
}

use serde::Deserialize;

use crate::client::Mpesa;
use crate::constants::CancelInvoice;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

#[derive(Clone, Debug, Deserialize)]
pub struct CancelBulkInvoicesResponse {
    #[serde(rename(deserialize = "rescode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub response_message: String,
    #[serde(rename(deserialize = "Status_Message"))]
    pub status_message: String,
}

#[derive(Debug)]
pub struct CancelBulkInvoicesBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    external_references: Vec<CancelInvoice<'mpesa>>,
}

impl<'mpesa, Env: ApiEnvironment> CancelBulkInvoicesBuilder<'mpesa, Env> {
    /// Creates a new Bill Manager Cancel bulk invoices builder
    pub fn new(client: &'mpesa Mpesa<Env>) -> CancelBulkInvoicesBuilder<'mpesa, Env> {
        CancelBulkInvoicesBuilder {
            client,
            external_references: vec![],
        }
    }

    /// Adds an `external_reference`
    pub fn external_reference(
        mut self,
        external_reference: &'mpesa str,
    ) -> CancelBulkInvoicesBuilder<'mpesa, Env> {
        self.external_references
            .push(CancelInvoice { external_reference });
        self
    }

    /// Adds `external_references`
    pub fn external_references(
        mut self,
        external_references: Vec<&'mpesa str>,
    ) -> CancelBulkInvoicesBuilder<'mpesa, Env> {
        self.external_references.append(
            &mut external_references
                .into_iter()
                .map(|external_reference| CancelInvoice { external_reference })
                .collect(),
        );
        self
    }

    /// Bill Manager Cancel Bulk Invoices API
    ///
    /// Cancels a list of invoices by their `external_reference`
    ///
    /// A successful request returns a `CancelBulkInvoicesResponse` type
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<CancelBulkInvoicesResponse> {
        let url = format!(
            "{}/v1/billmanager-invoice/cancel-bulk-invoices",
            self.client.environment.base_url()
        );

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json(&self.external_references)
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json().await?;
            return Ok(value);
        }

        let value = response.json().await?;
        Err(MpesaError::CancelBulkInvoicesError(value))
    }
}

use crate::client::{Mpesa, MpesaResult};
use crate::environment::ApiEnvironment;
use crate::errors::MpesaError;
use serde::{Deserialize, Serialize};

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
pub struct CancelInvoiceBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    external_references: Vec<CancelInvoicePayload<'mpesa>>,
}

impl<'mpesa, Env: ApiEnvironment> CancelInvoiceBuilder<'mpesa, Env> {
    /// Creates a new Bill Manager Cancel invoice builder
    pub fn new(client: &'mpesa Mpesa<Env>) -> CancelInvoiceBuilder<'mpesa, Env> {
        CancelInvoiceBuilder {
            client,
            external_references: vec![],
        }
    }

    /// Adds `external_references`
    pub fn external_references(
        mut self,
        external_references: Vec<&'mpesa str>,
    ) -> CancelInvoiceBuilder<'mpesa, Env> {
        self.external_references = external_references
            .into_iter()
            .map(|external_reference| CancelInvoicePayload { external_reference })
            .collect();
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
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<CancelInvoiceResponse> {
        let url = format!(
            "{}/v1/billmanager-invoice/cancel-single-invoice",
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
        Err(MpesaError::CancelInvoiceError(value))
    }
}

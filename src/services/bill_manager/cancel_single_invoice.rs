use derive_builder::Builder;
use serde::Deserialize;

use crate::client::Mpesa;
use crate::constants::CancelInvoice;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

#[derive(Clone, Debug, Deserialize)]
pub struct CancelSingleInvoiceResponse {
    #[serde(rename(deserialize = "rescode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub response_message: String,
    #[serde(rename(deserialize = "Status_Message"))]
    pub status_message: String,
}

#[derive(Builder, Clone, Debug)]
#[builder(build_fn(error = "MpesaError"))]
pub struct CancelSingleInvoice<'mpesa, Env: ApiEnvironment> {
    #[builder(pattern = "immutable", private)]
    client: &'mpesa Mpesa<Env>,

    #[builder(setter(into), try_setter)]
    external_reference: &'mpesa str,
}

impl<'mpesa, Env: ApiEnvironment> From<CancelSingleInvoice<'mpesa, Env>> for CancelInvoice<'mpesa> {
    fn from(builder: CancelSingleInvoice<'mpesa, Env>) -> Self {
        CancelInvoice {
            external_reference: builder.external_reference,
        }
    }
}

impl<'mpesa, Env: ApiEnvironment> CancelSingleInvoice<'mpesa, Env> {
    pub(crate) fn builder(client: &'mpesa Mpesa<Env>) -> CancelSingleInvoiceBuilder<'mpesa, Env> {
        CancelSingleInvoiceBuilder::default().client(client)
    }

    /// Bill Manager Cancel Single Invoice API
    ///
    /// Cancels an invoice by its `external_reference`
    ///
    /// A successful request returns a `CancelSingleInvoiceResponse` type
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<CancelSingleInvoiceResponse> {
        let url = format!(
            "{}/v1/billmanager-invoice/cancel-single-invoice",
            self.client.environment.base_url()
        );

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json::<CancelInvoice>(&self.into())
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json().await?;
            return Ok(value);
        }

        let value = response.json().await?;
        Err(MpesaError::CancelSingleInvoiceError(value))
    }
}

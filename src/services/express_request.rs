use crate::client::{Mpesa, MpesaResult};
use crate::constants::CommandId;
use crate::environment::ApiEnvironment;
use crate::errors::MpesaError;
use chrono::prelude::Local;
use derive_builder::Builder;
use openssl::base64;
use serde::{Deserialize, Serialize};

/// Source: [test credentials](https://developer.safaricom.co.ke/test_credentials)
static DEFAULT_PASSKEY: &str = "bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919";

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct MpesaExpressRequest<'mpesa> {
    business_short_code: &'mpesa str,
    password: String,
    timestamp: String,
    transaction_type: CommandId,
    amount: f64,
    party_a: &'mpesa str,
    party_b: &'mpesa str,
    phone_number: &'mpesa str,
    call_back_url: &'mpesa str,
    account_reference: &'mpesa str,
    transaction_desc: Option<&'mpesa str>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MpesaExpressRequestResponse {
    pub checkout_request_id: String,
    pub customer_message: String,
    pub merchant_request_id: String,
    pub response_code: String,
    pub response_description: String,
}

#[derive(Builder, Debug, Clone)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct MpesaExpress<'mpesa, Env: ApiEnvironment> {
    #[builder(pattern = "owned")]
    client: &'mpesa Mpesa<Env>,
    #[builder(setter(into))]
    business_short_code: &'mpesa str,
    transaction_type: CommandId,
    #[builder(setter(into))]
    amount: f64,
    party_a: &'mpesa str,
    party_b: &'mpesa str,
    phone_number: &'mpesa str,
    callback_url: &'mpesa str,
    #[builder(setter(into))]
    account_ref: &'mpesa str,
    #[builder(setter(into, strip_option), default)]
    transaction_desc: Option<&'mpesa str>,
    #[builder(setter(into))]
    pass_key: &'mpesa str,
}

impl<'mpesa, Env: ApiEnvironment> From<MpesaExpress<'mpesa, Env>> for MpesaExpressRequest<'mpesa> {
    fn from(express: MpesaExpress<'mpesa, Env>) -> MpesaExpressRequest<'mpesa> {
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let encoded_password = base64::encode_block(
            format!(
                "{}{}{}",
                express.business_short_code, express.pass_key, timestamp
            )
            .as_bytes(),
        );

        MpesaExpressRequest {
            business_short_code: express.business_short_code,
            password: encoded_password,
            timestamp,
            transaction_type: express.transaction_type,
            amount: express.amount,
            party_a: express.party_a,
            party_b: express.party_b,
            phone_number: express.phone_number,
            call_back_url: express.callback_url,
            account_reference: express.account_ref,
            transaction_desc: express.transaction_desc,
        }
    }
}

impl<'mpesa, Env: ApiEnvironment> MpesaExpressBuilder<'mpesa, Env> {
    fn validate(&self) -> Result<(), String> {
        // None of the option can be none except transaction desc
        Ok(())
    }
}
impl<'mpesa, Env: ApiEnvironment> MpesaExpress<'mpesa, Env> {
    pub(crate) fn builder(client: &'mpesa Mpesa<Env>) -> MpesaExpressBuilder<'mpesa, Env> {
        MpesaExpressBuilder::default().client(client)
    }

    /// # Lipa na M-Pesa Online Payment / Mpesa Express/ Stk push
    ///
    /// Initiates a M-Pesa transaction on behalf of a customer using STK Push
    ///
    /// A sucessfult request returns a `MpesaExpressRequestResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    #[allow(clippy::or_fun_call)]
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<MpesaExpressRequestResponse> {
        let url = format!(
            "{}/mpesa/stkpush/v1/processrequest",
            self.client.environment.base_url()
        );

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json::<MpesaExpressRequest>(&self.into())
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json::<_>().await?;
            return Ok(value);
        }

        let value = response.json().await?;
        Err(MpesaError::MpesaExpressRequestError(value))
    }
}

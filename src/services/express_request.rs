use chrono::prelude::Local;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use openssl::base64;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::client::Mpesa;
use crate::constants::CommandId;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

/// Source: [test credentials](https://developer.safaricom.co.ke/test_credentials)
static DEFAULT_PASSKEY: &str = "bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919";

const EXPRESS_REQUEST_URL: &str = "/mpesa/stkpush/v1/processrequest";

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MpesaExpressRequest<'mpesa> {
    /// This is the organization's shortcode (Paybill or Buygoods - A 5 to
    /// 6-digit account number) used to identify an organization and receive
    /// the transaction.
    pub business_short_code: &'mpesa str,
    /// This is the password used for encrypting the request sent:
    pub password: String,
    /// This is the Timestamp of the transaction, normally in the format of
    /// (YYYYMMDDHHMMSS)
    #[serde(serialize_with = "serialize_utc_to_string")]
    pub timestamp: DateTime<Utc>,
    /// This is the transaction type that is used to identify the transaction
    /// when sending the request to M-PESA
    pub transaction_type: CommandId,
    /// This is the Amount transacted normally a numeric value
    pub amount: f64,
    ///The phone number sending money.
    pub party_a: &'mpesa str,
    /// The organization that receives the funds
    pub party_b: &'mpesa str,
    /// The Mobile Number to receive the STK Pin Prompt.
    /// This number can be the same as PartyA value above.
    ///
    ///  The parameter expected is a Valid Safaricom Mobile Number that is
    /// M-PESA registered in the format 2547XXXXXXXX
    pub phone_number: &'mpesa str,
    /// A CallBack URL is a valid secure URL that is used to receive
    /// notifications from M-Pesa API.
    /// It is the endpoint to which the results will be sent by M-Pesa API.
    #[serde(rename = "CallBackURL")]
    pub call_back_url: Url,
    /// Account Reference: This is an Alpha-Numeric parameter that is defined
    /// by your system as an Identifier of the transaction for
    /// CustomerPayBillOnline
    pub account_reference: &'mpesa str,
    /// This is any additional information/comment that can be sent along with
    /// the request from your system
    pub transaction_desc: Option<&'mpesa str>,
}

fn serialize_utc_to_string<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let date = date.with_timezone(&Local);
    let s = date.format("%Y%m%d%H%M%S").to_string();
    serializer.serialize_str(&s)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MpesaExpressResponse {
    ///This is a global unique identifier of the processed checkout transaction
    /// request.
    #[serde(rename = "CheckoutRequestID")]
    pub checkout_request_id: String,
    pub customer_message: String,
    /// This is a global unique Identifier for any submitted payment request.
    #[serde(rename = "MerchantRequestID")]
    pub merchant_request_id: String,
    /// This is a Numeric status code that indicates the status of the
    /// transaction submission. 0 means successful submission and any other
    /// code means an error occurred.
    pub response_code: String,
    ///Response description is an acknowledgment message from the API that
    /// gives the status of the request submission. It usually maps to a
    /// specific ResponseCode value.
    ///
    /// It can be a Success submission message or an error description.
    pub response_description: String,
}

#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "MpesaError"))]
pub struct MpesaExpress<'mpesa, Env: ApiEnvironment> {
    #[builder(pattern = "immutable")]
    client: &'mpesa Mpesa<Env>,
    #[builder(setter(into))]
    business_short_code: &'mpesa str,
    transaction_type: CommandId,
    #[builder(setter(into))]
    amount: f64,
    party_a: &'mpesa str,
    party_b: &'mpesa str,
    phone_number: &'mpesa str,
    #[builder(try_setter, setter(into))]
    callback_url: Url,
    #[builder(setter(into))]
    account_ref: &'mpesa str,
    #[builder(setter(into, strip_option), default)]
    transaction_desc: Option<&'mpesa str>,
    #[builder(setter(into))]
    pass_key: &'mpesa str,
}

impl<'mpesa, Env: ApiEnvironment> From<MpesaExpress<'mpesa, Env>> for MpesaExpressRequest<'mpesa> {
    fn from(express: MpesaExpress<'mpesa, Env>) -> MpesaExpressRequest<'mpesa> {
        let timestamp = chrono::Utc::now();

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

impl<'mpesa, Env: ApiEnvironment> MpesaExpress<'mpesa, Env> {
    /// Creates new `MpesaExpressBuilder`
    pub(crate) fn builder(client: &'mpesa Mpesa<Env>) -> MpesaExpressBuilder<'mpesa, Env> {
        MpesaExpressBuilder::default().client(client)
    }

    pub fn from_request(
        client: &'mpesa Mpesa<Env>,
        request: MpesaExpressRequest<'mpesa>,
    ) -> MpesaExpress<'mpesa, Env> {
        MpesaExpress {
            client,
            business_short_code: request.business_short_code,
            transaction_type: request.transaction_type,
            amount: request.amount,
            party_a: request.party_a,
            party_b: request.party_b,
            phone_number: request.phone_number,
            callback_url: request.call_back_url,
            account_ref: request.account_reference,
            transaction_desc: request.transaction_desc,
            pass_key: DEFAULT_PASSKEY,
        }
    }

    /// # Lipa na M-Pesa Online Payment / Mpesa Express/ Stk push
    ///
    /// Initiates a M-Pesa transaction on behalf of a customer using STK Push
    ///
    /// A sucessfult request returns a `MpesaExpressRequestResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<MpesaExpressResponse> {
        let url = format!(
            "{}{}",
            self.client.environment.base_url(),
            EXPRESS_REQUEST_URL
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

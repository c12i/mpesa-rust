#![doc = include_str!("../../../docs/client/express_request.md")]

use chrono::prelude::Local;
use chrono::DateTime;
use derive_builder::Builder;
use openssl::base64;
use serde::{Deserialize, Serialize};

use super::{serialize_utc_to_string, DEFAULT_PASSKEY};
use crate::client::Mpesa;
use crate::errors::{MpesaError, MpesaResult};

const EXPRESS_QUERY_URL: &str = "mpesa/stkpushquery/v1/query";

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MpesaExpressQueryRequest<'mpesa> {
    /// This is the organization's shortcode (Paybill or Buygoods - A 5 to
    /// 6-digit account number) used to identify an organization and receive
    /// the transaction.
    pub business_short_code: &'mpesa str,
    /// This is the password used for encrypting the request sent:
    pub password: String,
    /// This is the Timestamp of the transaction, normally in the format of
    /// (YYYYMMDDHHMMSS)
    #[serde(serialize_with = "serialize_utc_to_string")]
    pub timestamp: DateTime<Local>,
    /// This is a global unique identifier of the processed checkout transaction
    /// request.
    #[serde(rename = "CheckoutRequestID")]
    pub checkout_request_id: &'mpesa str,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MpesaExpressQueryResponse {
    /// This is a global unique identifier of the processed checkout transaction
    /// request.
    #[serde(rename = "CheckoutRequestID")]
    pub checkout_request_id: String,
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

    /// This is a Numeric status code that indicates the status of the
    /// transaction submission. 0 means successful submission and any other
    /// code means an error occurred.
    pub result_code: String,
    ///Response description is an acknowledgment message from the API that
    /// gives the status of the request submission. It usually maps to a
    /// specific ResponseCode value.
    pub result_desc: String,
}

#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "MpesaError"))]
pub struct MpesaExpressQuery<'mpesa> {
    #[builder(pattern = "immutable")]
    client: &'mpesa Mpesa,
    /// This is the organization's shortcode (Paybill or Buygoods - A 5 to
    /// 6-digit account number) used to identify an organization and receive
    /// the transaction.
    #[builder(setter(into))]
    business_short_code: &'mpesa str,

    /// This is the password used for encrypting the request sent:
    /// The password for encrypting the request is obtained by base64 encoding
    /// BusinessShortCode, Passkey and Timestamp.
    /// The timestamp format is YYYYMMDDHHmmss
    #[builder(setter(into, strip_option), default = "Some(DEFAULT_PASSKEY)")]
    pass_key: Option<&'mpesa str>,

    /// This is a global unique identifier of the processed checkout transaction
    /// request.
    #[builder(setter(into))]
    checkout_request_id: &'mpesa str,
}

impl<'mpesa> From<MpesaExpressQuery<'mpesa>> for MpesaExpressQueryRequest<'mpesa> {
    fn from(express: MpesaExpressQuery<'mpesa>) -> MpesaExpressQueryRequest<'mpesa> {
        let timestamp = chrono::Local::now();

        let encoded_password =
            MpesaExpressQuery::encode_password(express.business_short_code, express.pass_key);

        MpesaExpressQueryRequest {
            business_short_code: express.business_short_code,
            password: encoded_password,
            timestamp,
            checkout_request_id: express.checkout_request_id,
        }
    }
}

impl<'mpesa> MpesaExpressQuery<'mpesa> {
    /// Creates new `MpesaExpressQueryBuilder`
    pub(crate) fn builder(client: &'mpesa Mpesa) -> MpesaExpressQueryBuilder<'mpesa> {
        MpesaExpressQueryBuilder::default().client(client)
    }

    /// Encodes the password for the request
    /// The password for encrypting the request is obtained by base64 encoding
    /// BusinessShortCode, Passkey and Timestamp.
    /// The timestamp format is YYYYMMDDHHmmss
    pub fn encode_password(business_short_code: &str, pass_key: Option<&'mpesa str>) -> String {
        let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
        base64::encode_block(
            format!(
                "{}{}{}",
                business_short_code,
                pass_key.unwrap_or(DEFAULT_PASSKEY),
                timestamp
            )
            .as_bytes(),
        )
    }

    /// Creates a new `MpesaExpressQuery` from a `MpesaExpressQueryRequest`
    pub fn from_request(
        client: &'mpesa Mpesa,
        request: MpesaExpressQueryRequest<'mpesa>,
        pass_key: Option<&'mpesa str>,
    ) -> MpesaExpressQuery<'mpesa> {
        MpesaExpressQuery {
            client,
            business_short_code: request.business_short_code,
            checkout_request_id: request.checkout_request_id,
            pass_key,
        }
    }

    /// # Lipa na M-Pesa Online Payment / Mpesa Express/ Stk push
    ///
    /// Initiates a M-Pesa transaction on behalf of a customer using STK Push
    ///
    /// A successful request returns a `MpesaExpressQueryResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<MpesaExpressQueryResponse> {
        self.client
            .send::<MpesaExpressQueryRequest, _>(crate::client::Request {
                method: reqwest::Method::POST,
                path: EXPRESS_QUERY_URL,
                body: self.into(),
            })
            .await
    }
}

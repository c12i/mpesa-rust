use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::client::{Mpesa, MpesaResult};
use crate::constants::TransactionType;
use crate::environment::ApiEnvironment;
use crate::errors::MpesaError;

const DYNAMIC_QR_URL: &str = "/mpesa/qrcode/v1/generate";

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct DynamicQRRequest<'mpesa> {
    /// Name of the Company/M-Pesa Merchant Name
    merchant_name: &'mpesa str,
    /// Transaction Reference Number
    ref_no: &'mpesa str,
    /// The total amount of the transaction
    amount: f64,
    #[serde(rename = "TrxCode")]
    /// Transaction Type
    transaction_type: TransactionType,
    ///Credit Party Identifier.
    ///
    /// Can be a Mobile Number, Business Number, Agent
    /// Till, Paybill or Business number, or Merchant Buy Goods.
    #[serde(rename = "CPI")]
    credit_party_identifier: &'mpesa str,
    /// Size of the QR code image in pixels.
    ///
    /// QR code image will always be a square image.
    size: &'mpesa str,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DynamicQRResponse {
    #[serde(rename(deserialize = "QRCode"))]
    pub qr_code: String,
    pub response_code: String,
    pub response_description: String,
}

/// Dynamic QR builder struct
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "MpesaError"))]
pub struct DynamicQR<'mpesa, Env: ApiEnvironment> {
    #[builder(pattern = "immutable")]
    client: &'mpesa Mpesa<Env>,
    /// Name of the Company/M-Pesa Merchant Name
    #[builder(setter(into))]
    merchant_name: &'mpesa str,
    /// Transaction Reference Number
    #[builder(setter(into))]
    amount: f64,
    /// The total amount of the transaction
    ref_no: &'mpesa str,
    /// Transaction Type
    ///
    /// This can be a `TransactionType` or a `&str`
    /// The `&str` must be one of the following:
    /// - `BG` for Buy Goods
    /// - `PB` for Pay Bill
    /// - `WA` Withdraw Cash
    /// - `SM` Send Money (Mobile Number)
    /// - `SB` Sent to Business. Business number CPI in MSISDN format.
    #[builder(try_setter, setter(into))]
    transaction_type: TransactionType,
    /// Credit Party Identifier.
    /// Can be a Mobile Number, Business Number, Agent
    /// Till, Paybill or Business number, or Merchant Buy Goods.
    #[builder(setter(into))]
    credit_party_identifier: &'mpesa str,
    /// Size of the QR code image in pixels.
    ///
    /// QR code image will always be a square image.
    #[builder(setter(into))]
    size: &'mpesa str,
}

impl<'mpesa, Env: ApiEnvironment> From<DynamicQR<'mpesa, Env>> for DynamicQRRequest<'mpesa> {
    fn from(express: DynamicQR<'mpesa, Env>) -> DynamicQRRequest<'mpesa> {
        DynamicQRRequest {
            merchant_name: express.merchant_name,
            ref_no: express.ref_no,
            amount: express.amount,
            transaction_type: express.transaction_type,
            credit_party_identifier: express.credit_party_identifier,
            size: express.size,
        }
    }
}

impl<'mpesa, Env: ApiEnvironment> DynamicQR<'mpesa, Env> {
    pub(crate) fn builder(client: &'mpesa Mpesa<Env>) -> DynamicQRBuilder<'mpesa, Env> {
        DynamicQRBuilder::default().client(client)
    }

    /// # Generate a Dynamic QR
    ///
    /// This enables Safaricom M-PESA customers who
    /// have My Safaricom App or M-PESA app, to scan a QR (Quick Response)
    /// code, to capture till number and amount then authorize to pay for goods
    /// and services at select LIPA NA M-PESA (LNM) merchant outlets.
    ///
    /// # Response
    /// A successful request returns a `DynamicQRResponse` type
    /// which contains the QR code
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<DynamicQRResponse> {
        let url = format!("{}{}", self.client.environment.base_url(), DYNAMIC_QR_URL);

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json::<DynamicQRRequest>(&self.into())
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json::<_>().await?;
            return Ok(value);
        }

        let value = response.json().await?;
        Err(MpesaError::MpesaDynamicQrError(value))
    }
}

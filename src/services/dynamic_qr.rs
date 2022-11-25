use serde::Deserialize;
use serde::Serialize;

use crate::constants::TransactionType;
use crate::ApiEnvironment;
use crate::Mpesa;
use crate::MpesaError;
use crate::MpesaResult;

#[derive(Debug, Serialize)]
pub struct DynamicQRPayload<'mpesa> {
    #[serde(rename(serialize = "MerchantName"))]
    merchant_name: Option<&'mpesa str>,
    #[serde(rename(serialize = "RefNo"))]
    ref_no: Option<&'mpesa str>,
    #[serde(rename(serialize = "TrxCode"))]
    trx_code: Option<TransactionType>,
    #[serde(rename(serialize = "CPI"))]
    credit_party_identifier: Option<&'mpesa str>,
    #[serde(rename(serialize = "Amount"))]
    amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicQRResponse {
    #[serde(rename(deserialize = "ResponseCode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "QRCode"))]
    pub qr_code: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    pub response_description: String,
}

#[derive(Debug)]
pub struct DynamicQRBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    merchant_name: Option<&'mpesa str>,
    ref_no: Option<&'mpesa str>,
    credit_party_identifier: Option<&'mpesa str>,
    amount: Option<f64>,
    trx_code: Option<TransactionType>,
}

impl<'mpesa, Env: ApiEnvironment> DynamicQRBuilder<'mpesa, Env> {
    pub fn new(client: &'mpesa Mpesa<Env>) -> Self {
        Self {
            client,
            merchant_name: None,
            ref_no: None,
            credit_party_identifier: None,
            amount: None,
            trx_code: None,
        }
    }

    pub fn merchant_name(mut self, merchant_name: &'mpesa str) -> Self {
        self.merchant_name = Some(merchant_name);
        self
    }
    pub fn amount<Number: Into<f64>>(mut self, amount: Number) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn trx_code(mut self, trx_code: TransactionType) -> Self {
        self.trx_code = Some(trx_code);
        self
    }
    pub fn ref_no(mut self, ref_no: &'mpesa str) -> Self {
        self.ref_no = Some(ref_no);
        self
    }

    pub fn credit_party_identifier(mut self, credit_party_identifier: &'mpesa str) -> Self {
        self.credit_party_identifier = Some(credit_party_identifier);
        self
    }

    pub async fn send(self) -> MpesaResult<DynamicQRResponse> {
        let url = format!(
            "{}/mpesa/qrcode/v1/generate",
            self.client.environment.base_url()
        );

        let payload = DynamicQRPayload {
            merchant_name: self.merchant_name,
            ref_no: self.ref_no,
            trx_code: self.trx_code,
            credit_party_identifier: self.credit_party_identifier,
            amount: self.amount,
        };
        eprintln!("Value {}", serde_json::to_string(&payload).unwrap());
        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let value = response.json::<_>().await?;
            return Err(MpesaError::MpesaDynamicQRError(value));
        };
        let response = response.json().await?;
        Ok(response)
    }
}

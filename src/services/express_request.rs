use chrono::prelude::Local;
use openssl::base64;
use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::constants::CommandId;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

/// Source: [test credentials](https://developer.safaricom.co.ke/test_credentials)
static DEFAULT_PASSKEY: &str = "bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919";

#[derive(Debug, Serialize)]
struct MpesaExpressRequestPayload<'mpesa> {
    #[serde(rename(serialize = "BusinessShortCode"))]
    business_short_code: &'mpesa str,
    #[serde(rename(serialize = "Password"))]
    password: &'mpesa str,
    #[serde(rename(serialize = "Timestamp"))]
    timestamp: &'mpesa str,
    #[serde(rename(serialize = "TransactionType"))]
    transaction_type: CommandId,
    #[serde(rename(serialize = "Amount"))]
    amount: f64,
    #[serde(rename(serialize = "PartyA"), skip_serializing_if = "Option::is_none")]
    party_a: Option<&'mpesa str>,
    #[serde(rename(serialize = "PartyB"), skip_serializing_if = "Option::is_none")]
    party_b: Option<&'mpesa str>,
    #[serde(rename(serialize = "PhoneNumber"))]
    phone_number: &'mpesa str,
    #[serde(rename(serialize = "CallBackURL"))]
    call_back_url: &'mpesa str,
    #[serde(rename(serialize = "AccountReference"))]
    account_reference: &'mpesa str,
    #[serde(rename(serialize = "TransactionDesc"))]
    transaction_desc: &'mpesa str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MpesaExpressRequestResponse {
    #[serde(rename(deserialize = "CheckoutRequestID"))]
    pub checkout_request_id: String,
    #[serde(rename(deserialize = "CustomerMessage"))]
    pub customer_message: String,
    #[serde(rename(deserialize = "MerchantRequestID"))]
    pub merchant_request_id: String,
    #[serde(rename(deserialize = "ResponseCode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    pub response_description: String,
}

pub struct MpesaExpressRequestBuilder<'mpesa, Env: ApiEnvironment> {
    business_short_code: &'mpesa str,
    client: &'mpesa Mpesa<Env>,
    transaction_type: Option<CommandId>,
    amount: Option<f64>,
    party_a: Option<&'mpesa str>,
    party_b: Option<&'mpesa str>,
    phone_number: Option<&'mpesa str>,
    callback_url: Option<&'mpesa str>,
    account_ref: Option<&'mpesa str>,
    transaction_desc: Option<&'mpesa str>,
    pass_key: Option<&'mpesa str>,
}

impl<'mpesa, Env: ApiEnvironment> MpesaExpressRequestBuilder<'mpesa, Env> {
    pub fn new(
        client: &'mpesa Mpesa<Env>,
        business_short_code: &'mpesa str,
    ) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        MpesaExpressRequestBuilder {
            client,
            business_short_code,
            transaction_type: None,
            transaction_desc: None,
            amount: None,
            party_a: None,
            party_b: None,
            phone_number: None,
            callback_url: None,
            account_ref: None,
            pass_key: None,
        }
    }

    /// Public method get the `business_short_code`
    pub fn business_short_code(&'mpesa self) -> &'mpesa str {
        self.business_short_code
    }

    /// Retrieves the production passkey if present or defaults to the key provided in Safaricom's [test credentials](https://developer.safaricom.co.ke/test_credentials)
    fn get_pass_key(&'mpesa self) -> &'mpesa str {
        if let Some(key) = self.pass_key {
            return key;
        }
        DEFAULT_PASSKEY
    }

    /// Utility method to generate base64 encoded password as per Safaricom's [specifications](https://developer.safaricom.co.ke/docs#lipa-na-m-pesa-online-payment)
    /// Returns the encoded password and a timestamp string
    fn generate_password_and_timestamp(&self) -> (String, String) {
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let encoded_password = base64::encode_block(
            format!(
                "{}{}{}",
                self.business_short_code(),
                self.get_pass_key(),
                timestamp
            )
            .as_bytes(),
        );
        (encoded_password, timestamp)
    }

    /// Your passkey.
    /// Optional in sandbox, will default to key provided in Safaricom's [test credentials](https://developer.safaricom.co.ke/test_credentials)
    /// Required in production
    ///
    /// # Errors
    /// If thee `pass_key` is invalid
    pub fn pass_key(mut self, pass_key: &'mpesa str) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        self.pass_key = Some(pass_key);
        self
    }

    /// Adds an `amount` to the request
    /// This is a required field
    pub fn amount<Number: Into<f64>>(
        mut self,
        amount: Number,
    ) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        self.amount = Some(amount.into());
        self
    }

    /// The MSISDN sending the funds
    ///
    /// # Errors
    /// If `phone_number` is invalid
    pub fn phone_number(
        mut self,
        phone_number: &'mpesa str,
    ) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        self.phone_number = Some(phone_number);
        self
    }

    /// The url to where responses from M-Pesa will be sent to.
    ///
    /// # Errors
    /// If the `callback_url` is invalid
    pub fn callback_url(
        mut self,
        callback_url: &'mpesa str,
    ) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        self.callback_url = Some(callback_url);
        self
    }

    /// The MSISDN sending the funds
    ///
    /// # Errors
    /// If `party_a` is invalid
    pub fn party_a(mut self, party_a: &'mpesa str) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        self.party_a = Some(party_a);
        self
    }

    /// The organization shortcode receiving the funds
    ///
    /// # Errors
    /// If `party_b` is invalid
    pub fn party_b(mut self, party_b: &'mpesa str) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        self.party_b = Some(party_b);
        self
    }

    /// Optional - Used with M-Pesa PayBills.
    pub fn account_ref(
        mut self,
        account_ref: &'mpesa str,
    ) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        self.account_ref = Some(account_ref);
        self
    }

    /// Optional, defaults to `CommandId::CustomerPayBillOnline`
    ///
    /// # Errors
    /// If the `CommandId` is invalid
    pub fn transaction_type(
        mut self,
        command_id: CommandId,
    ) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        self.transaction_type = Some(command_id);
        self
    }

    /// A description of the transaction.
    /// Optional - defaults to "None"
    pub fn transaction_desc(
        mut self,
        description: &'mpesa str,
    ) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        self.transaction_desc = Some(description);
        self
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

        let (password, timestamp) = self.generate_password_and_timestamp();

        let payload = MpesaExpressRequestPayload {
            business_short_code: self.business_short_code,
            password: &password,
            timestamp: &timestamp,
            amount: self
                .amount
                .ok_or(MpesaError::Message("amount is required"))?,
            party_a: if self.party_a.is_some() {
                self.party_a
            } else {
                self.phone_number
            },
            party_b: if self.party_b.is_some() {
                self.party_b
            } else {
                Some(self.business_short_code)
            },
            phone_number: self
                .phone_number
                .ok_or(MpesaError::Message("phone_number is required"))?,
            call_back_url: self
                .callback_url
                .ok_or(MpesaError::Message("callback_url is required"))?,
            account_reference: self.account_ref.unwrap_or_else(|| stringify!(None)),
            transaction_type: self
                .transaction_type
                .unwrap_or_else(|| CommandId::CustomerPayBillOnline),
            transaction_desc: self.transaction_desc.unwrap_or_else(|| stringify!(None)),
        };

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json(&payload)
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

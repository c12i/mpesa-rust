use crate::client::{Mpesa, MpesaResult};
use crate::constants::CommandId;
use crate::errors::MpesaError;
use chrono::prelude::Local;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
struct MpesaExpressRequestPayload<'a> {
    BusinessShortCode: &'a str,
    Password: &'a str,
    Timestamp: &'a str,
    TransactionType: CommandId,
    Amount: u32,
    PartyA: &'a str,
    PartyB: &'a str,
    PhoneNumber: &'a str,
    CallBackURL: &'a str,
    AccountReference: &'a str,
    TransactionDesc: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MpesaExpressRequestResponse {
    CheckoutRequestID: String,
    CustomerMessage: String,
    MerchantRequestID: String,
    ResponseCode: String,
    ResponseDescription: String,
}

#[allow(dead_code)]
impl<'a> MpesaExpressRequestResponse {
    pub fn checkout_request_id(&'a self) -> &'a str {
        &self.CheckoutRequestID
    }

    pub fn customer_message(&'a self) -> &'a str {
        &self.CustomerMessage
    }

    pub fn merchant_request_id(&'a self) -> &'a str {
        &self.MerchantRequestID
    }

    pub fn response_code(&'a self) -> &'a str {
        &self.ResponseDescription
    }
}

pub struct MpesaExpressRequestBuilder<'a> {
    business_short_code: &'a str,
    client: &'a Mpesa,
    transaction_type: Option<CommandId>,
    amount: Option<u32>,
    party_a: Option<&'a str>,
    party_b: Option<&'a str>,
    phone_number: Option<&'a str>,
    callback_url: Option<&'a str>,
    account_ref: Option<&'a str>,
    transaction_desc: Option<&'a str>,
    pass_key: Option<&'a str>,
}

impl<'a> MpesaExpressRequestBuilder<'a> {
    pub fn new(client: &'a Mpesa, business_short_code: &'a str) -> MpesaExpressRequestBuilder<'a> {
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
    pub fn business_short_code(&'a self) -> &'a str {
        self.business_short_code
    }

    /// Retrieves the production passkey if present or defaults to the key provided in Safaricom's [test credentials](https://developer.safaricom.co.ke/test_credentials)
    fn get_pass_key(&'a self) -> &'a str {
        if let Some(key) = self.pass_key {
            return key;
        }
        "bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919"
    }

    /// Utility method to generate base64 encoded password as per Safaricom's [specifications](https://developer.safaricom.co.ke/docs#lipa-na-m-pesa-online-payment)
    /// Returns the encoded password and a timestamp string
    fn generate_password_and_timestamp(&self) -> (String, String) {
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let encoded_password = base64::encode(
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
    pub fn pass_key(mut self, pass_key: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.pass_key = Some(pass_key);
        self
    }

    /// Amount to be transacted
    ///
    /// # Errors
    /// If `amount` is invalid
    pub fn amount(mut self, amount: u32) -> MpesaExpressRequestBuilder<'a> {
        self.amount = Some(amount);
        self
    }

    /// The MSISDN sending the funds
    ///
    /// # Errors
    /// If `phone_number` is invalid
    pub fn phone_number(mut self, phone_number: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.phone_number = Some(phone_number);
        self
    }

    /// The url to where responses from M-Pesa will be sent to.
    ///
    /// # Errors
    /// If the `callback_url` is invalid
    pub fn callback_url(mut self, callback_url: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.callback_url = Some(callback_url);
        self
    }

    /// The MSISDN sending the funds
    ///
    /// # Errors
    /// If `party_a` is invalid
    pub fn party_a(mut self, party_a: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.party_a = Some(party_a);
        self
    }

    /// The organization shortcode receiving the funds
    ///
    /// # Errors
    /// If `party_b` is invalid
    pub fn party_b(mut self, party_b: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.party_b = Some(party_b);
        self
    }

    /// Optional - Used with M-Pesa PayBills.
    pub fn account_ref(mut self, account_ref: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.account_ref = Some(account_ref);
        self
    }

    /// Optional, defaults to `CommandId::CustomerPayBillOnline`
    ///
    /// # Errors
    /// If the `CommandId` is invalid
    pub fn transaction_type(mut self, command_id: CommandId) -> MpesaExpressRequestBuilder<'a> {
        self.transaction_type = Some(command_id);
        self
    }

    /// A description of the transaction.
    /// Optional - defaults to "None"
    pub fn transaction_desc(mut self, description: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.transaction_desc = Some(description);
        self
    }

    /// *Lipa na M-Pesa Online Payment / Mpesa Express/ Stk push*
    ///
    /// Initiates a M-Pesa transaction on behalf of a customer using STK Push
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub fn send(self) -> MpesaResult<MpesaExpressRequestResponse> {
        let url = format!(
            "{}/mpesa/stkpush/v1/processrequest",
            self.client.environment().base_url()
        );

        let (password, timestamp) = self.generate_password_and_timestamp();

        let payload = MpesaExpressRequestPayload {
            BusinessShortCode: self.business_short_code,
            Password: &password,
            Timestamp: &timestamp,
            Amount: self.amount.unwrap_or(10),
            PartyA: self.party_a.unwrap_or(self.business_short_code),
            PartyB: self.party_b.unwrap_or(self.phone_number.unwrap_or("None")),
            PhoneNumber: self.phone_number.unwrap_or("None"),
            CallBackURL: self.callback_url.unwrap_or("None"),
            AccountReference: self.account_ref.unwrap_or("None"),
            TransactionType: self
                .transaction_type
                .unwrap_or(CommandId::CustomerPayBillOnline),
            TransactionDesc: self.transaction_desc.unwrap_or("None"),
        };

        let response = Client::new()
            .post(&url)
            .bearer_auth(self.client.auth()?)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            let value: MpesaExpressRequestResponse = response.json()?;
            return Ok(value);
        }

        let value: Value = response.json()?;
        Err(MpesaError::MpesaExpressRequestError(value))
    }
}

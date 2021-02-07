use crate::client::{Mpesa, MpesaResult};
use crate::constants::CommandId;
use crate::errors::MpesaError;
use chrono::prelude::Local;
use reqwest::blocking::Client;
use serde::Serialize;
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

    pub fn business_short_code(&'a self) -> &'a str {
        self.business_short_code
    }

    fn get_pass_key(&'a self) -> &'a str {
        if let Some(key) = self.pass_key {
            return key;
        }
        "bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919"
    }

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

    pub fn pass_key(mut self, pass_key: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.pass_key = Some(pass_key);
        self
    }

    pub fn amount(mut self, amount: u32) -> MpesaExpressRequestBuilder<'a> {
        self.amount = Some(amount);
        self
    }

    pub fn phone_number(mut self, phone_number: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.phone_number = Some(phone_number);
        self
    }

    pub fn callback_url(mut self, callback_url: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.callback_url = Some(callback_url);
        self
    }

    pub fn party_a(mut self, party_a: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.party_a = Some(party_a);
        self
    }

    pub fn party_b(mut self, party_b: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.party_b = Some(party_b);
        self
    }

    pub fn account_ref(mut self, account_ref: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.account_ref = Some(account_ref);
        self
    }

    pub fn transaction_type(mut self, command_id: CommandId) -> MpesaExpressRequestBuilder<'a> {
        self.transaction_type = Some(command_id);
        self
    }

    pub fn transaction_desc(mut self, description: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.transaction_desc = Some(description);
        self
    }

    pub fn send(self) -> MpesaResult<Value> {
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
            let value: Value = response.json()?;
            return Ok(value);
        }

        let value: Value = response.json()?;
        Err(MpesaError::MpesaExpressRequestError(value))
    }
}

use crate::client::{Mpesa, MpesaResult};
use crate::constants::CommandId;
use crate::errors::MpesaError;
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
    TransactionDesc: &'a str
}

pub struct MpesaExpressRequestBuilder<'a> {
    business_short_code: Option<&'a str>,
    client: &'a Mpesa,
    transaction_type: Option<CommandId>,
    amount: Option<u32>,
    party_a: Option<&'a str>,
    party_b: Option<&'a str>,
    phone_number: Option<&'a str>,
    callback_url: Option<&'a str>,
    account_ref: Option<&'a str>,
    transaction_desc: Option<&'a str>,
}

impl <'a> MpesaExpressRequestBuilder<'a> {
    pub fn new(client: &'a Mpesa) -> MpesaExpressRequestBuilder<'a> {
        MpesaExpressRequestBuilder {
            client,
            business_short_code: None,
            transaction_type: None,
            transaction_desc: None,
            amount: None,
            party_a: None,
            party_b: None,
            phone_number: None,
            callback_url: None,
            account_ref: None
        }
    }

    pub fn business_short_code(mut self, short_code: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.business_short_code = Some(short_code);
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

    pub fn parties(mut self, party_a: &'a str, party_b: &'a str) -> MpesaExpressRequestBuilder<'a> {
        self.party_a = Some(party_a);
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
        self.transaction_desc = description;
        self
    }

    pub fn send(self) -> MpesaResult<Value> {
        let url = format!(
            "{}/mpesa/stkpush/v1/processsrequest",
            self.client.environment().base_url()
        );
    }
}
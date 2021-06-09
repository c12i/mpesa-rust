use crate::client::{Mpesa, MpesaResult};
use crate::constants::CommandId;
use crate::errors::MpesaError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
/// Payload to make payment requests from C2B.
/// See more: https://developer.safaricom.co.ke/docs#c2b-api
struct C2bSimulatePayload<'a> {
    CommandID: CommandId,
    Amount: u32,
    Msisdn: &'a str,
    BillRefNumber: &'a str,
    ShortCode: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct C2bSimulateResponse {
    ConversationID: String,
    OriginatorCoversationID: String,
    ResponseDescription: String,
}

#[allow(dead_code)]
impl<'a> C2bSimulateResponse {
    pub fn conversation_id(&'a self) -> &'a str {
        &self.ConversationID
    }

    pub fn originator_conversation_id(&'a self) -> &'a str {
        &self.OriginatorCoversationID
    }

    pub fn response_description(&'a self) -> &'a str {
        &self.ResponseDescription
    }
}

#[derive(Debug)]
pub struct C2bSimulateBuilder<'a> {
    client: &'a Mpesa,
    command_id: Option<CommandId>,
    amount: Option<u32>,
    msisdn: Option<&'a str>,
    bill_ref_number: Option<&'a str>,
    short_code: Option<&'a str>,
}

impl<'a> C2bSimulateBuilder<'a> {
    /// Creates a new C2B Simulate builder
    pub fn new(client: &'a Mpesa) -> C2bSimulateBuilder<'a> {
        C2bSimulateBuilder {
            client,
            command_id: None,
            amount: None,
            msisdn: None,
            bill_ref_number: None,
            short_code: None,
        }
    }

    /// Adds `CommandId`. Defaults to `CommandId::CustomerPaybillOnline` if no value explicitly passed
    ///
    /// # Errors
    /// If `CommandId` is not valid
    pub fn command_id(mut self, command_id: CommandId) -> C2bSimulateBuilder<'a> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds the amount being transacted. This is a required field
    ///
    /// # Errors
    /// If invalid amount, less than 10?
    pub fn amount(mut self, amount: u32) -> C2bSimulateBuilder<'a> {
        self.amount = Some(amount);
        self
    }

    /// Adds the MSISDN(phone number) sending the transaction, start by country code without the `+`.
    /// This is a required field
    ///
    /// # Errors
    /// If `MSISDN` is invalid
    pub fn msisdn(mut self, msisdn: &'a str) -> C2bSimulateBuilder<'a> {
        self.msisdn = Some(msisdn);
        self
    }

    /// Adds `ShortCode`; the 6 digit MPESA Till Number or PayBill Number
    ///
    /// # Errors
    /// If Till or PayBill number is invalid
    pub fn short_code(mut self, short_code: &'a str) -> C2bSimulateBuilder<'a> {
        self.short_code = Some(short_code);
        self
    }

    /// Adds Bull reference number. This field is optional and will by default be `"None"`.
    pub fn bill_ref_number(mut self, bill_ref_number: &'a str) -> C2bSimulateBuilder<'a> {
        self.bill_ref_number = Some(bill_ref_number);
        self
    }

    /// **C2B Simulate API**
    ///
    /// Make payment requests from Client to Business
    ///
    /// This enables you to receive the payment requests in real time.
    /// See more [here](https://developer.safaricom.co.ke/c2b/apis/post/simulate)
    ///
    /// A successful request returns a `serde_json::Value` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub fn send(self) -> MpesaResult<C2bSimulateResponse> {
        let url = format!(
            "{}/mpesa/c2b/v1/simulate",
            self.client.environment().base_url()
        );

        let payload = C2bSimulatePayload {
            CommandID: self.command_id.unwrap_or(CommandId::CustomerPayBillOnline),
            Amount: self.amount.unwrap_or(10),
            Msisdn: self.msisdn.unwrap_or("None"),
            BillRefNumber: self.bill_ref_number.unwrap_or("None"),
            ShortCode: self.short_code.unwrap_or("None"),
        };

        let response = Client::new()
            .post(&url)
            .bearer_auth(self.client.auth()?)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            let value: C2bSimulateResponse = response.json()?;
            return Ok(value);
        }

        let value: Value = response.json()?;
        Err(MpesaError::C2bSimulateError(value))
    }
}

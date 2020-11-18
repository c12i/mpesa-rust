use crate::client::{Mpesa, MpesaResult};
use crate::constants::{CommandId, IdentifierTypes};
use crate::errors::MpesaError;
use reqwest::blocking::Client;
use serde_json::{json, Value};

#[derive(Debug)]
/// Payload to make payment requests from C2B.
/// See more: https://developer.safaricom.co.ke/docs#c2b-api
struct C2bSimulatePayload<'a> {
    command_id: CommandId,
    amount: u32,
    msisdn: &'a str,
    bill_ref_number: &'a str,
    short_code: &'a str,
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

    /// # C2B Simulate API
    /// Make payment requests from Client to Business
    ///
    /// This enables you to receive the payment requests in real time.
    /// See more [here](https://developer.safaricom.co.ke/c2b/apis/post/simulate)
    pub fn send(self) -> MpesaResult<Value> {
        let url = format!(
            "{}/mpesa/c2b/v1/simulate",
            self.client.environment().base_url()
        );

        let payload = C2bSimulatePayload {
            command_id: self.command_id.unwrap_or(CommandId::CustomerPayBillOnline),
            amount: self.amount.unwrap_or(10),
            msisdn: self.msisdn.unwrap_or("None"),
            bill_ref_number: self.bill_ref_number.unwrap_or("None"),
            short_code: self.short_code.unwrap_or("None"),
        };

        let data = json!({
            "CommandID": payload.command_id.to_string(),
            "Amount": payload.amount,
            "Msisdn": payload.msisdn,
            "BillRefNumber": payload.bill_ref_number,
            "ShortCode": payload.short_code,
        });

        let response = Client::new()
            .post(&url)
            .bearer_auth(self.client.auth()?)
            .json(&data)
            .send()?;

        if response.status().is_success() {
            let value: Value = response.json()?;
            return Ok(value);
        }

        let value: Value = response.json()?;
        Err(MpesaError::ErrorResponse(value))
    }
}

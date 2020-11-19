use crate::client::MpesaResult;
use crate::{CommandId, Mpesa, MpesaError, MpesaSecurity};
use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Serialize)]
/// Payload to allow for b2c transactions:
struct B2cPayload<'a> {
    initiator_name: &'a str,
    security_credentials: &'a str,
    command_id: CommandId,
    amount: u32,
    party_a: &'a str,
    party_b: &'a str,
    remarks: &'a str,
    queue_timeout_url: &'a str,
    result_url: &'a str,
    occasion: &'a str,
}

#[derive(Debug)]
/// B2C transaction builder struct
pub struct B2cBuilder<'a> {
    initiator_name: &'a str,
    client: &'a Mpesa,
    command_id: Option<CommandId>,
    amount: Option<u32>,
    party_a: Option<&'a str>,
    party_b: Option<&'a str>,
    remarks: Option<&'a str>,
    queue_timeout_url: Option<&'a str>,
    result_url: Option<&'a str>,
    occasion: Option<&'a str>,
}

impl<'a> B2cBuilder<'a> {
    /// Create a new B2C builder.
    /// Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
    pub fn new(client: &'a Mpesa, initiator_name: &'a str) -> B2cBuilder<'a> {
        B2cBuilder {
            client,
            initiator_name,
            amount: None,
            party_a: None,
            party_b: None,
            remarks: None,
            queue_timeout_url: None,
            result_url: None,
            occasion: None,
            command_id: None,
        }
    }

    /// Adds the `CommandId`. Defaults to `CommandId::BusinessPayment` if not explicitly provided.
    pub fn command_id(mut self, command_id: CommandId) -> B2cBuilder<'a> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds `Party A` and `Party B`. Both are required fields
    /// `Party A` should be a paybill number while `Party B` should be a mobile number.
    ///
    /// # Errors
    /// If either `Party A` or `Party B` is invalid or not provided
    pub fn parties(mut self, party_a: &'a str, party_b: &'a str) -> B2cBuilder<'a> {
        // TODO: add validation
        self.party_a = Some(party_a);
        self.party_b = Some(party_b);
        self
    }

    /// Adds `Remarks`. This is an optional field, will default to "None" if not explicitly provided
    pub fn remarks(mut self, remarks: &'a str) -> B2cBuilder<'a> {
        self.remarks = Some(remarks);
        self
    }

    /// Adds `Occasion`. This is an optional field, will default to an empty string
    pub fn occasion(mut self, occasion: &'a str) -> B2cBuilder<'a> {
        self.occasion = Some(occasion);
        self
    }

    /// This is a required field
    ///
    /// # Errors
    /// If the amount is less than 10?
    pub fn amount(mut self, amount: u32) -> B2cBuilder<'a> {
        self.amount = Some(amount);
        self
    }

    /// Adds `QueueTimeoutUrl` and `ResultUrl`. This is a required field
    ///
    /// # Error
    /// If either `QueueTimeoutUrl` and `ResultUrl` is invalid or not provided
    pub fn urls(mut self, timeout_url: &'a str, result_url: &'a str) -> B2cBuilder<'a> {
        // TODO: validate urls; will probably return a `Result` from this
        self.queue_timeout_url = Some(timeout_url);
        self.result_url = Some(result_url);
        self
    }

    ///# B2C API
    /// Sends b2c payment request.
    ///
    /// This API enables Business to Customer (B2C) transactions between a company and
    /// customers who are the end-users of its products or services. Use of this API requires a
    /// valid and verified B2C M-Pesa Short code.
    /// See more [here](https://developer.safaricom.co.ke/docs?shell#b2c-api)
    ///
    /// A successful request returns a `serde_json::Value` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure.
    pub fn send(self) -> MpesaResult<Value> {
        let url = format!(
            "{}/mpesa/b2c/v1/paymentrequest",
            self.client.environment().base_url()
        );
        let credentials = self.client.gen_security_credentials()?;

        let payload = B2cPayload {
            initiator_name: self.initiator_name,
            security_credentials: &credentials,
            command_id: self.command_id.unwrap_or(CommandId::BusinessPayment),
            amount: self.amount.unwrap_or(10),
            party_a: self.party_a.unwrap_or("None"),
            party_b: self.party_b.unwrap_or("None"),
            remarks: self.remarks.unwrap_or("None"),
            queue_timeout_url: self.queue_timeout_url.unwrap_or("None"),
            result_url: self.result_url.unwrap_or("None"),
            occasion: self.occasion.unwrap_or("None"),
        };

        let data = json!({
            "InitiatorName": payload.initiator_name,
            "SecurityCredential": payload.security_credentials,
            "CommandID": payload.command_id.to_string(),
            "Amount": payload.amount,
            "PartyA": payload.party_a,
            "PartyB": payload.party_b,
            "Remarks": payload.remarks,
            "QueueTimeOutURL": payload.queue_timeout_url,
            "ResultURL": payload.result_url,
            "Occasion": payload.occasion,
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

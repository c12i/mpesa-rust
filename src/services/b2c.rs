use crate::client::MpesaResult;
use crate::{CommandId, Mpesa, MpesaError, MpesaSecurity};
use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Serialize)]
/// Payload to allow for b2c transactions:
/// See https://developer.safaricom.co.ke/docs#b2c-api for a
/// detailed description of each field.
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
    /// Create a new B2C builder
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

    /// Adds the `CommandId`
    pub fn command_id(mut self, commandId: CommandId) -> B2cBuilder<'a> {
        self.command_id = Some(commandId);
        self
    }

    /// Adds `Party A` and `Party B`
    pub fn parties(mut self, party_a: &'a str, party_b: &'a str) -> B2cBuilder<'a> {
        // TODO: add validation
        self.party_a = Some(party_a);
        self.party_b = Some(party_b);
        self
    }

    /// Adds `Remarks`
    pub fn remarks(mut self, remarks: &'a str) -> B2cBuilder<'a> {
        self.remarks = Some(remarks);
        self
    }

    /// Adds `Occasion`
    pub fn occasion(mut self, occasion: &'a str) -> B2cBuilder<'a> {
        self.occasion = Some(occasion);
        self
    }

    pub fn amount(mut self, amount: u32) -> B2cBuilder<'a> {
        self.amount = Some(amount);
        self
    }

    /// Adds `QueueTimeoutUrl` and `ResultUrl`
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
    /// See more at: https://developer.safaricom.co.ke/docs?shell#b2c-api
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
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
            party_a: self.party_a.unwrap_or(""),
            party_b: self.party_b.unwrap_or(""),
            remarks: self.remarks.unwrap_or("None"),
            queue_timeout_url: self.queue_timeout_url.unwrap_or(""),
            result_url: self.result_url.unwrap_or(""),
            occasion: self.occasion.unwrap_or(""),
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

        if response.status() == 200 {
            let value: Value = response.json()?;
            return Ok(value);
        }
        let value: Value = response.json()?;
        Err(MpesaError::ErrorResponse(value))
    }
}

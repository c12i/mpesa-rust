use crate::constants::{IdentifierTypes,CommandId};
use crate::client::{Mpesa, MpesaResult};
use crate::errors::MpesaError;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;
use crate::MpesaSecurity;

#[derive(Debug)]
pub struct B2bPayload<'a> {
    initiator_name: &'a str,
    security_credentials: &'a str,
    command_id: CommandId,
    amount: u32,
    party_a: &'a str,
    sender_id: u32,
    party_b: &'a str,
    receiver_id: u32,
    remarks: &'a str,
    queue_timeout_url: &'a str,
    result_url: &'a str,
    account_ref: &'a str,
}

#[derive(Debug)]
/// B2B transaction builder struct
pub struct B2bBuilder<'a> {
    initiator_name: &'a str,
    client: &'a Mpesa,
    command_id: Option<CommandId>,
    amount: Option<u32>,
    party_a: Option<&'a str>,
    sender_id: Option<IdentifierTypes>,
    party_b: Option<&'a str>,
    receiver_id: Option<IdentifierTypes>,
    remarks: Option<&'a str>,
    queue_timeout_url: Option<&'a str>,
    result_url: Option<&'a str>,
    account_ref: Option<&'a str>,
}

impl<'a> B2bBuilder {
    /// Creates a new B2B builder
    /// Requires a `initiator_name`
    pub fn new(client: &'a Mpesa, initiator_name: &'a str) -> B2bBuilder<'a> {
        B2bBuilder {
            client,
            initiator_name,
            amount: None,
            party_a: None,
            sender_id: None,
            party_b: None,
            receiver_id: None,
            remarks: None,
            queue_timeout_url: None,
            result_url: None,
            command_id: None,
            account_ref: None
        }
    }

    /// Adds the `CommandId`. Defaults to `CommandId::BusinessToBusinessTransfer` if not explicitly provided.
    pub fn command_id(mut self, command_id: CommandId) -> B2bBuilder<'a> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds `Party A` and `Party B`. Both are required fields
    /// `Party A` should be a paybill number while `Party B` should be a mobile number.
    ///
    /// # Errors
    /// If either `Party A` or `Party B` is invalid or not provided
    pub fn parties(mut self, party_a: &'a str, party_b: &'a str) -> B2bBuilder<'a> {
        self.party_a = Some(party_a);
        self.party_b = Some(party_b);
        self
    }

    /// Adds `QueueTimeoutUrl` and `ResultUrl`. This is a required field
    ///
    /// # Error
    /// If either `QueueTimeoutUrl` and `ResultUrl` is invalid or not provided
    pub fn urls(mut self, timeout_url: &'a str, result_url: &'a str) -> B2bBuilder<'a> {
        // TODO: validate urls
        self.queue_timeout_url = Some(timeout_url);
        self.result_url = Some(result_url);
        self
    }

    /// Adds `sender_id`. Will default to `IdentifierTypes::ShortCode` if not explicitly provided
    pub fn sender_id(mut self, sender_id: IdentifierTypes) -> B2bBuilder<'a> {
        self.sender_id = Some(sender_id);
        self
    }

    /// Adds `receiver_id`. Will default to `IdentifierTypes::ShortCode` if not explicitly provided
    pub fn receiver_id(mut self, receiver_id: IdentifierTypes) -> B2bBuilder<'a> {
        self.receiver_id = Some(receiver_id);
        self
    }

    /// Adds `account_ref`. This field is required
    pub fn account_ref(mut self, account_ref: &'a str) -> B2bBuilder<'a> {
        // TODO: add validation
        self.account_ref = Some(account_ref);
        self
    }

    /// This is a required field
    ///
    /// # Errors
    /// If the amount is less than 10?
    pub fn amount(mut self, amount: u32) -> B2bBuilder<'a> {
        self.amount = Some(amount);
        self
    }

    /// Adds `remarks`. This field is optional, will default to empty string if not explicitly passed
    pub fn remarks(mut self, remarks: &'a str) -> B2bBuilder<'a> {
        self.remarks = Some(remarks);
        self
    }

    /// # B2B API
    /// Sends b2b payment request.
    ///
    /// This API enables Business to Business (B2B) transactions between a business and another
    /// business. Use of this API requires a valid and verified B2B M-Pesa short code for the
    /// business initiating the transaction and the both businesses involved in the transaction
    /// See more at https://developer.safaricom.co.ke/docs?shell#b2b-api
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub fn send(self) -> MpesaResult<Value> {
        let url = format!(
            "{}/mpesa/b2b/v1/paymentrequest",
            self.environment.base_url()
        );
        let credentials = self.client.gen_security_credentials()?;

        let payload = B2bPayload {
            initiator_name: self.initiator_name,
            security_credentials: &credentials,
            command_id: self.command_id.unwrap_or(CommandId::BusinessToBusinessTransfer),
            amount: self.amount.unwrap_or(10),
            party_a: self.party_a.unwrap_or(""),
            sender_id: self.sender_id.unwrap_or(IdentifierTypes::Shortcode).get_code(),
            party_b: self.party_b.unwrap_or(""),
            receiver_id: self.receiver_id.unwrap_or(IdentifierTypes::Shortcode).get_code(),
            remarks: self.remarks.unwrap_or(""),
            queue_timeout_url: self.queue_timeout_url.unwrap_or(""),
            result_url: self.result_url.unwrap_or(""),
            account_ref: self.account_ref.unwrap_or(""),
        };

        let data = json!({
            "Initiator": payload.initiator_name,
            "SecurityCredential": payload.security_credentials,
            "CommandID": payload.command_id.to_string(),
            "SenderIdentifierType": payload.sender_id,
            "RecieverIdentifierType": payload.receiver_id,
            "Amount": payload.amount,
            "PartyA": payload.party_a,
            "PartyB": payload.party_b,
            "AccountReference": payload.account_ref,
            "Remarks": payload.remarks,
            "QueueTimeOutURL": payload.queue_timeout_url,
            "ResultURL": payload.result_url,
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

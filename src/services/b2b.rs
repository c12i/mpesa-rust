use crate::client::{Mpesa, MpesaResult};
use crate::constants::{CommandId, IdentifierTypes};
use crate::errors::MpesaError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
struct B2bPayload<'a> {
    #[serde(rename(serialize = "Initiator"))]
    initiator: &'a str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credential: &'a str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "Amount"))]
    amount: u32,
    #[serde(rename(serialize = "PartyA"), skip_serializing_if = "Option::is_none")]
    party_a: Option<&'a str>,
    #[serde(rename(serialize = "SenderIdentifierType"))]
    sender_identifier_type: &'a str,
    #[serde(rename(serialize = "PartyB"), skip_serializing_if = "Option::is_none")]
    party_b: Option<&'a str>,
    #[serde(rename(serialize = "RecieverIdentifierType"))]
    reciever_identifier_type: &'a str,
    #[serde(rename(serialize = "Remarks"))]
    remarks: &'a str,
    #[serde(
        rename(serialize = "QueueTimeOutURL"),
        skip_serializing_if = "Option::is_none"
    )]
    queue_time_out_url: Option<&'a str>,
    #[serde(
        rename(serialize = "ResultURL"),
        skip_serializing_if = "Option::is_none"
    )]
    result_url: Option<&'a str>,
    #[serde(
        rename(serialize = "AccountReference"),
        skip_serializing_if = "Option::is_none"
    )]
    account_reference: Option<&'a str>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct B2bResponse {
    #[serde(rename(deserialize = "ConversationID"))]
    pub conversation_id: String,
    #[serde(rename(deserialize = "OriginatorConversationID"))]
    pub originator_conversation_id: String,
    #[serde(rename(deserialize = "ResponseCode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    pub response_description: String,
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

impl<'a> B2bBuilder<'a> {
    /// Creates a new B2B builder
    /// Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
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
            account_ref: None,
        }
    }

    /// Adds the `CommandId`. Defaults to `CommandId::BusinessToBusinessTransfer` if not explicitly provided.
    ///
    /// # Errors
    /// If invalid `CommandId` is provided
    pub fn command_id(mut self, command_id: CommandId) -> B2bBuilder<'a> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds `Party A` which is a required field
    /// `Party A` should be a paybill number.
    ///
    /// # Errors
    /// If `Party A` is invalid or not provided
    pub fn party_a(mut self, party_a: &'a str) -> B2bBuilder<'a> {
        self.party_a = Some(party_a);
        self
    }

    /// Adds `Party B` which is a required field
    /// `Party B` should be a mobile number.
    ///
    /// # Errors
    /// If `Party B` is invalid or not provided
    pub fn party_b(mut self, party_b: &'a str) -> B2bBuilder<'a> {
        self.party_b = Some(party_b);
        self
    }

    /// Adds `Party A` and `Party B`. Both are required fields
    /// `Party A` should be a paybill number while `Party B` should be a mobile number.
    ///
    /// # Errors
    /// If either `Party A` or `Party B` is invalid or not provided
    #[deprecated]
    pub fn parties(mut self, party_a: &'a str, party_b: &'a str) -> B2bBuilder<'a> {
        self.party_a = Some(party_a);
        self.party_b = Some(party_b);
        self
    }

    // Adds `QueueTimeoutUrl` This is a required field
    ///
    /// # Error
    /// If `QueueTimeoutUrl` is invalid or not provided
    pub fn timeout_url(mut self, timeout_url: &'a str) -> B2bBuilder<'a> {
        self.queue_timeout_url = Some(timeout_url);
        self
    }

    // Adds `ResultUrl` This is a required field
    ///
    /// # Error
    /// If `ResultUrl` is invalid or not provided
    pub fn result_url(mut self, result_url: &'a str) -> B2bBuilder<'a> {
        self.result_url = Some(result_url);
        self
    }

    /// Adds `QueueTimeoutUrl` and `ResultUrl`. This is a required field
    ///
    /// # Error
    /// If either `QueueTimeoutUrl` and `ResultUrl` is invalid or not provided
    #[deprecated]
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

    /// Adds `remarks`. This field is optional, will default to "None" if not explicitly passed
    pub fn remarks(mut self, remarks: &'a str) -> B2bBuilder<'a> {
        self.remarks = Some(remarks);
        self
    }

    /// **B2B API**
    ///
    /// Sends b2b payment request.
    ///
    /// This API enables Business to Business (B2B) transactions between a business and another
    /// business. Use of this API requires a valid and verified B2B M-Pesa short code for the
    /// business initiating the transaction and the both businesses involved in the transaction
    /// See more [here](https://developer.safaricom.co.ke/docs?shell#b2b-api)
    ///
    /// A successful request returns a `serde_json::Value` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub fn send(self) -> MpesaResult<B2bResponse> {
        let url = format!(
            "{}/mpesa/b2b/v1/paymentrequest",
            self.client.environment().base_url()
        );
        let credentials = self.client.gen_security_credentials()?;

        let payload = B2bPayload {
            initiator: self.initiator_name,
            security_credential: &credentials,
            command_id: self
                .command_id
                .unwrap_or_else(|| CommandId::BusinessToBusinessTransfer),
            // TODO: Can this be improved?
            amount: self.amount.unwrap_or_default(),
            party_a: self.party_a,
            sender_identifier_type: &self
                .sender_id
                .unwrap_or_else(|| IdentifierTypes::ShortCode)
                .to_string(),
            party_b: self.party_b,
            reciever_identifier_type: &self
                .receiver_id
                .unwrap_or_else(|| IdentifierTypes::ShortCode)
                .to_string(),
            remarks: self.remarks.unwrap_or_else(|| "None"),
            queue_time_out_url: self.queue_timeout_url,
            result_url: self.result_url,
            account_reference: self.account_ref,
        };

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth()?)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            let value: B2bResponse = response.json()?;
            return Ok(value);
        }

        let value: Value = response.json()?;
        Err(MpesaError::B2bError(value))
    }
}

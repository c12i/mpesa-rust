#![doc = include_str!("../../docs/client/b2b.md")]

use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::constants::{CommandId, IdentifierTypes};
use crate::errors::{MpesaError, MpesaResult};

const B2B_URL: &str = "mpesa/b2b/v1/paymentrequest";

#[derive(Debug, Serialize)]
struct B2bPayload<'mpesa> {
    #[serde(rename(serialize = "Initiator"))]
    initiator: &'mpesa str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credential: &'mpesa str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "Amount"))]
    amount: f64,
    #[serde(rename(serialize = "PartyA"))]
    party_a: &'mpesa str,
    #[serde(rename(serialize = "SenderIdentifierType"))]
    sender_identifier_type: &'mpesa str,
    #[serde(rename(serialize = "PartyB"))]
    party_b: &'mpesa str,
    #[serde(rename(serialize = "RecieverIdentifierType"))]
    reciever_identifier_type: &'mpesa str,
    #[serde(rename(serialize = "Remarks"))]
    remarks: &'mpesa str,
    #[serde(
        rename(serialize = "QueueTimeOutURL"),
        skip_serializing_if = "Option::is_none"
    )]
    queue_time_out_url: Option<&'mpesa str>,
    #[serde(
        rename(serialize = "ResultURL"),
        skip_serializing_if = "Option::is_none"
    )]
    result_url: Option<&'mpesa str>,
    #[serde(
        rename(serialize = "AccountReference"),
        skip_serializing_if = "Option::is_none"
    )]
    account_reference: Option<&'mpesa str>,
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
pub struct B2bBuilder<'mpesa> {
    initiator_name: &'mpesa str,
    client: &'mpesa Mpesa,
    command_id: Option<CommandId>,
    amount: Option<f64>,
    party_a: Option<&'mpesa str>,
    sender_id: Option<IdentifierTypes>,
    party_b: Option<&'mpesa str>,
    receiver_id: Option<IdentifierTypes>,
    remarks: Option<&'mpesa str>,
    queue_timeout_url: Option<&'mpesa str>,
    result_url: Option<&'mpesa str>,
    account_ref: Option<&'mpesa str>,
}

impl<'mpesa> B2bBuilder<'mpesa> {
    /// Creates a new B2B builder
    /// Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
    pub fn new(client: &'mpesa Mpesa, initiator_name: &'mpesa str) -> B2bBuilder<'mpesa> {
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
    pub fn command_id(mut self, command_id: CommandId) -> B2bBuilder<'mpesa> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds `Party A` which is a required field
    /// `Party A` should be a paybill number.
    ///
    /// # Errors
    /// If `Party A` is invalid or not provided
    pub fn party_a(mut self, party_a: &'mpesa str) -> B2bBuilder<'mpesa> {
        self.party_a = Some(party_a);
        self
    }

    /// Adds `Party B` which is a required field
    /// `Party B` should be a mobile number.
    ///
    /// # Errors
    /// If `Party B` is invalid or not provided
    pub fn party_b(mut self, party_b: &'mpesa str) -> B2bBuilder<'mpesa> {
        self.party_b = Some(party_b);
        self
    }

    /// Adds `Party A` and `Party B`. Both are required fields
    /// `Party A` should be a paybill number while `Party B` should be a mobile number.
    ///
    /// # Errors
    /// If either `Party A` or `Party B` is invalid or not provided
    #[deprecated]
    pub fn parties(mut self, party_a: &'mpesa str, party_b: &'mpesa str) -> B2bBuilder<'mpesa> {
        self.party_a = Some(party_a);
        self.party_b = Some(party_b);
        self
    }

    // Adds `QueueTimeoutUrl` This is a required field
    ///
    /// # Error
    /// If `QueueTimeoutUrl` is invalid or not provided
    pub fn timeout_url(mut self, timeout_url: &'mpesa str) -> B2bBuilder<'mpesa> {
        self.queue_timeout_url = Some(timeout_url);
        self
    }

    // Adds `ResultUrl` This is a required field
    ///
    /// # Error
    /// If `ResultUrl` is invalid or not provided
    pub fn result_url(mut self, result_url: &'mpesa str) -> B2bBuilder<'mpesa> {
        self.result_url = Some(result_url);
        self
    }

    /// Adds `QueueTimeoutUrl` and `ResultUrl`. This is a required field
    ///
    /// # Error
    /// If either `QueueTimeoutUrl` and `ResultUrl` is invalid or not provided
    #[deprecated]
    pub fn urls(mut self, timeout_url: &'mpesa str, result_url: &'mpesa str) -> B2bBuilder<'mpesa> {
        // TODO: validate urls
        self.queue_timeout_url = Some(timeout_url);
        self.result_url = Some(result_url);
        self
    }

    /// Adds `sender_id`. Will default to `IdentifierTypes::ShortCode` if not explicitly provided
    pub fn sender_id(mut self, sender_id: IdentifierTypes) -> B2bBuilder<'mpesa> {
        self.sender_id = Some(sender_id);
        self
    }

    /// Adds `receiver_id`. Will default to `IdentifierTypes::ShortCode` if not explicitly provided
    pub fn receiver_id(mut self, receiver_id: IdentifierTypes) -> B2bBuilder<'mpesa> {
        self.receiver_id = Some(receiver_id);
        self
    }

    /// Adds `account_ref`. This field is required
    pub fn account_ref(mut self, account_ref: &'mpesa str) -> B2bBuilder<'mpesa> {
        // TODO: add validation
        self.account_ref = Some(account_ref);
        self
    }

    /// Adds an `amount` to the request
    /// This is a required field
    pub fn amount<Number: Into<f64>>(mut self, amount: Number) -> B2bBuilder<'mpesa> {
        self.amount = Some(amount.into());
        self
    }

    /// Adds `remarks`. This field is optional, will default to "None" if not explicitly passed
    pub fn remarks(mut self, remarks: &'mpesa str) -> B2bBuilder<'mpesa> {
        self.remarks = Some(remarks);
        self
    }

    /// # B2B API
    ///
    /// Sends b2b payment request.
    ///
    /// This API enables Business to Business (B2B) transactions between a business and another
    /// business. Use of this API requires a valid and verified B2B M-Pesa short code for the
    /// business initiating the transaction and the both businesses involved in the transaction
    /// See more [here](https://developer.safaricom.co.ke/docs?shell#b2b-api)
    ///
    /// A successful request returns a `B2bResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<B2bResponse> {
        let credentials = self.client.gen_security_credentials()?;

        let payload = B2bPayload {
            initiator: self.initiator_name,
            security_credential: &credentials,
            command_id: self
                .command_id
                .unwrap_or(CommandId::BusinessToBusinessTransfer),
            amount: self
                .amount
                .ok_or(MpesaError::Message("amount is required"))?,
            party_a: self
                .party_a
                .ok_or(MpesaError::Message("party_a is required"))?,
            sender_identifier_type: &self
                .sender_id
                .unwrap_or(IdentifierTypes::ShortCode)
                .to_string(),
            party_b: self
                .party_b
                .ok_or(MpesaError::Message("party_b is required"))?,
            reciever_identifier_type: &self
                .receiver_id
                .unwrap_or(IdentifierTypes::ShortCode)
                .to_string(),
            remarks: self.remarks.unwrap_or_else(|| stringify!(None)),
            queue_time_out_url: self.queue_timeout_url,
            result_url: self.result_url,
            account_reference: self.account_ref,
        };

        self.client
            .send(crate::client::Request {
                method: reqwest::Method::POST,
                path: B2B_URL,
                body: payload,
            })
            .await
    }
}

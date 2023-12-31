#![doc = include_str!("../../docs/client/transaction_status.md")]

use serde::{Deserialize, Serialize};

use crate::{CommandId, IdentifierTypes, Mpesa, MpesaError, MpesaResult};

const TRANSACTION_STATUS_URL: &str = "mpesa/transactionstatus/v1/query";

#[derive(Debug, Serialize)]
pub struct TransactionStatusPayload<'mpesa> {
    #[serde(rename(serialize = "Initiator"))]
    initiator: &'mpesa str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credentials: &'mpesa str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "TransactionID"))]
    transaction_id: &'mpesa str,
    #[serde(rename = "PartyA")]
    party_a: &'mpesa str,
    #[serde(rename(serialize = "IdentifierType"))]
    identifier_type: IdentifierTypes,
    #[serde(rename(serialize = "ResultURL"))]
    result_url: &'mpesa str,
    #[serde(rename(serialize = "QueueTimeOutURL"))]
    timeout_url: &'mpesa str,
    #[serde(rename(serialize = "Remarks"))]
    remarks: &'mpesa str,
    #[serde(rename(serialize = "Occasion"))]
    occasion: &'mpesa str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatusResponse {
    #[serde(rename(deserialize = "ConversationID"))]
    pub conversation_id: String,
    #[serde(rename(deserialize = "OriginatorConversationID"))]
    pub originator_conversation_id: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    pub response_description: String,
}

#[derive(Debug)]
pub struct TransactionStatusBuilder<'mpesa> {
    client: &'mpesa Mpesa,
    initiator: &'mpesa str,
    command_id: Option<CommandId>,
    transaction_id: Option<&'mpesa str>,
    party_a: Option<&'mpesa str>,
    identifier_type: Option<IdentifierTypes>,
    result_url: Option<&'mpesa str>,
    timeout_url: Option<&'mpesa str>,
    remarks: Option<&'mpesa str>,
    occasion: Option<&'mpesa str>,
}

impl<'mpesa> TransactionStatusBuilder<'mpesa> {
    /// Creates new `TransactionStatusBuilder`
    pub fn new(client: &'mpesa Mpesa, initiator: &'mpesa str) -> TransactionStatusBuilder<'mpesa> {
        TransactionStatusBuilder {
            client,
            initiator,
            command_id: None,
            transaction_id: None,
            party_a: None,
            identifier_type: None,
            result_url: None,
            timeout_url: None,
            remarks: None,
            occasion: None,
        }
    }

    /// Adds `CommandId`. Defaults to `CommandId::TransactionStatus` if no value explicitly passed
    ///
    /// # Errors
    /// If `CommandId` is not valid
    pub fn command_id(mut self, command_id: CommandId) -> Self {
        self.command_id = Some(command_id);
        self
    }

    /// Add the Mpesa Transaction ID of the transaction which you wish to reverse
    ///
    /// This is a required field.
    pub fn transaction_id(mut self, transaction_id: &'mpesa str) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    /// Organization receiving the transaction
    ///
    /// This is required field
    pub fn party_a(mut self, party_a: &'mpesa str) -> Self {
        self.party_a = Some(party_a);
        self
    }

    /// Type of organization receiving the transaction
    ///
    /// This is an optional field, defaults to `IdentifierTypes::ShortCode`
    pub fn identifier_type(mut self, identifier_type: IdentifierTypes) -> Self {
        self.identifier_type = Some(identifier_type);
        self
    }

    // Adds `ResultUrl` This is a required field
    ///
    /// # Error
    /// If `ResultUrl` is invalid or not provided
    pub fn result_url(mut self, result_url: &'mpesa str) -> Self {
        self.result_url = Some(result_url);
        self
    }

    /// Adds `QueueTimeoutUrl` and `ResultUrl`. This is a required field
    ///
    /// # Error
    /// If either `QueueTimeoutUrl` and `ResultUrl` is invalid or not provided
    pub fn timeout_url(mut self, timeout_url: &'mpesa str) -> Self {
        self.timeout_url = Some(timeout_url);
        self
    }

    /// Comments that are sent along with the transaction.
    ///
    /// This is an optional field, defaults to "None"
    pub fn remarks(mut self, remarks: &'mpesa str) -> Self {
        self.remarks = Some(remarks);
        self
    }

    /// Adds any additional information to be associated with the transaction.
    ///
    /// This is an optional Parameter, defaults to "None"
    pub fn occasion(mut self, occasion: &'mpesa str) -> Self {
        self.occasion = Some(occasion);
        self
    }

    /// # Transaction Status API
    ///
    /// Requests for the status of a transaction
    ///
    /// This API enables the status of a B2B, B2C or C2B M-Pesa transaction
    /// Required  parameters:
    ///
    /// `transaction_id`: This is the Mpesa Transaction ID of the transaction which you wish to reverse
    ///
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/Documentation)
    ///
    /// A successful request returns a `TransactionStatusResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure.
    pub async fn send(self) -> MpesaResult<TransactionStatusResponse> {
        let credentials = self.client.gen_security_credentials()?;

        let payload = TransactionStatusPayload {
            initiator: self.initiator,
            security_credentials: &credentials,
            command_id: self.command_id.unwrap_or(CommandId::TransactionStatusQuery),
            transaction_id: self
                .transaction_id
                .ok_or(MpesaError::Message("transaction_id is required"))?,
            party_a: self
                .party_a
                .ok_or(MpesaError::Message("party_a is required"))?,
            identifier_type: self.identifier_type.unwrap_or(IdentifierTypes::ShortCode),
            result_url: self
                .result_url
                .ok_or(MpesaError::Message("result_url is required"))?,
            timeout_url: self
                .timeout_url
                .ok_or(MpesaError::Message("timeout_url is required"))?,
            remarks: self.remarks.unwrap_or(stringify!(None)),
            occasion: self.occasion.unwrap_or(stringify!(None)),
        };

        self.client
            .send(crate::client::Request {
                method: reqwest::Method::POST,
                path: TRANSACTION_STATUS_URL,
                body: payload,
            })
            .await
    }
}

#![doc = include_str!("../../docs/client/transaction_reversal.md")]

use serde::{Deserialize, Serialize};

use crate::{CommandId, IdentifierTypes, Mpesa, MpesaError, MpesaResult};

const TRANSACTION_REVERSAL_URL: &str = "mpesa/reversal/v1/request";

#[derive(Debug, Serialize)]
pub struct TransactionReversalPayload<'mpesa> {
    #[serde(rename(serialize = "Initiator"))]
    initiator: &'mpesa str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credentials: &'mpesa str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "TransactionID"))]
    transaction_id: &'mpesa str,
    #[serde(rename(serialize = "ReceiverParty"))]
    receiver_party: &'mpesa str,
    #[serde(rename(serialize = "RecieverIdentifierType"))]
    receiver_identifier_type: IdentifierTypes,
    #[serde(rename(serialize = "ResultURL"))]
    result_url: &'mpesa str,
    #[serde(rename(serialize = "QueueTimeOutURL"))]
    timeout_url: &'mpesa str,
    #[serde(rename(serialize = "Remarks"))]
    remarks: &'mpesa str,
    #[serde(rename(serialize = "Occasion"))]
    occasion: &'mpesa str,
    #[serde(rename(serialize = "Amount"))]
    amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReversalResponse {
    #[serde(rename(deserialize = "ConversationID"))]
    pub conversation_id: String,
    #[serde(rename(deserialize = "OriginatorConversationID"))]
    pub originator_conversation_id: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    pub response_description: String,
}

#[derive(Debug)]
pub struct TransactionReversalBuilder<'mpesa> {
    client: &'mpesa Mpesa,
    initiator: &'mpesa str,
    command_id: Option<CommandId>,
    transaction_id: Option<&'mpesa str>,
    receiver_party: Option<&'mpesa str>,
    receiver_identifier_type: Option<IdentifierTypes>,
    result_url: Option<&'mpesa str>,
    timeout_url: Option<&'mpesa str>,
    remarks: Option<&'mpesa str>,
    occasion: Option<&'mpesa str>,
    amount: Option<f64>,
}

impl<'mpesa> TransactionReversalBuilder<'mpesa> {
    /// Creates new `TransactionReversalBuilder`
    pub fn new(
        client: &'mpesa Mpesa,
        initiator: &'mpesa str,
    ) -> TransactionReversalBuilder<'mpesa> {
        TransactionReversalBuilder {
            client,
            initiator,
            command_id: None,
            transaction_id: None,
            receiver_party: None,
            receiver_identifier_type: None,
            result_url: None,
            timeout_url: None,
            remarks: None,
            occasion: None,
            amount: None,
        }
    }

    /// Adds `CommandId`. Defaults to `CommandId::TransactionReversal` if no value explicitly passed
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
    pub fn receiver_party(mut self, receiver_party: &'mpesa str) -> Self {
        self.receiver_party = Some(receiver_party);
        self
    }

    /// Type of organization receiving the transaction
    ///
    /// This is an optional field, will default to `IdentifierTypes::ShortCode`
    pub fn receiver_identifier_type(mut self, receiver_identifier_type: IdentifierTypes) -> Self {
        self.receiver_identifier_type = Some(receiver_identifier_type);
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
    /// This is an optiona field; defaults to "None"
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

    /// Adds an `amount` to the request
    ///
    /// This is a required field
    pub fn amount<Number: Into<f64>>(mut self, amount: Number) -> Self {
        self.amount = Some(amount.into());
        self
    }

    /// # Transaction Reversal API
    ///
    /// Requests for transaction reversal
    ///
    /// This API enables reversal of a B2B, B2C or C2B M-Pesa transaction
    /// Required  parameters:
    ///
    /// `transaction_id`: This is the Mpesa Transaction ID of the transaction which you wish to reverse
    ///
    /// `amount` : The amount transacted in the transaction to be reversed , down to the cent
    ///
    /// `receiver_party`: Your organization's short code.
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/Documentation)
    ///
    /// A successful request returns a `TransactionReversalResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure.
    pub async fn send(self) -> MpesaResult<TransactionReversalResponse> {
        let credentials = self.client.gen_security_credentials()?;

        let payload = TransactionReversalPayload {
            initiator: self.initiator,
            security_credentials: &credentials,
            command_id: self.command_id.unwrap_or(CommandId::TransactionReversal),
            transaction_id: self
                .transaction_id
                .ok_or(MpesaError::Message("transaction_id is required"))?,
            receiver_party: self
                .receiver_party
                .ok_or(MpesaError::Message("receiver_party is required"))?,
            receiver_identifier_type: self
                .receiver_identifier_type
                .unwrap_or(IdentifierTypes::Reversal),
            result_url: self
                .result_url
                .ok_or(MpesaError::Message("result_url is required"))?,
            timeout_url: self
                .timeout_url
                .ok_or(MpesaError::Message("timeout_url is required"))?,
            remarks: self.remarks.unwrap_or(stringify!(None)),
            occasion: self.occasion.unwrap_or(stringify!(None)),
            amount: self
                .amount
                .ok_or(MpesaError::Message("amount is required"))?,
        };

        self.client
            .send(crate::client::Request {
                method: reqwest::Method::POST,
                path: TRANSACTION_REVERSAL_URL,
                body: payload,
            })
            .await
    }
}

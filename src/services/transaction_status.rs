use serde::Deserialize;
use serde::Serialize;

use crate::ApiEnvironment;
use crate::CommandId;
use crate::IdentifierTypes;
use crate::Mpesa;
use crate::MpesaError;
use crate::MpesaResult;

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
    party_a: Option<&'mpesa str>,
    #[serde(rename(serialize = "IdentifierType"))]
    identifier_type: Option<IdentifierTypes>,
    #[serde(rename(serialize = "ResultURL"))]
    result_url: Option<&'mpesa str>,
    #[serde(rename(serialize = "QueueTimeOutURL"))]
    timeout_url: Option<&'mpesa str>,
    #[serde(rename(serialize = "Remarks"))]
    remarks: Option<&'mpesa str>,
    #[serde(rename(serialize = "Occasion"))]
    occasion: Option<&'mpesa str>,
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
pub struct TransactionStatusBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
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

impl<'mpesa, Env: ApiEnvironment> TransactionStatusBuilder<'mpesa, Env> {
    /// Creates new `TransactionStatusBuilder`
    pub fn new(
        client: &'mpesa Mpesa<Env>,
        initiator: &'mpesa str,
    ) -> TransactionStatusBuilder<'mpesa, Env> {
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
    /// This is required field
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
    /// This is required field
    pub fn remarks(mut self, remarks: &'mpesa str) -> Self {
        self.remarks = Some(remarks);
        self
    }

    /// Adds any additional information to be associated with the transaction.
    /// This is an optional Parameter
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
        let url = format!(
            "{}/mpesa/transactionstatus/v1/query",
            self.client.environment().base_url()
        );

        let credentials = self.client.gen_security_credentials()?;

        let payload = TransactionStatusPayload {
            initiator: self.initiator,
            security_credentials: &credentials,
            command_id: self.command_id.unwrap_or(CommandId::TransactionStatusQuery),
            transaction_id: self
                .transaction_id
                .ok_or(MpesaError::Message("transaction_id is required field"))?,
            party_a: self.party_a,
            identifier_type: self.identifier_type,
            result_url: self.result_url,
            timeout_url: self.timeout_url,
            remarks: self.remarks,
            occasion: self.occasion,
        };

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let value = response.json().await?;
            return Err(MpesaError::MpesaTransactionStatusError(value));
        };

        let response = response.json::<_>().await?;
        Ok(response)
    }
}

use serde::Deserialize;
use serde::Serialize;

use crate::ApiEnvironment;
use crate::CommandId;
use crate::Mpesa;
use crate::MpesaError;
use crate::MpesaResult;

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
    #[serde(rename(serialize = "ReceiverIdentifierType"))]
    receiver_identifier_type: &'mpesa str,
    #[serde(rename(serialize = "ResultURL"))]
    result_url: &'mpesa str,
    #[serde(rename(serialize = "QueueTimeOutURL"))]
    queue_timeout_url: &'mpesa str,
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
    conversation_id: String,
    #[serde(rename(deserialize = "OriginatorConversationID"))]
    originator_conversation_id: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    response_description: String,
}

#[derive(Debug)]
pub struct TransactionReversalBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    initiator: Option<&'mpesa str>,
    security_credentials: Option<&'mpesa str>,
    command_id: Option<CommandId>,
    transaction_id: Option<&'mpesa str>,
    receiver_party: Option<&'mpesa str>,
    receiver_identifier_type: Option<&'mpesa str>,
    result_url: Option<&'mpesa str>,
    queue_timeout_url: Option<&'mpesa str>,
    remarks: Option<&'mpesa str>,
    occasion: Option<&'mpesa str>,
    amount: Option<f64>,
}

impl<'mpesa, Env: ApiEnvironment> TransactionReversalBuilder<'mpesa, Env> {
    pub fn new(
        client: &'mpesa Mpesa<Env>,
        transaction_id: &'mpesa str,
        amount: f64,
        receiver_party: &'mpesa str,
    ) -> TransactionReversalBuilder<'mpesa, Env> {
        TransactionReversalBuilder {
            client,
            initiator: None,
            security_credentials: None,
            command_id: None,
            transaction_id: Some(transaction_id),
            receiver_party: Some(receiver_party),
            receiver_identifier_type: None,
            result_url: None,
            queue_timeout_url: None,
            remarks: None,
            occasion: None,
            amount: Some(amount),
        }
    }

    pub fn command_id(mut self, command_id: CommandId) -> Self {
        self.command_id = Some(command_id);
        self
    }

    pub fn transaction_id(mut self, transaction_id: &'mpesa str) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    pub fn receiver_party(mut self, receiver_party: &'mpesa str) -> Self {
        self.receiver_party = Some(receiver_party);
        self
    }

    pub fn receiver_identifier_type(mut self, receiver_identifier_type: &'mpesa str) -> Self {
        self.receiver_identifier_type = Some(receiver_identifier_type);
        self
    }

    pub fn result_url(mut self, result_url: &'mpesa str) -> Self {
        self.result_url = Some(result_url);
        self
    }

    pub fn queue_timeout_url(mut self, queue_timeout_url: &'mpesa str) -> Self {
        self.queue_timeout_url = Some(queue_timeout_url);
        self
    }

    pub fn remarks(mut self, remarks: &'mpesa str) -> Self {
        self.remarks = Some(remarks);
        self
    }

    pub fn occasion(mut self, occasion: &'mpesa str) -> Self {
        self.occasion = Some(occasion);
        self
    }

    pub async fn send(&self) -> MpesaResult<TransactionReversalResponse> {
        let url = format!(
            "{}/reversal/v1/request",
            self.client.environment().base_url()
        );

        let payload = TransactionReversalPayload {
            initiator: self.initiator.unwrap(),
            security_credentials: self.security_credentials.unwrap(),
            command_id: self.command_id.clone().unwrap(),
            transaction_id: self.transaction_id.unwrap(),
            receiver_party: self.receiver_party.unwrap(),
            receiver_identifier_type: self.receiver_identifier_type.unwrap(),
            result_url: self.result_url.unwrap(),
            queue_timeout_url: self.queue_timeout_url.unwrap(),
            remarks: self.remarks.unwrap(),
            occasion: self.occasion.unwrap(),
            amount: self.amount.unwrap_or_default(),
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
            return Err(MpesaError::MpesaExpressRequestError(value));
        };

        let response = response.json::<_>().await?;
        Ok(response)
    }
}

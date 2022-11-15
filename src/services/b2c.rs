use crate::client::MpesaResult;
use crate::{CommandId, Mpesa, MpesaError};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
/// Payload to allow for b2c transactions:
struct B2cPayload<'a> {
    #[serde(rename(serialize = "InitiatorName"))]
    initiator_name: &'a str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credential: &'a str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "Amount"))]
    amount: u32,
    #[serde(rename(serialize = "PartyA"), skip_serializing_if = "Option::is_none")]
    party_a: Option<&'a str>,
    #[serde(rename(serialize = "PartyB"), skip_serializing_if = "Option::is_none")]
    party_b: Option<&'a str>,
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
        rename(serialize = "Occasion"),
        skip_serializing_if = "Option::is_none"
    )]
    occasion: Option<&'a str>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct B2cResponse {
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

    /// Adds `Party A` which is a required field
    /// `Party A` should be a paybill number.
    ///
    /// # Errors
    /// If `Party A` is invalid or not provided
    pub fn party_a(mut self, party_a: &'a str) -> B2cBuilder<'a> {
        self.party_a = Some(party_a);
        self
    }

    /// Adds `Party B` which is a required field
    /// `Party B` should be a mobile number.
    ///
    /// # Errors
    /// If `Party B` is invalid or not provided
    pub fn party_b(mut self, party_b: &'a str) -> B2cBuilder<'a> {
        self.party_b = Some(party_b);
        self
    }

    /// Adds `Party A` and `Party B`. Both are required fields
    /// `Party A` should be a paybill number while `Party B` should be a mobile number.
    ///
    /// # Errors
    /// If either `Party A` or `Party B` is invalid or not provided
    #[deprecated]
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

    /// Adds an `amount` to the request
    /// This is a required field
    pub fn amount(mut self, amount: u32) -> B2cBuilder<'a> {
        self.amount = Some(amount);
        self
    }

    // Adds `QueueTimeoutUrl` This is a required field
    ///
    /// # Error
    /// If `QueueTimeoutUrl` is invalid or not provided
    pub fn timeout_url(mut self, timeout_url: &'a str) -> B2cBuilder<'a> {
        self.queue_timeout_url = Some(timeout_url);
        self
    }

    // Adds `ResultUrl` This is a required field
    ///
    /// # Error
    /// If `ResultUrl` is invalid or not provided
    pub fn result_url(mut self, result_url: &'a str) -> B2cBuilder<'a> {
        self.result_url = Some(result_url);
        self
    }

    /// Adds `QueueTimeoutUrl` and `ResultUrl`. This is a required field
    ///
    /// # Error
    /// If either `QueueTimeoutUrl` and `ResultUrl` is invalid or not provided
    #[deprecated]
    pub fn urls(mut self, timeout_url: &'a str, result_url: &'a str) -> B2cBuilder<'a> {
        // TODO: validate urls; will probably return a `Result` from this
        self.queue_timeout_url = Some(timeout_url);
        self.result_url = Some(result_url);
        self
    }

    /// # B2C API
    ///
    /// Sends b2c payment request.
    ///
    /// This API enables Business to Customer (B2C) transactions between a company and
    /// customers who are the end-users of its products or services. Use of this API requires a
    /// valid and verified B2C M-Pesa Short code.
    /// See more [here](https://developer.safaricom.co.ke/docs?shell#b2c-api)
    ///
    /// A successful request returns a `B2cResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure.
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<B2cResponse> {
        let url = format!(
            "{}/mpesa/b2c/v1/paymentrequest",
            self.client.environment().base_url()
        );
        let credentials = self.client.gen_security_credentials()?;

        let payload = B2cPayload {
            initiator_name: self.initiator_name,
            security_credential: &credentials,
            command_id: self
                .command_id
                .unwrap_or_else(|| CommandId::BusinessPayment),
            amount: self.amount.unwrap_or_default(),
            party_a: self.party_a,
            party_b: self.party_b,
            remarks: self.remarks.unwrap_or_else(|| "None"),
            queue_time_out_url: self.queue_timeout_url,
            result_url: self.result_url,
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

        if response.status().is_success() {
            let value: B2cResponse = response.json().await?;
            return Ok(value);
        }

        let value: Value = response.json().await?;
        Err(MpesaError::B2cError(value))
    }
}

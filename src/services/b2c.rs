use serde::{Deserialize, Serialize};

use crate::environment::ApiEnvironment;
use crate::{CommandId, Mpesa, MpesaError, MpesaResult};

const B2C_URL: &str = "mpesa/b2c/v1/paymentrequest";

#[derive(Debug, Serialize)]
/// Payload to allow for b2c transactions:
struct B2cPayload<'mpesa> {
    #[serde(rename(serialize = "InitiatorName"))]
    initiator_name: &'mpesa str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credential: &'mpesa str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "Amount"))]
    amount: f64,
    #[serde(rename(serialize = "PartyA"))]
    party_a: &'mpesa str,
    #[serde(rename(serialize = "PartyB"))]
    party_b: &'mpesa str,
    #[serde(rename(serialize = "Remarks"))]
    remarks: &'mpesa str,
    #[serde(rename(serialize = "QueueTimeOutURL"))]
    queue_time_out_url: &'mpesa str,
    #[serde(rename(serialize = "ResultURL"))]
    result_url: &'mpesa str,
    #[serde(rename(serialize = "Occasion"))]
    occasion: &'mpesa str,
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
pub struct B2cBuilder<'mpesa, Env: ApiEnvironment> {
    initiator_name: &'mpesa str,
    client: &'mpesa Mpesa<Env>,
    command_id: Option<CommandId>,
    amount: Option<f64>,
    party_a: Option<&'mpesa str>,
    party_b: Option<&'mpesa str>,
    remarks: Option<&'mpesa str>,
    queue_timeout_url: Option<&'mpesa str>,
    result_url: Option<&'mpesa str>,
    occasion: Option<&'mpesa str>,
}

impl<'mpesa, Env: ApiEnvironment> B2cBuilder<'mpesa, Env> {
    /// Create a new B2C builder.
    /// Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
    pub fn new(client: &'mpesa Mpesa<Env>, initiator_name: &'mpesa str) -> B2cBuilder<'mpesa, Env> {
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
    pub fn command_id(mut self, command_id: CommandId) -> B2cBuilder<'mpesa, Env> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds `Party A` which is a required field
    /// `Party A` should be a paybill number.
    ///
    /// # Errors
    /// If `Party A` is invalid or not provided
    pub fn party_a(mut self, party_a: &'mpesa str) -> B2cBuilder<'mpesa, Env> {
        self.party_a = Some(party_a);
        self
    }

    /// Adds `Party B` which is a required field
    /// `Party B` should be a mobile number.
    ///
    /// # Errors
    /// If `Party B` is invalid or not provided
    pub fn party_b(mut self, party_b: &'mpesa str) -> B2cBuilder<'mpesa, Env> {
        self.party_b = Some(party_b);
        self
    }

    /// Adds `Party A` and `Party B`. Both are required fields
    /// `Party A` should be a paybill number while `Party B` should be a mobile number.
    ///
    /// # Errors
    /// If either `Party A` or `Party B` is invalid or not provided
    #[deprecated]
    pub fn parties(
        mut self,
        party_a: &'mpesa str,
        party_b: &'mpesa str,
    ) -> B2cBuilder<'mpesa, Env> {
        // TODO: add validation
        self.party_a = Some(party_a);
        self.party_b = Some(party_b);
        self
    }

    /// Adds `Remarks`. This is an optional field, will default to "None" if not explicitly provided
    pub fn remarks(mut self, remarks: &'mpesa str) -> B2cBuilder<'mpesa, Env> {
        self.remarks = Some(remarks);
        self
    }

    /// Adds `Occasion`. This is an optional field, will default to an empty string
    pub fn occasion(mut self, occasion: &'mpesa str) -> B2cBuilder<'mpesa, Env> {
        self.occasion = Some(occasion);
        self
    }

    /// Adds an `amount` to the request
    /// This is a required field
    pub fn amount<Number: Into<f64>>(mut self, amount: Number) -> B2cBuilder<'mpesa, Env> {
        self.amount = Some(amount.into());
        self
    }

    // Adds `QueueTimeoutUrl` This is a required field
    ///
    /// # Error
    /// If `QueueTimeoutUrl` is invalid or not provided
    pub fn timeout_url(mut self, timeout_url: &'mpesa str) -> B2cBuilder<'mpesa, Env> {
        self.queue_timeout_url = Some(timeout_url);
        self
    }

    // Adds `ResultUrl` This is a required field
    ///
    /// # Error
    /// If `ResultUrl` is invalid or not provided
    pub fn result_url(mut self, result_url: &'mpesa str) -> B2cBuilder<'mpesa, Env> {
        self.result_url = Some(result_url);
        self
    }

    /// Adds `QueueTimeoutUrl` and `ResultUrl`. This is a required field
    ///
    /// # Error
    /// If either `QueueTimeoutUrl` and `ResultUrl` is invalid or not provided
    #[deprecated]
    pub fn urls(
        mut self,
        timeout_url: &'mpesa str,
        result_url: &'mpesa str,
    ) -> B2cBuilder<'mpesa, Env> {
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
    pub async fn send(self) -> MpesaResult<B2cResponse> {
        let credentials = self.client.gen_security_credentials()?;

        let payload = B2cPayload {
            initiator_name: self.initiator_name,
            security_credential: &credentials,
            command_id: self.command_id.unwrap_or(CommandId::BusinessPayment),
            amount: self
                .amount
                .ok_or(MpesaError::Message("amount is required"))?,
            party_a: self
                .party_a
                .ok_or(MpesaError::Message("party_a is required"))?,
            party_b: self
                .party_b
                .ok_or(MpesaError::Message("party_b is required"))?,
            remarks: self.remarks.unwrap_or_else(|| stringify!(None)),
            queue_time_out_url: self
                .queue_timeout_url
                .ok_or(MpesaError::Message("queue_timeout_url is required"))?,
            result_url: self
                .result_url
                .ok_or(MpesaError::Message("result_url is required"))?,
            occasion: self.occasion.unwrap_or_else(|| stringify!(None)),
        };

        self.client
            .send(crate::client::Request {
                method: reqwest::Method::POST,
                path: B2C_URL,
                body: payload,
            })
            .await
    }
}

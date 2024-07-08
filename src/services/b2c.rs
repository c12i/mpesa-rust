#![doc = include_str!("../../docs/client/b2c.md")]

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{CommandId, Mpesa, MpesaError, MpesaResult};

const B2C_URL: &str = "mpesa/b2c/v1/paymentrequest";

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct B2cRequest {
    pub initiator_name: String,
    pub security_credential: String,
    pub command_id: CommandId,
    pub amount: f64,
    pub party_a: String,
    pub party_b: String,
    pub remarks: Option<String>,
    #[serde(rename = "QueueTimeOutURL")]
    pub queue_time_out_url: Url,
    #[serde(rename = "ResultURL")]
    pub result_url: Url,
    pub occasion: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct B2cResponse {
    #[serde(rename(deserialize = "ConversationID"))]
    pub conversation_id: String,
    #[serde(rename(deserialize = "OriginatorConversationID"))]
    pub originator_conversation_id: String,
    pub response_code: String,
    pub response_description: String,
}

/// B2C transaction builder struct
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "MpesaError"))]
pub struct B2c<'mpesa> {
    #[builder(pattern = "immutable")]
    client: &'mpesa Mpesa,
    /// The credential/ username used to authenticate the transaction request
    #[builder(setter(into))]
    initiator_name: String,
    /// The amount being transacted
    #[builder(setter(into))]
    amount: f64,
    /// Organization's shortcode initiating the transaction
    #[builder(setter(into))]
    party_a: String,
    /// Phone number receiving the transaction
    #[builder(setter(into))]
    party_b: String,
    /// The path that stores information of time out transaction
    #[builder(try_setter, setter(into))]
    queue_timeout_url: Url,
    /// The path that stores information of transaction
    #[builder(try_setter, setter(into))]
    result_url: Url,
    /// Comments that are sent along with the transaction
    #[builder(setter(into), default = "None")]
    remarks: Option<String>,
    /// Optional parameter
    #[builder(setter(into), default = "None")]
    occasion: Option<String>,
    /// The type of operation
    #[builder(default = "CommandId::BusinessPayment")]
    command_id: CommandId,
}

impl<'mpesa> B2c<'mpesa> {
    /// Creates a new B2C builder
    pub fn builder(client: &'mpesa Mpesa) -> B2cBuilder<'mpesa> {
        B2cBuilder::default().client(client)
    }

    /// # B2C API
    ///
    /// Sends b2c payment request.
    ///
    /// This API enables Business to Customer (B2C) transactions between a company and
    /// customers who are the end-users of its products or services. Use of this API requires a
    /// valid and verified B2C M-Pesa Short code.
    ///
    /// A successful request returns a `B2cResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure.
    pub async fn send(self) -> MpesaResult<B2cResponse> {
        let credentials = self.client.gen_security_credentials()?;

        let payload = B2cRequest {
            initiator_name: self.initiator_name,
            security_credential: credentials,
            command_id: self.command_id,
            amount: self.amount,
            party_a: self.party_a,
            party_b: self.party_b,
            remarks: self.remarks,
            queue_time_out_url: self.queue_timeout_url,
            result_url: self.result_url,
            occasion: self.occasion,
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

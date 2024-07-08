use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::client::Mpesa;
use crate::constants::{CommandId, IdentifierTypes};
use crate::errors::{MpesaError, MpesaResult};

const B2B_URL: &str = "mpesa/b2b/v1/paymentrequest";

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct B2bRequest {
    /// The credential/ username used to authenticate the transaction request
    pub initiator: String,
    pub security_credential: String,
    pub command_id: CommandId,
    pub amount: f64,
    pub party_a: String,
    pub sender_identifier_type: IdentifierTypes,
    pub party_b: String,
    #[serde(rename(serialize = "RecieverIdentifierType"))]
    pub reciever_identifier_type: IdentifierTypes,
    pub remarks: Option<String>,
    #[serde(rename = "QueueTimeOutURL")]
    pub queue_time_out_url: Url,
    #[serde(rename = "ResultURL")]
    pub result_url: Url,
    #[serde(rename = "AccountReference")]
    pub account_reference: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct B2bResponse {
    #[serde(rename(deserialize = "ConversationID"))]
    pub conversation_id: String,
    #[serde(rename(deserialize = "OriginatorConversationID"))]
    pub originator_conversation_id: String,
    pub response_code: String,
    pub response_description: String,
}

/// B2B transaction builder struct
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "MpesaError"))]
pub struct B2b<'mpesa> {
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
    /// Organization's shortcode receiving the funds
    #[builder(setter(into))]
    party_b: String,
    /// The path that stores information of time out transaction
    #[builder(try_setter, setter(into))]
    queue_timeout_url: Url,
    /// The path that stores information of transaction
    #[builder(try_setter, setter(into))]
    result_url: Url,
    /// Unique identifier for the transaction
    #[builder(setter(into))]
    account_ref: String,
    /// Type of organization sending the transaction
    #[builder(default = "IdentifierTypes::ShortCode")]
    sender_id: IdentifierTypes,
    /// Type of organization receiving the funds
    #[builder(default = "IdentifierTypes::ShortCode")]
    receiver_id: IdentifierTypes,
    /// Comments that are sent along with the transaction
    #[builder(setter(into), default = "None")]
    remarks: Option<String>,
    /// The type of operation
    #[builder(default = "CommandId::BusinessToBusinessTransfer")]
    command_id: CommandId,
}

impl<'mpesa> B2b<'mpesa> {
    /// Creates a new B2B builder
    pub(crate) fn builder(client: &'mpesa Mpesa) -> B2bBuilder<'mpesa> {
        B2bBuilder::default().client(client)
    }

    /// # B2B API
    ///
    /// Sends b2b payment request.
    ///
    /// This API enables Business to Business (B2B) transactions between a business and another
    /// business. Use of this API requires a valid and verified B2B M-Pesa short code for the
    /// business initiating the transaction and the both businesses involved in the transaction.
    ///
    /// A successful request returns a `B2bResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<B2bResponse> {
        let credentials = self.client.gen_security_credentials()?;

        let payload = B2bRequest {
            initiator: self.initiator_name,
            security_credential: credentials,
            command_id: self.command_id,
            amount: self.amount,
            party_a: self.party_a,
            sender_identifier_type: self.sender_id,
            party_b: self.party_b,
            reciever_identifier_type: self.receiver_id,
            remarks: self.remarks,
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

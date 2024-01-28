#![doc = include_str!("../../docs/client/account_balance.md")]

use derive_builder::Builder;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::constants::{CommandId, IdentifierTypes};
use crate::{Mpesa, MpesaError, MpesaResult};

const ACCOUNT_BALANCE_URL: &str = "mpesa/accountbalance/v1/query";

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountBalanceRequest<'mpesa> {
    pub initiator: &'mpesa str,
    pub security_credential: String,
    #[serde(rename(serialize = "CommandID"))]
    pub command_id: CommandId,
    pub party_a: &'mpesa str,
    pub identifier_type: IdentifierTypes,
    pub remarks: &'mpesa str,
    #[serde(rename(serialize = "QueueTimeOutURL"))]
    pub queue_time_out_url: Url,
    #[serde(rename(serialize = "ResultURL"))]
    pub result_url: Url,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AccountBalanceResponse {
    #[serde(rename(deserialize = "ConversationID"))]
    pub conversation_id: String,
    #[serde(rename(deserialize = "OriginatorConversationID"))]
    pub originator_conversation_id: String,
    #[serde(rename(deserialize = "ResponseCode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    pub response_description: String,
}

#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "MpesaError"))]
pub struct AccountBalance<'mpesa> {
    #[builder(pattern = "immutable", private)]
    client: &'mpesa Mpesa,
    #[builder(setter(into))]
    /// The credential/ username used to authenticate the transaction request
    initiator_name: &'mpesa str,
    /// Adds a `CommandId`, the unique command passed to the MPESA system.
    /// Defaults to `CommandId::AccountBalance` if not passed explicitly.
    ///
    /// # Errors
    /// If `CommandId` is invalid
    #[builder(default = "crate::CommandId::AccountBalance")]
    command_id: CommandId,
    /// Adds `PartyA`, the shortcode of the organization receiving the transaction.
    /// This is a required field.
    ///
    /// # Errors
    /// If `Party A` is not provided or invalid
    party_a: &'mpesa str,
    // Adds the `ReceiverIdentifierType`, the type of organization receiving the transaction.
    /// Defaults to `IdentifierTypes::ShortCode` if not passed explicitly
    ///
    /// # Errors
    /// If invalid `ReceiverIdentifierType` is provided
    #[builder(default = "crate::IdentifierTypes::ShortCode")]
    identifier_type: IdentifierTypes,
    /// Adds `Remarks`, a comment sent along transaction.
    /// Optional field that defaults to `"None"` if no value is provided
    #[builder(setter(into, strip_option), default = "Some(\"None\")")]
    remarks: Option<&'mpesa str>,
    // Adds `QueueTimeoutUrl` This is a required field
    ///
    /// # Error
    /// If `QueueTimeoutUrl` is invalid or not provided
    #[builder(try_setter, setter(into))]
    queue_timeout_url: Url,
    // Adds `ResultUrl` This is a required field
    ///
    /// # Error
    /// If `ResultUrl` is invalid or not provided
    #[builder(try_setter, setter(into))]
    result_url: Url,
}

impl<'mpesa> TryFrom<AccountBalance<'mpesa>> for AccountBalanceRequest<'mpesa> {
    type Error = MpesaError;

    fn try_from(value: AccountBalance<'mpesa>) -> MpesaResult<AccountBalanceRequest> {
        Ok(AccountBalanceRequest {
            command_id: value.command_id,
            identifier_type: value.identifier_type,
            initiator: value.initiator_name,
            party_a: value.party_a,
            queue_time_out_url: value.queue_timeout_url,
            remarks: value.remarks.unwrap_or_default(),
            result_url: value.result_url,
            security_credential: value.client.gen_security_credentials()?,
        })
    }
}

impl<'mpesa> AccountBalance<'mpesa> {
    /// Creates a new `AccountBalanceBuilder`
    pub(crate) fn builder(client: &'mpesa Mpesa) -> AccountBalanceBuilder<'mpesa> {
        AccountBalanceBuilder::default().client(client)
    }

    pub fn from_request(
        client: &'mpesa Mpesa,
        request: AccountBalanceRequest<'mpesa>,
    ) -> AccountBalance<'mpesa> {
        AccountBalance {
            client,
            command_id: request.command_id,
            identifier_type: request.identifier_type,
            initiator_name: request.initiator,
            party_a: request.party_a,
            queue_timeout_url: request.queue_time_out_url,
            remarks: Some(request.remarks),
            result_url: request.result_url,
        }
    }

    pub async fn send(self) -> MpesaResult<AccountBalanceResponse> {
        self.client
            .send::<AccountBalanceRequest, _>(crate::client::Request {
                method: reqwest::Method::POST,
                path: ACCOUNT_BALANCE_URL,
                body: self.try_into()?,
            })
            .await
    }
}

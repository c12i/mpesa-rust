#![doc = include_str!("../../docs/client/account_balance.md")]

use serde::{Deserialize, Serialize};

use crate::constants::{CommandId, IdentifierTypes};
use crate::{Mpesa, MpesaError, MpesaResult};

const ACCOUNT_BALANCE_URL: &str = "mpesa/accountbalance/v1/query";

#[derive(Debug, Serialize)]
/// Account Balance payload
struct AccountBalancePayload<'mpesa> {
    #[serde(rename(serialize = "Initiator"))]
    initiator: &'mpesa str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credential: &'mpesa str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "PartyA"))]
    party_a: &'mpesa str,
    #[serde(rename(serialize = "IdentifierType"))]
    identifier_type: &'mpesa str,
    #[serde(rename(serialize = "Remarks"))]
    remarks: &'mpesa str,
    #[serde(rename(serialize = "QueueTimeOutURL"))]
    queue_time_out_url: &'mpesa str,
    #[serde(rename(serialize = "ResultURL"))]
    result_url: &'mpesa str,
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
#[derive(Debug)]
pub struct AccountBalanceBuilder<'mpesa> {
    initiator_name: &'mpesa str,
    client: &'mpesa Mpesa,
    command_id: Option<CommandId>,
    party_a: Option<&'mpesa str>,
    identifier_type: Option<IdentifierTypes>,
    remarks: Option<&'mpesa str>,
    queue_timeout_url: Option<&'mpesa str>,
    result_url: Option<&'mpesa str>,
}

impl<'mpesa> AccountBalanceBuilder<'mpesa> {
    /// Creates a new `AccountBalanceBuilder`.
    /// Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
    pub fn new(
        client: &'mpesa Mpesa,
        initiator_name: &'mpesa str,
    ) -> AccountBalanceBuilder<'mpesa> {
        AccountBalanceBuilder {
            initiator_name,
            client,
            command_id: None,
            party_a: None,
            identifier_type: None,
            remarks: None,
            queue_timeout_url: None,
            result_url: None,
        }
    }

    /// Adds a `CommandId`, the unique command passed to the MPESA system.
    /// Defaults to `CommandId::AccountBalance` if not passed explicitly.
    ///
    /// # Errors
    /// If `CommandId` is invalid
    pub fn command_id(mut self, command_id: CommandId) -> AccountBalanceBuilder<'mpesa> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds `PartyA`, the shortcode of the organization receiving the transaction.
    /// This is a required field.
    ///
    /// # Errors
    /// If `Party A` is not provided or invalid
    pub fn party_a(mut self, party_a: &'mpesa str) -> AccountBalanceBuilder<'mpesa> {
        self.party_a = Some(party_a);
        self
    }

    /// Adds the `ReceiverIdentifierType`, the type of organization receiving the transaction.
    /// Defaults to `IdentifierTypes::ShortCode` if not passed explicitly
    ///
    /// # Errors
    /// If invalid `ReceiverIdentifierType` is provided
    pub fn identifier_type(
        mut self,
        identifier_type: IdentifierTypes,
    ) -> AccountBalanceBuilder<'mpesa> {
        self.identifier_type = Some(identifier_type);
        self
    }

    /// Adds `Remarks`, a comment sent along transaction.
    /// Optional field that defaults to `"None"` if no value is provided
    pub fn remarks(mut self, remarks: &'mpesa str) -> AccountBalanceBuilder<'mpesa> {
        self.remarks = Some(remarks);
        self
    }

    // Adds `QueueTimeoutUrl` This is a required field
    ///
    /// # Error
    /// If `QueueTimeoutUrl` is invalid or not provided
    pub fn timeout_url(mut self, timeout_url: &'mpesa str) -> AccountBalanceBuilder<'mpesa> {
        self.queue_timeout_url = Some(timeout_url);
        self
    }

    // Adds `ResultUrl` This is a required field
    ///
    /// # Error
    /// If `ResultUrl` is invalid or not provided
    pub fn result_url(mut self, result_url: &'mpesa str) -> AccountBalanceBuilder<'mpesa> {
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
    ) -> AccountBalanceBuilder<'mpesa> {
        self.queue_timeout_url = Some(timeout_url);
        self.result_url = Some(result_url);
        self
    }

    /// # AccountBalance API
    ///
    /// Enquire the balance on an M-Pesa BuyGoods (Till Number).
    /// A successful request returns a `C2bRegisterResponse` type.
    /// See more [here](https://developer.safaricom.co.ke/docs#account-balance-api)
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<AccountBalanceResponse> {
        let credentials = self.client.gen_security_credentials()?;

        let payload = AccountBalancePayload {
            command_id: self.command_id.unwrap_or(CommandId::AccountBalance),
            party_a: self
                .party_a
                .ok_or(MpesaError::Message("party_a is required"))?,
            identifier_type: &self
                .identifier_type
                .unwrap_or(IdentifierTypes::ShortCode)
                .to_string(),
            remarks: self.remarks.unwrap_or_else(|| stringify!(None)),
            initiator: self.initiator_name,
            queue_time_out_url: self
                .queue_timeout_url
                .ok_or(MpesaError::Message("queue_timeout_url is required"))?,
            result_url: self
                .result_url
                .ok_or(MpesaError::Message("result_url is required"))?,
            security_credential: &credentials,
        };

        self.client
            .send(crate::client::Request {
                method: reqwest::Method::POST,
                path: ACCOUNT_BALANCE_URL,
                body: payload,
            })
            .await
    }
}

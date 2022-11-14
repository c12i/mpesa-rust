use crate::client::MpesaResult;
use crate::constants::{CommandId, IdentifierTypes};
use crate::{Mpesa, MpesaError};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
/// Account Balance payload
struct AccountBalancePayload<'a> {
    #[serde(rename(serialize = "Initiator"))]
    initiator: &'a str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credential: &'a str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "PartyA"), skip_serializing_if = "Option::is_none")]
    party_a: Option<&'a str>,
    #[serde(rename(serialize = "IdentifierType"))]
    identifier_type: &'a str,
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
pub struct AccountBalanceBuilder<'a> {
    initiator_name: &'a str,
    client: &'a Mpesa,
    command_id: Option<CommandId>,
    party_a: Option<&'a str>,
    identifier_type: Option<IdentifierTypes>,
    remarks: Option<&'a str>,
    queue_timeout_url: Option<&'a str>,
    result_url: Option<&'a str>,
}

impl<'a> AccountBalanceBuilder<'a> {
    /// Creates a new `AccountBalanceBuilder`.
    /// Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
    pub fn new(client: &'a Mpesa, initiator_name: &'a str) -> AccountBalanceBuilder<'a> {
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
    pub fn command_id(mut self, command_id: CommandId) -> AccountBalanceBuilder<'a> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds `PartyA`, the shortcode of the organization receiving the transaction.
    /// This is a required field.
    ///
    /// # Errors
    /// If `Party A` is not provided or invalid
    pub fn party_a(mut self, party_a: &'a str) -> AccountBalanceBuilder<'a> {
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
    ) -> AccountBalanceBuilder<'a> {
        self.identifier_type = Some(identifier_type);
        self
    }

    /// Adds `Remarks`, a comment sent along transaction.
    /// Optional field that defaults to `"None"` if no value is provided
    pub fn remarks(mut self, remarks: &'a str) -> AccountBalanceBuilder<'a> {
        self.remarks = Some(remarks);
        self
    }

    // Adds `QueueTimeoutUrl` This is a required field
    ///
    /// # Error
    /// If `QueueTimeoutUrl` is invalid or not provided
    pub fn timeout_url(mut self, timeout_url: &'a str) -> AccountBalanceBuilder<'a> {
        self.queue_timeout_url = Some(timeout_url);
        self
    }

    // Adds `ResultUrl` This is a required field
    ///
    /// # Error
    /// If `ResultUrl` is invalid or not provided
    pub fn result_url(mut self, result_url: &'a str) -> AccountBalanceBuilder<'a> {
        self.result_url = Some(result_url);
        self
    }

    /// Adds `QueueTimeoutUrl` and `ResultUrl`. This is a required field
    ///
    /// # Error
    /// If either `QueueTimeoutUrl` and `ResultUrl` is invalid or not provided
    #[deprecated]
    pub fn urls(mut self, timeout_url: &'a str, result_url: &'a str) -> AccountBalanceBuilder<'a> {
        self.queue_timeout_url = Some(timeout_url);
        self.result_url = Some(result_url);
        self
    }

    /// **AccountBalance API**
    ///
    /// Enquire the balance on an M-Pesa BuyGoods (Till Number).
    /// A successful request returns a `serde_json::Value` type.
    /// See more [here](https://developer.safaricom.co.ke/docs#account-balance-api)
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub fn send(self) -> MpesaResult<AccountBalanceResponse> {
        let url = format!(
            "{}/mpesa/accountbalance/v1/query",
            self.client.environment().base_url()
        );

        let credentials = self.client.gen_security_credentials()?;

        let payload = AccountBalancePayload {
            command_id: self.command_id.unwrap_or_else(|| CommandId::AccountBalance),
            party_a: self.party_a,
            identifier_type: &self
                .identifier_type
                .unwrap_or_else(|| IdentifierTypes::ShortCode)
                .to_string(),
            remarks: self.remarks.unwrap_or_else(|| "None"),
            initiator: self.initiator_name,
            queue_time_out_url: self.queue_timeout_url,
            result_url: self.result_url,
            security_credential: &credentials,
        };

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth()?)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            let value: AccountBalanceResponse = response.json()?;
            return Ok(value);
        }

        let value: Value = response.json()?;
        Err(MpesaError::AccountBalanceError(value))
    }
}

use crate::client::MpesaResult;
use crate::constants::{CommandId, IdentifierTypes};
use crate::{Mpesa, MpesaError, MpesaSecurity};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
/// Account Balance payload
struct AccountBalancePayload<'a> {
    Initiator: &'a str,
    SecurityCredential: &'a str,
    CommandID: CommandId,
    PartyA: &'a str,
    IdentifierType: &'a str,
    Remarks: &'a str,
    QueueTimeOutURL: &'a str,
    ResultURL: &'a str,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AccountBalanceResponse {
    ConversationID: String,
    OriginatorConversationID: String,
    ResponseCode: String,
    ResponseDescription: String,
}

#[allow(dead_code)]
impl<'a> AccountBalanceResponse {
    pub fn conversation_id(&'a self) -> &'a String {
        &self.ConversationID
    }

    pub fn originator_conversation_id(&'a self) -> &'a String {
        &self.OriginatorConversationID
    }

    pub fn response_code(&'a self) -> &'a String {
        &self.ResponseCode
    }

    pub fn response_description(&'a self) -> &'a String {
        &self.ResponseDescription
    }
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

    /// Adds `QueueTimeoutUrl` and `ResultUrl`. This is a required field
    ///
    /// # Error
    /// If either `QueueTimeoutUrl` and `ResultUrl` is invalid or not provided
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
    pub fn send(self) -> MpesaResult<AccountBalanceResponse> {
        let url = format!(
            "{}/mpesa/accountbalance/v1/query",
            self.client.environment().base_url()
        );

        let credentials = self.client.gen_security_credentials()?;

        let payload = AccountBalancePayload {
            CommandID: self.command_id.unwrap_or(CommandId::AccountBalance),
            PartyA: self.party_a.unwrap_or("None"),
            IdentifierType: &self
                .identifier_type
                .unwrap_or(IdentifierTypes::ShortCode)
                .to_string(),
            Remarks: self.remarks.unwrap_or("None"),
            Initiator: self.initiator_name,
            QueueTimeOutURL: self.queue_timeout_url.unwrap_or("None"),
            ResultURL: self.result_url.unwrap_or("None"),
            SecurityCredential: &credentials,
        };

        let response = Client::new()
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

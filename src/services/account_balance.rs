use crate::client::MpesaResult;
use crate::constants::{CommandId, IdentifierTypes};
use crate::{Mpesa, MpesaError, MpesaSecurity};
use reqwest::blocking::Client;
use serde_json::{json, Value};

#[derive(Debug)]
/// Account Balance payload
struct AccountBalancePayload<'a> {
    initiator_name: &'a str,
    security_credentials: &'a str,
    command_id: CommandId,
    party_a: &'a str,
    identifier_type: IdentifierTypes,
    remarks: &'a str,
    queue_timeout_url: &'a str,
    result_url: &'a str,
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

    /// # AccountBalance API
    /// Enquire the balance on an M-Pesa BuyGoods (Till Number).
    /// A successful request returns a `serde_json::Value` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub fn send(self) -> MpesaResult<Value> {
        let url = format!(
            "{}/mpesa/accountbalance/v1/query",
            self.client.environment().base_url()
        );

        let credentials = self.client.gen_security_credentials()?;

        let payload = AccountBalancePayload {
            command_id: self.command_id.unwrap_or(CommandId::AccountBalance),
            party_a: self.party_a.unwrap_or("None"),
            identifier_type: self.identifier_type.unwrap_or(IdentifierTypes::Shortcode),
            remarks: self.remarks.unwrap_or("None"),
            initiator_name: self.initiator_name,
            queue_timeout_url: self.queue_timeout_url.unwrap_or("None"),
            result_url: self.result_url.unwrap_or("None"),
            security_credentials: &credentials,
        };

        let data = json!({
            "CommandID": payload.command_id.to_string(),
            "PartyA": payload.party_a,
            "IdentifierType": payload.identifier_type.get_code(),
            "Remarks": payload.remarks,
            "Initiator": payload.initiator_name,
            "SecurityCredential": payload.security_credentials,
            "QueueTimeOutURL": payload.queue_timeout_url,
            "ResultURL": payload.result_url,
        });

        let response = Client::new()
            .post(&url)
            .bearer_auth(self.client.auth()?)
            .json(&data)
            .send()?;

        if response.status().is_success() {
            let value: Value = response.json()?;
            return Ok(value);
        }

        let value: Value = response.json()?;
        Err(MpesaError::ErrorResponse(value))
    }
}

use crate::client::MpesaResult;
use crate::constants::{CommandId, IdentifierTypes};
use crate::environment::ApiEnvironment;
use crate::{Mpesa, MpesaError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
/// Account Balance payload
struct AccountBalancePayload<'mpesa> {
    #[serde(rename(serialize = "Initiator"))]
    initiator: &'mpesa str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credential: &'mpesa str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "PartyA"), skip_serializing_if = "Option::is_none")]
    party_a: Option<&'mpesa str>,
    #[serde(rename(serialize = "IdentifierType"))]
    identifier_type: &'mpesa str,
    #[serde(rename(serialize = "Remarks"))]
    remarks: &'mpesa str,
    #[serde(
        rename(serialize = "QueueTimeOutURL"),
        skip_serializing_if = "Option::is_none"
    )]
    queue_time_out_url: Option<&'mpesa str>,
    #[serde(
        rename(serialize = "ResultURL"),
        skip_serializing_if = "Option::is_none"
    )]
    result_url: Option<&'mpesa str>,
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
pub struct AccountBalanceBuilder<'mpesa, Env: ApiEnvironment> {
    initiator_name: &'mpesa str,
    client: &'mpesa Mpesa<Env>,
    command_id: Option<CommandId>,
    party_a: Option<&'mpesa str>,
    identifier_type: Option<IdentifierTypes>,
    remarks: Option<&'mpesa str>,
    queue_timeout_url: Option<&'mpesa str>,
    result_url: Option<&'mpesa str>,
}

impl<'mpesa, Env: ApiEnvironment> AccountBalanceBuilder<'mpesa, Env> {
    /// Creates a new `AccountBalanceBuilder`.
    /// Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
    pub fn new(
        client: &'mpesa Mpesa<Env>,
        initiator_name: &'mpesa str,
    ) -> AccountBalanceBuilder<'mpesa, Env> {
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
    pub fn command_id(mut self, command_id: CommandId) -> AccountBalanceBuilder<'mpesa, Env> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds `PartyA`, the shortcode of the organization receiving the transaction.
    /// This is a required field.
    ///
    /// # Errors
    /// If `Party A` is not provided or invalid
    pub fn party_a(mut self, party_a: &'mpesa str) -> AccountBalanceBuilder<'mpesa, Env> {
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
    ) -> AccountBalanceBuilder<'mpesa, Env> {
        self.identifier_type = Some(identifier_type);
        self
    }

    /// Adds `Remarks`, a comment sent along transaction.
    /// Optional field that defaults to `"None"` if no value is provided
    pub fn remarks(mut self, remarks: &'mpesa str) -> AccountBalanceBuilder<'mpesa, Env> {
        self.remarks = Some(remarks);
        self
    }

    // Adds `QueueTimeoutUrl` This is a required field
    ///
    /// # Error
    /// If `QueueTimeoutUrl` is invalid or not provided
    pub fn timeout_url(mut self, timeout_url: &'mpesa str) -> AccountBalanceBuilder<'mpesa, Env> {
        self.queue_timeout_url = Some(timeout_url);
        self
    }

    // Adds `ResultUrl` This is a required field
    ///
    /// # Error
    /// If `ResultUrl` is invalid or not provided
    pub fn result_url(mut self, result_url: &'mpesa str) -> AccountBalanceBuilder<'mpesa, Env> {
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
    ) -> AccountBalanceBuilder<'mpesa, Env> {
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
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<AccountBalanceResponse> {
        let url = format!(
            "{}/mpesa/accountbalance/v1/query",
            self.client.environment.base_url()
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
            .bearer_auth(self.client.auth().await?)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json::<_>().await?;
            return Ok(value);
        }

        let value = response.json().await?;
        Err(MpesaError::AccountBalanceError(value))
    }
}

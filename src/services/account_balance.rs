use crate::client::MpesaResult;
use crate::{Mpesa, MpesaError, MpesaSecurity};
use crate::constants::{CommandId, IdentifierTypes};
use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug)]
/// Account Balance payload
pub struct AccountBalancePayload<'a> {
    initiator_name: &'a str,
    security_credentials: &'a str,
    command_id: CommandId,
    party_a: &'a str,
    identifier_type: IdentifierTypes,
    remarks: &'a str,
    queue_timeout_url: &'a str,
    result_url: &'a str,
}

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
            result_url: None
        }
    }
}

/// B2C response
/// Field names deliberately in Pascal case to correctly deserialize the
/// response data
#[derive(Debug, Deserialize)]
pub struct AccountBalanceResponse {
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub ResponseCode: String,
    pub ResponseDescription: String,
}

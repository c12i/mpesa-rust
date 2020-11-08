use crate::{CommandId, IdentifierTypes};
use serde::Deserialize;

#[derive(Debug)]
pub struct AccountBalancePayload<'a> {
    pub initiator_name: &'a str,
    pub security_credentials: &'a str,
    pub command_id: CommandId,
    pub party_a: &'a str,
    pub identifier_type: IdentifierTypes,
    pub remarks: &'a str,
    pub queue_timeout_url: &'a str,
    pub result_url: &'a str,
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

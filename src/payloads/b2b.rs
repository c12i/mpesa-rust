use serde::Deserialize;
use crate::CommandId;

#[derive(Debug)]
pub struct B2bPayload<'a> {
    pub initiator_name: &'a str,
    pub security_credentials: &'a str,
    pub command_id: CommandId,
    pub amount: u32,
    pub party_a: &'a str,
    pub sender_id: u32,
    pub party_b: &'a str,
    pub receiver_id: u32,
    pub remarks: &'a str,
    pub queue_timeout_url: &'a str,
    pub result_url: &'a str,
    pub account_ref: &'a str,
}

#[derive(Debug,Deserialize)]
/// B2C response
/// Field names deliberately in Pascal case to correctly deserialize the
/// response data
pub struct B2bResponse {
    pub ConversationID: String,
    pub OriginatorConversationID: String,
    pub ResponseCode: String,
    pub ResponseDescription: String,
}


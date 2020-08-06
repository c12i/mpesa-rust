use serde::Deserialize;
use crate::CommandId;

#[derive(Debug)]
/// Payload to register the 3rd party’s confirmation and validation URLs to M-Pesa
/// See more here: https://developer.safaricom.co.ke/docs?shell#c2b-api
struct C2bRegisterPayload<'a> {
    pub validation_url: &'a str,
    pub confirmation_url: &'a str,
    pub response_type: &'a str,
    pub short_code: &'a str,
}

#[derive(Debug,Deserialize)]
/// C2B register response
/// Field names deliberately in Pascal case to correctly deserialize the
/// response data
struct C2bRegisterResponse {
    pub ConversationID: String,
    pub OriginatorConversationID: String,
    pub ResponseDescription: String,
}
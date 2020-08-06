use serde::Deserialize;
use std::fmt::{Display,Formatter,Result as FmtResult};
use crate::CommandId;

#[derive(Debug)]
/// Payload to register the 3rd party’s confirmation and validation URLs to M-Pesa
/// See more here: https://developer.safaricom.co.ke/docs?shell#c2b-api
pub struct C2bRegisterPayload<'a> {
    pub validation_url: &'a str,
    pub confirmation_url: &'a str,
    pub response_type: ResponseType,
    pub short_code: &'a str,
}

#[derive(Debug,Deserialize)]
/// C2B register response
/// Field names deliberately in Pascal case to correctly deserialize the
/// response data
pub struct C2bRegisterResponse {
    pub ConversationID: String,
    pub OriginatorConversationID: String,
    pub ResponseDescription: String,
}

#[derive(Debug)]
/// C2B Register Response types
pub enum ResponseType {
    Complete,
    Cancelled,
}

impl ResponseType {
    /// Stringify response type
    fn response_type_string(&self) -> &'static str {
        match self {
            ResponseType::Cancelled => "Cancelled",
            ResponseType::Complete => "Complete",
        }
    }
}

impl Display for ResponseType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self.response_type_string())
    }
}

#[derive(Debug)]
/// Payload to make payment requests from C2B.
/// See more: https://developer.safaricom.co.ke/docs#c2b-api
pub struct C2bSimulatePayload<'a> {
    pub command_id: CommandId,
    pub amount: u32,
    pub msisdn: &'a str,
    pub bill_ref_number: &'a str,
    pub short_code: &'a str,
}

#[derive(Debug, Deserialize)]
/// C2B payment response
pub struct C2bSimulateResponse {
    pub ConversationID: String,
    pub OriginatorCoversationID: String,
    pub ResponseDescription: String,
}
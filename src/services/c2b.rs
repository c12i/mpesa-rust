use crate::CommandId;
use serde::Deserialize;

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

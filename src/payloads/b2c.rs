use crate::CommandId;

#[derive(Debug)]
/// Payload to allow for b2c transactions:
/// See https://developer.safaricom.co.ke/docs#b2c-api for a
/// detailed description of each field.
pub struct B2cPayload<'a> {
    pub initiator_name: &'a str,
    pub security_credentials: &'a str,
    pub command_id: CommandId,
    pub amount: u32,
    pub party_a: &'a str,
    pub party_b: &'a str,
    pub remarks: &'a str,
    pub queue_timeout_url: &'a str,
    pub result_url: &'a str,
    pub occasion: &'a str,
}

#[derive(Debug)]
/// B2C response
pub struct B2cResponse<'a> {
    conversation_id: &'a str,
    original_conversation_id: &'a str,
    response_code: &'a str,
    response_description: &'a str,
}
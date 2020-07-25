use crate::CommandId;

#[derive(Debug)]
pub struct B2bPayload<'a> {
    pub initiator_name: &'a str,
    pub security_credentials: &'a str,
    pub command_id: CommandId,
    pub amount: u32,
    pub party_a: &'a str,
    pub sender_id: &'a str,
    pub party_b: &'a str,
    pub receiver_id: &'a str,
    pub remarks: &'a str,
    pub queue_timeout_url: &'a str,
    pub result_url: &'a str,
    pub occasion: &'a str,
}

#[derive(Debug)]
pub struct B2bResponse<'a> {
    conversation_id: &'a str,
    originator_conversation_id: &'a str,
    response_description: &'a str,
}


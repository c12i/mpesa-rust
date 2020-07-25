use crate::CommandId;

#[derive(Debug)]
pub struct B2bPayload<'a> {
    initiator_name: &'a str,
    security_credentials: &'a str,
    command_id: CommandId,
}
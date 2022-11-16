use serde::Serialize;

use crate::CommandId;

#[derive(Debug, Serialize)]
pub struct TransactionReversalPayload<'a> {
    #[serde(rename(serialize = "Initiator"))]
    initiator: &'a str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credentials: &'a str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "TransactionID"))]
    transaction_id: &'a str,
    #[serde(rename(serialize = "ReceiverParty"))]
    receiver_party: &'a str,
    #[serde(rename(serialize = "ReceiverIdentifierType"))]
    receiver_identifier_type: &'a str,
    #[serde(rename(serialize = "ResultURL"))]
    result_url: &'a str,
    #[serde(rename(serialize = "QueueTimeOutURL"))]
    queue_timeout_url: &'a str,
    #[serde(rename(serialize = "Remarks"))]
    remarks: &'a str,
    #[serde(rename(serialize = "Occasion"))]
    ocassion: &'a str,
}

pub struct TransactionReversalBuilder;

pub struct TransactionReversalResponse;

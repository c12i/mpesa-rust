use serde::Serialize;

use crate::CommandId;

#[derive(Debug, Serialize)]
pub struct TransactionReversalPayload<'mpesa> {
    #[serde(rename(serialize = "Initiator"))]
    initiator: &'mpesa str,
    #[serde(rename(serialize = "SecurityCredential"))]
    security_credentials: &'mpesa str,
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "TransactionID"))]
    transaction_id: &'mpesa str,
    #[serde(rename(serialize = "ReceiverParty"))]
    receiver_party: &'mpesa str,
    #[serde(rename(serialize = "ReceiverIdentifierType"))]
    receiver_identifier_type: &'mpesa str,
    #[serde(rename(serialize = "ResultURL"))]
    result_url: &'mpesa str,
    #[serde(rename(serialize = "QueueTimeOutURL"))]
    queue_timeout_url: &'mpesa str,
    #[serde(rename(serialize = "Remarks"))]
    remarks: &'mpesa str,
    #[serde(rename(serialize = "Occasion"))]
    ocassion: &'mpesa str,
}

pub struct TransactionReversalBuilder;

pub struct TransactionReversalResponse;

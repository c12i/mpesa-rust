use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{ApiEnvironment, CommandId, IdentifierTypes, Mpesa, MpesaError, MpesaResult};

const TRANSACTION_REVERSAL_URL: &str = "mpesa/reversal/v1/request";

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionReversalRequest<'mpesa> {
    /// The name of the initiator to initiate the request.
    pub initiator: &'mpesa str,
    /// Encrypted Credential of user getting transaction reversed.
    pub security_credential: String,
    /// Unique command for each transaction type.
    #[serde(rename = "CommandID")]
    pub command_id: CommandId,
    /// This is the Mpesa Transaction ID of the transaction which you wish to
    #[serde(rename = "TransactionID")]
    pub transaction_id: &'mpesa str,
    /// The organization that receives the transaction.
    pub receiver_party: &'mpesa str,
    /// Type of organization that receives the transaction.
    pub receiver_identifier_type: IdentifierTypes,
    /// The path that stores information about the transaction.
    #[serde(rename = "ResultURL")]
    pub result_url: Url,
    /// The path that stores information about the time-out transaction.
    #[serde(rename = "QueueTimeOutURL")]
    pub queue_timeout_url: Url,
    /// Comments that are sent along with the transaction.
    pub remarks: &'mpesa str,
    /// Comments that are sent along with the transaction.
    pub occasion: Option<&'mpesa str>,
    /// The amount transacted in the transaction is to be reversed, down to the
    /// cent.
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionReversalResponse {
    /// The unique request ID for tracking a transaction.
    #[serde(rename = "ConversationID")]
    pub conversation_id: String,
    /// The unique request ID is returned by mpesa for each request made.
    #[serde(rename = "OriginatorConversationID")]
    pub originator_conversation_id: String,
    /// Response Description message
    pub response_description: String,
    /// Response Code
    pub response_code: String,
}

#[derive(Builder, Debug)]
#[builder(build_fn(error = "MpesaError"))]
pub struct TransactionReversal<'mpesa, Env: ApiEnvironment> {
    #[builder(pattern = "immutable")]
    client: &'mpesa Mpesa<Env>,
    /// The name of the initiator to initiate the request.
    initiator: &'mpesa str,
    /// This is the Mpesa Transaction ID of the transaction which you wish to
    /// reverse.
    #[builder(setter(into))]
    transaction_id: &'mpesa str,
    /// The organization that receives the transaction.
    #[builder(setter(into))]
    receiver_party: &'mpesa str,
    /// The path that stores information about the transaction.
    #[builder(try_setter, setter(into))]
    result_url: Url,
    /// The path that stores information about the time-out transaction.
    #[builder(try_setter, setter(into))]
    timeout_url: Url,
    /// Comments that are sent along with the transaction.
    #[builder(setter(into))]
    remarks: &'mpesa str,
    /// Comments that are sent along with the transaction.
    #[builder(setter(into, strip_option), default)]
    occasion: Option<&'mpesa str>,
    /// The amount transacted in the transaction is to be reversed, down to the
    /// cent.
    amount: u32,
}

impl<'mpesa, Env: ApiEnvironment> TryFrom<TransactionReversal<'mpesa, Env>>
    for TransactionReversalRequest<'mpesa>
{
    type Error = MpesaError;

    fn try_from(
        value: TransactionReversal<'mpesa, Env>,
    ) -> Result<TransactionReversalRequest<'mpesa>, Self::Error> {
        let credentials = value.client.gen_security_credentials()?;

        Ok(TransactionReversalRequest {
            initiator: value.initiator,
            security_credential: credentials,
            command_id: CommandId::TransactionReversal,
            transaction_id: value.transaction_id,
            receiver_party: value.receiver_party,
            receiver_identifier_type: IdentifierTypes::Reversal,
            result_url: value.result_url,
            queue_timeout_url: value.timeout_url,
            remarks: value.remarks,
            occasion: value.occasion,
            amount: value.amount,
        })
    }
}

impl<'mpesa, Env: ApiEnvironment> TransactionReversal<'mpesa, Env> {
    /// Creates new `TransactionReversalBuilder`
    pub(crate) fn builder(client: &'mpesa Mpesa<Env>) -> TransactionReversalBuilder<'mpesa, Env> {
        TransactionReversalBuilder::default().client(client)
    }

    /// Creates a new `TransactionReversal` from a `TransactionReversalRequest`
    pub fn from_request(
        client: &'mpesa Mpesa<Env>,
        request: TransactionReversalRequest<'mpesa>,
    ) -> TransactionReversal<'mpesa, Env> {
        TransactionReversal {
            client,
            initiator: request.initiator,
            transaction_id: request.transaction_id,
            receiver_party: request.receiver_party,
            result_url: request.result_url,
            timeout_url: request.queue_timeout_url,
            remarks: request.remarks,
            occasion: request.occasion,
            amount: request.amount,
        }
    }

    /// # Transaction Reversal API
    ///
    /// Requests for transaction reversal
    ///
    /// This API enables reversal of a B2B, B2C or C2B M-Pesa transaction
    /// Required  parameters:
    ///
    /// `transaction_id`: This is the Mpesa Transaction ID of the transaction which you wish to reverse
    ///
    /// `amount` : The amount transacted in the transaction to be reversed , down to the cent
    ///
    /// `receiver_party`: Your organization's short code.
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/Documentation)
    ///
    /// A successful request returns a `TransactionReversalResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure.
    pub async fn send(self) -> MpesaResult<TransactionReversalResponse> {
        self.client
            .send::<TransactionReversalRequest, _>(crate::client::Request {
                method: reqwest::Method::POST,
                path: TRANSACTION_REVERSAL_URL,
                body: self.try_into()?,
            })
            .await
    }
}

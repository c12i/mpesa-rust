use serde::Deserialize;
use serde::Serialize;

use crate::ApiEnvironment;
use crate::CommandId;
use crate::IdentifierTypes;
use crate::Mpesa;
use crate::MpesaError;
use crate::MpesaResult;

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
    #[serde(rename(serialize = "RecieverIdentifierType"))]
    receiver_identifier_type: IdentifierTypes,
    #[serde(rename(serialize = "ResultURL"))]
    result_url: &'mpesa str,
    #[serde(rename(serialize = "QueueTimeOutURL"))]
    timeout_url: &'mpesa str,
    #[serde(rename(serialize = "Remarks"))]
    remarks: &'mpesa str,
    #[serde(rename(serialize = "Occasion"))]
    occasion: &'mpesa str,
    #[serde(rename(serialize = "Amount"))]
    amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReversalResponse {
    #[serde(rename(deserialize = "ConversationID"))]
    pub conversation_id: String,
    #[serde(rename(deserialize = "OriginatorConversationID"))]
    pub originator_conversation_id: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    pub response_description: String,
}

#[derive(Debug)]
pub struct TransactionReversalBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    initiator: &'mpesa str,
    command_id: Option<CommandId>,
    transaction_id: Option<&'mpesa str>,
    receiver_party: Option<&'mpesa str>,
    receiver_identifier_type: Option<IdentifierTypes>,
    result_url: Option<&'mpesa str>,
    timeout_url: Option<&'mpesa str>,
    remarks: Option<&'mpesa str>,
    occasion: Option<&'mpesa str>,
    amount: Option<f64>,
}

impl<'mpesa, Env: ApiEnvironment> TransactionReversalBuilder<'mpesa, Env> {
    /// Creates new `TransactionReversalBuilder`
    pub fn new(
        client: &'mpesa Mpesa<Env>,
        initiator: &'mpesa str,
    ) -> TransactionReversalBuilder<'mpesa, Env> {
        TransactionReversalBuilder {
            client,
            initiator,
            command_id: None,
            transaction_id: None,
            receiver_party: None,
            receiver_identifier_type: None,
            result_url: None,
            timeout_url: None,
            remarks: None,
            occasion: None,
            amount: None,
        }
    }

    /// Adds `CommandId`. Defaults to `CommandId::TransactionReversal` if no value explicitly passed
    ///
    /// # Errors
    /// If `CommandId` is not valid
    pub fn command_id(mut self, command_id: CommandId) -> Self {
        self.command_id = Some(command_id);
        self
    }

    /// Add the Mpesa Transaction ID of the transaction which you wish to reverse
    ///
    /// This is a required field.
    pub fn transaction_id(mut self, transaction_id: &'mpesa str) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    /// Organization receiving the transaction
    ///
    /// This is required field
    pub fn receiver_party(mut self, receiver_party: &'mpesa str) -> Self {
        self.receiver_party = Some(receiver_party);
        self
    }

    /// Type of organization receiving the transaction
    ///
    /// This is an optional field, will default to `IdentifierTypes::ShortCode`
    pub fn receiver_identifier_type(mut self, receiver_identifier_type: IdentifierTypes) -> Self {
        self.receiver_identifier_type = Some(receiver_identifier_type);
        self
    }

    // Adds `ResultUrl` This is a required field
    ///
    /// # Error
    /// If `ResultUrl` is invalid or not provided
    pub fn result_url(mut self, result_url: &'mpesa str) -> Self {
        self.result_url = Some(result_url);
        self
    }

    /// Adds `QueueTimeoutUrl` and `ResultUrl`. This is a required field
    ///
    /// # Error
    /// If either `QueueTimeoutUrl` and `ResultUrl` is invalid or not provided
    pub fn timeout_url(mut self, timeout_url: &'mpesa str) -> Self {
        self.timeout_url = Some(timeout_url);
        self
    }

    /// Comments that are sent along with the transaction.
    ///
    /// This is an optiona field; defaults to "None"
    pub fn remarks(mut self, remarks: &'mpesa str) -> Self {
        self.remarks = Some(remarks);
        self
    }

    /// Adds any additional information to be associated with the transaction.
    ///
    /// This is an optional Parameter, defaults to "None"
    pub fn occasion(mut self, occasion: &'mpesa str) -> Self {
        self.occasion = Some(occasion);
        self
    }

    /// Adds an `amount` to the request
    ///
    /// This is a required field
    pub fn amount<Number: Into<f64>>(mut self, amount: Number) -> Self {
        self.amount = Some(amount.into());
        self
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
        let url = format!(
            "{}/mpesa/reversal/v1/request",
            self.client.environment.base_url()
        );

        let credentials = self.client.gen_security_credentials()?;

        let payload = TransactionReversalPayload {
            initiator: self.initiator,
            security_credentials: &credentials,
            command_id: self.command_id.unwrap_or(CommandId::TransactionReversal),
            transaction_id: self
                .transaction_id
                .ok_or(MpesaError::Message("transaction_id is required"))?,
            receiver_party: self
                .receiver_party
                .ok_or(MpesaError::Message("receiver_party is required"))?,
            receiver_identifier_type: self
                .receiver_identifier_type
                .unwrap_or(IdentifierTypes::ShortCode),
            result_url: self
                .result_url
                .ok_or(MpesaError::Message("result_url is required"))?,
            timeout_url: self
                .timeout_url
                .ok_or(MpesaError::Message("timeout_url is required"))?,
            remarks: self.remarks.unwrap_or(stringify!(None)),
            occasion: self.occasion.unwrap_or(stringify!(None)),
            amount: self
                .amount
                .ok_or(MpesaError::Message("amount is required"))?,
        };

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let value = response.json().await?;
            return Err(MpesaError::MpesaTransactionReversalError(value));
        };

        let response = response.json::<_>().await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::get_mpesa_client;
    use mpesa::MpesaError;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    #[tokio::test]
    async fn transaction_reversal_success() {
        let (client, server) = get_mpesa_client!();
        let sample_response_body = json!({
            "OriginatorConversationID": "29464-48063588-1",
            "ConversationID": "AG_20230206_201056794190723278ff",
            "ResponseDescription": "Accept the service request successfully.",
        });
        Mock::given(method("POST"))
            .and(path("/mpesa/reversal/v1/request"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
            .expect(1)
            .mount(&server)
            .await;
        let response = client
            .transaction_reversal("testapi496")
            .result_url("https://testdomain.com/ok")
            .timeout_url("https://testdomain.com/err")
            .transaction_id("OEI2AK4Q16")
            .amount(1.0)
            .receiver_party("600111")
            .remarks("wrong recipient")
            .send()
            .await
            .unwrap();
        assert_eq!(response.originator_conversation_id, "29464-48063588-1");
        assert_eq!(response.conversation_id, "AG_20230206_201056794190723278ff");
        assert_eq!(
            response.response_description,
            "Accept the service request successfully."
        );
    }

    #[tokio::test]
    async fn transaction_reversal_fails_if_no_transaction_id_is_provided() {
        let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
        let sample_response_body = json!({
            "OriginatorConversationID": "29464-48063588-1",
            "ConversationID": "AG_20230206_201056794190723278ff",
            "ResponseDescription": "Accept the service request successfully.",
        });
        Mock::given(method("POST"))
            .and(path("/mpesa/reversal/v1/request"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
            .expect(0)
            .mount(&server)
            .await;
        if let Err(e) = client
            .transaction_reversal("testapi496")
            .result_url("https://testdomain.com/ok")
            .timeout_url("https://testdomain.com/err")
            .amount(1.0)
            .receiver_party("600111")
            .send()
            .await
        {
            let MpesaError::Message(msg) = e else {
                panic!("Expected MpesaError::Message, but found {}", e);
            };
            assert_eq!(msg, "transaction_id is required");
        } else {
            panic!("Expected error");
        }
    }

    #[tokio::test]
    async fn transaction_reversal_fails_if_no_amount_is_provided() {
        let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
        let sample_response_body = json!({
            "OriginatorConversationID": "29464-48063588-1",
            "ConversationID": "AG_20230206_201056794190723278ff",
            "ResponseDescription": "Accept the service request successfully.",
        });
        Mock::given(method("POST"))
            .and(path("/mpesa/reversal/v1/request"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
            .expect(0)
            .mount(&server)
            .await;
        if let Err(e) = client
            .transaction_reversal("testapi496")
            .transaction_id("OEI2AK4Q16")
            .result_url("https://testdomain.com/ok")
            .timeout_url("https://testdomain.com/err")
            .receiver_party("600111")
            .send()
            .await
        {
            let MpesaError::Message(msg) = e else {
                panic!("Expected MpesaError::Message, but found {}", e);
            };
            assert_eq!(msg, "amount is required")
        } else {
            panic!("Expected error");
        }
    }

    #[tokio::test]
    async fn transaction_reversal_fails_if_no_result_url_is_provided() {
        let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
        let sample_response_body = json!({
            "OriginatorConversationID": "29464-48063588-1",
            "ConversationID": "AG_20230206_201056794190723278ff",
            "ResponseDescription": "Accept the service request successfully.",
        });
        Mock::given(method("POST"))
            .and(path("/mpesa/reversal/v1/request"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
            .expect(0)
            .mount(&server)
            .await;
        if let Err(e) = client
            .transaction_reversal("testapi496")
            .transaction_id("OEI2AK4Q16")
            .amount(1.0)
            .result_url("https://testdomain.com/ok")
            .receiver_party("600111")
            .send()
            .await
        {
            let MpesaError::Message(msg) = e else {
                panic!("Expected MpesaError::Message, but found {}", e);
            };
            assert_eq!(msg, "timeout_url is required")
        } else {
            panic!("Expected error");
        }
    }

    #[tokio::test]
    async fn transaction_reversal_fails_if_no_timeout_url_is_provided() {
        let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
        let sample_response_body = json!({
            "OriginatorConversationID": "29464-48063588-1",
            "ConversationID": "AG_20230206_201056794190723278ff",
            "ResponseDescription": "Accept the service request successfully.",
        });
        Mock::given(method("POST"))
            .and(path("/mpesa/reversal/v1/request"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
            .expect(0)
            .mount(&server)
            .await;
        if let Err(e) = client
            .transaction_reversal("testapi496")
            .transaction_id("OEI2AK4Q16")
            .amount(1.0)
            .timeout_url("https://testdomain.com/err")
            .receiver_party("600111")
            .send()
            .await
        {
            let MpesaError::Message(msg) = e else {
                panic!("Expected MpesaError::Message, but found {}", e);
            };
            assert_eq!(msg, "result_url is required")
        } else {
            panic!("Expected error");
        }
    }

    #[tokio::test]
    async fn transaction_reversal_fails_if_no_receiver_party_is_provided() {
        let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
        let sample_response_body = json!({
            "OriginatorConversationID": "29464-48063588-1",
            "ConversationID": "AG_20230206_201056794190723278ff",
            "ResponseDescription": "Accept the service request successfully.",
        });
        Mock::given(method("POST"))
            .and(path("/mpesa/reversal/v1/request"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
            .expect(0)
            .mount(&server)
            .await;
        if let Err(e) = client
            .transaction_reversal("testapi496")
            .transaction_id("OEI2AK4Q16")
            .amount(1.0)
            .result_url("https://testdomain.com/ok")
            .timeout_url("https://testdomain.com/err")
            .send()
            .await
        {
            let MpesaError::Message(msg) = e else {
                panic!("Expected MpesaError::Message, but found {}", e);
            };
            assert_eq!(msg, "receiver_party is required")
        } else {
            panic!("Expected error");
        }
    }
}

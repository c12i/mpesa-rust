use mpesa::services::{TransactionReversal, TransactionReversalRequest};
use mpesa::IdentifierTypes;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

#[tokio::test]
async fn transaction_reversal_success() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/reversal/v1/request"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .transaction_reversal()
        .initiator("testapi496")
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .try_timeout_url("https://testdomain.com/err")
        .unwrap()
        .transaction_id("OEI2AK4Q16")
        .amount(1.0)
        .receiver_party("600111")
        .receiver_identifier_type(IdentifierTypes::ShortCode)
        .remarks("wrong recipient")
        .build()
        .unwrap()
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
async fn transaction_reversal_test_using_struct_initialization() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/reversal/v1/request"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;

    let payload = TransactionReversalRequest {
        initiator: "testapi496",
        security_credential: "testapi496".to_string(),
        command_id: mpesa::CommandId::TransactionReversal,
        transaction_id: "OEI2AK4Q16",
        receiver_party: "600111",
        receiver_identifier_type: IdentifierTypes::ShortCode,
        result_url: "https://testdomain.com/ok".parse().unwrap(),
        queue_timeout_url: "https://testdomain.com/err".parse().unwrap(),
        remarks: "wrong recipient",
        occasion: None,
        amount: 1.0,
    };

    let response = TransactionReversal::from_request(&client, payload)
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

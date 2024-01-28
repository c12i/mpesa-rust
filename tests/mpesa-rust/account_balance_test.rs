use mpesa::services::{AccountBalance, AccountBalanceRequest};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

#[tokio::test]
async fn account_balance_using_builder_pattern() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/accountbalance/v1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .account_balance()
        .initiator_name("testapi496")
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .try_queue_timeout_url("https://testdomain.com/err")
        .unwrap()
        .party_a("600496")
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
    assert_eq!(response.response_code, "0");
}

#[tokio::test]
async fn account_balance_using_struct_initialization() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/accountbalance/v1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;
    let request = AccountBalanceRequest {
        command_id: mpesa::CommandId::AccountBalance,
        identifier_type: mpesa::IdentifierTypes::TillNumber,
        initiator: "testapi496",
        party_a: "600496",
        queue_time_out_url: "https://testdomain.com/err".try_into().unwrap(),
        remarks: "None",
        result_url: "https://testdomain.com/ok".try_into().unwrap(),
        security_credential: client.gen_security_credentials().unwrap(),
    };
    let response = AccountBalance::from_request(&client, request)
        .send()
        .await
        .unwrap();
    assert_eq!(response.originator_conversation_id, "29464-48063588-1");
    assert_eq!(response.conversation_id, "AG_20230206_201056794190723278ff");
    assert_eq!(
        response.response_description,
        "Accept the service request successfully."
    );
    assert_eq!(response.response_code, "0");
}

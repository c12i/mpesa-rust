use mpesa::MpesaError;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

#[tokio::test]
async fn account_balance_success() {
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
        .account_balance("testapi496")
        .result_url("https://testdomain.com/ok")
        .timeout_url("https://testdomain.com/err")
        .party_a("600496")
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
async fn account_balance_fails_if_party_a_is_not_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/accountbalance/v1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .account_balance("testapi496")
        .result_url("https://testdomain.com/ok")
        .timeout_url("https://testdomain.com/err")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "party_a is required")
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn account_balance_fails_if_result_url_is_not_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/accountbalance/v1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .account_balance("testapi496")
        .party_a("600496")
        .timeout_url("https://testdomain.com/err")
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
async fn account_balance_fails_if_queue_timeout_url_is_not_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/accountbalance/v1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .account_balance("testapi496")
        .party_a("600496")
        .result_url("https://testdomain.com/ok")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "queue_timeout_url is required")
    } else {
        panic!("Expected error");
    }
}

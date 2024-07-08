use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

#[tokio::test]
async fn b2c_success() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/b2c/v1/paymentrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .b2c()
        .initiator_name("testapi496")
        .party_a("600496")
        .party_b("254708374149")
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .try_queue_timeout_url("https://testdomain.com/err")
        .unwrap()
        .amount(1000)
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
async fn b2c_fails_if_no_amount_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/b2c/v1/paymentrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .b2c()
        .initiator_name("testapi496")
        .party_a("600496")
        .party_b("254708374149")
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .try_queue_timeout_url("https://testdomain.com/err")
        .unwrap()
        .build()
    {
        assert!(e.to_string().contains("Field [amount] is required"))
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn b2c_fails_if_no_party_a_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/b2c/v1/paymentrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .b2c()
        .initiator_name("testapi496")
        .amount(1000)
        .party_b("254708374149")
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .try_queue_timeout_url("https://testdomain.com/err")
        .unwrap()
        .build()
    {
        assert!(e.to_string().contains("Field [part_a] is required"))
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn b2c_fails_if_no_party_b_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/b2c/v1/paymentrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .b2c()
        .initiator_name("testapi496")
        .amount(1000)
        .party_a("600496")
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .try_queue_timeout_url("https://testdomain.com/err")
        .unwrap()
        .build()
    {
        assert!(e.to_string().contains("Field [part_b] is required"))
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn b2c_fails_if_no_result_url_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/b2c/v1/paymentrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .b2c()
        .initiator_name("testapi496")
        .amount(1000)
        .party_a("600496")
        .party_b("254708374149")
        .try_queue_timeout_url("https://testdomain.com/err")
        .unwrap()
        .build()
    {
        assert!(e.to_string().contains("Field [result_url] is required"))
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn b2c_fails_if_no_queue_timeout_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/b2c/v1/paymentrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .b2c()
        .initiator_name("testapi496")
        .amount(1000)
        .party_a("600496")
        .party_b("254708374149")
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .build()
    {
        assert!(e.to_string().contains("Field [amount] is required"))
    } else {
        panic!("Expected error");
    }
}

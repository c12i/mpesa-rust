use crate::get_mpesa_client;
use mpesa::MpesaError;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn c2b_register_success() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "OriginatorConverstionID": "29464-48063588-1",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/c2b/v1/registerurl"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .c2b_register()
        .short_code("600496")
        .confirmation_url("https://testdomain.com/true")
        .validation_url("https://testdomain.com/valid")
        .send()
        .await
        .unwrap();
    assert_eq!(response.originator_conversation_id, "29464-48063588-1");
    assert_eq!(
        response.response_description,
        "Accept the service request successfully."
    );
    assert_eq!(response.response_code, "0");
    assert_eq!(response.conversation_id, None);
}

#[tokio::test]
async fn c2b_register_fails_if_short_code_is_not_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConverstionID": "29464-48063588-1",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/c2b/v1/registerurl"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .c2b_register()
        .confirmation_url("https://testdomain.com/true")
        .validation_url("https://testdomain.com/valid")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {panic!("Expected MpesaError::Message, but found {}", e)};
        assert_eq!(msg, "short_code is required");
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn c2b_register_fails_if_confirmation_url_is_not_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConverstionID": "29464-48063588-1",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/c2b/v1/registerurl"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .c2b_register()
        .short_code("600496")
        .validation_url("https://testdomain.com/valid")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {panic!("Expected MpesaError::Message, but found {}", e)};
        assert_eq!(msg, "confirmation_url is required");
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn c2b_register_fails_if_validation_url_is_not_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConverstionID": "29464-48063588-1",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/c2b/v1/registerurl"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .c2b_register()
        .short_code("600496")
        .confirmation_url("https://testdomain.com/true")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {panic!("Expected MpesaError::Message, but found {}", e)};
        assert_eq!(msg, "validation_url is required");
    } else {
        panic!("Expected error");
    }
}

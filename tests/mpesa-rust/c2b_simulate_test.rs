use mpesa::MpesaError;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

#[tokio::test]
async fn c2b_simulate_success() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "OriginatorCoversationID": "29464-48063588-1",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/c2b/v1/simulate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .c2b_simulate()
        .amount(1000)
        .bill_ref_number("2")
        .msisdn("254700000000")
        .short_code("600496")
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
async fn c2b_simulate_fails_if_no_amount_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorCoversationID": "29464-48063588-1",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/c2b/v1/simulate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .c2b_simulate()
        .bill_ref_number("2")
        .msisdn("254700000000")
        .short_code("600496")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "amount is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn c2b_simulate_fails_if_no_short_code_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorCoversationID": "29464-48063588-1",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/c2b/v1/simulate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .c2b_simulate()
        .amount(1000)
        .bill_ref_number("2")
        .msisdn("254700000000")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "short_code is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn c2b_simulate_fails_if_no_bill_ref_number_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorCoversationID": "29464-48063588-1",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/c2b/v1/simulate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .c2b_simulate()
        .amount(1000)
        .msisdn("254700000000")
        .short_code("600496")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "bill_ref_number is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn c2b_simulate_fails_if_no_msisdn_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorCoversationID": "29464-48063588-1",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/c2b/v1/simulate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .c2b_simulate()
        .amount(1000)
        .bill_ref_number("2")
        .short_code("600496")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "msisdn is required");
    } else {
        panic!("Expected error")
    }
}

use crate::get_mpesa_client;
use mpesa::MpesaError;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn stk_push_success_success() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "MerchantRequestID": "16813-1590513-1",
        "CheckoutRequestID": "ws_CO_DMZ_12321_23423476",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0",
        "CustomerMessage": "Success. Request accepeted for processing"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/stkpush/v1/processrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .express_request()
        .business_short_code("174379")
        .phone_number("254708374149")
        .amount(500)
        .callback_url("https://test.example.com/api")
        .build()
        .unwrap()
        .send()
        .await
        .unwrap();
    assert_eq!(response.merchant_request_id, "16813-1590513-1");
    assert_eq!(response.checkout_request_id, "ws_CO_DMZ_12321_23423476");
    assert_eq!(
        response.response_description,
        "Accept the service request successfully."
    );
    assert_eq!(
        response.customer_message,
        "Success. Request accepeted for processing"
    );
}

#[tokio::test]
async fn stk_push_fails_if_no_amount_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "MerchantRequestID": "16813-1590513-1",
        "CheckoutRequestID": "ws_CO_DMZ_12321_23423476",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0",
        "CustomerMessage": "Success. Request accepeted for processing"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/stkpush/v1/processrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .express_request()
        .business_short_code("174379")
        .phone_number("254708374149")
        .callback_url("https://test.example.com/api")
        .build()
        .unwrap()
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
async fn stk_push_fails_if_no_callback_url_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "MerchantRequestID": "16813-1590513-1",
        "CheckoutRequestID": "ws_CO_DMZ_12321_23423476",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0",
        "CustomerMessage": "Success. Request accepeted for processing"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/stkpush/v1/processrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .express_request()
        .business_short_code("174379")
        .phone_number("254708374149")
        .amount(500)
        .build()
        .unwrap()
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "callback_url is required")
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn stk_push_fails_if_no_phone_number_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "MerchantRequestID": "16813-1590513-1",
        "CheckoutRequestID": "ws_CO_DMZ_12321_23423476",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0",
        "CustomerMessage": "Success. Request accepeted for processing"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/stkpush/v1/processrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .express_request()
        .business_short_code("174379")
        .amount(500)
        .callback_url("https://test.example.com/api")
        .build()
        .unwrap()
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "phone_number is required")
    } else {
        panic!("Expected error");
    }
}

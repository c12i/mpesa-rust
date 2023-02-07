use crate::get_mpesa_client;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn stk_push_test() {
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
        .express_request("174379")
        .phone_number("254708374149")
        .amount(500)
        .callback_url("https://test.example.com/api")
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

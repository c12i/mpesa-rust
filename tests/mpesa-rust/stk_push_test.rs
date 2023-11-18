use mpesa::services::{MpesaExpress, MpesaExpressRequest};
use mpesa::CommandId;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;
use crate::helpers::TestEnvironment;

#[tokio::test]
async fn stk_push_success() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "MerchantRequestID": "16813-1590513-1",
        "CheckoutRequestID": "ws_CO_DMZ_12321_23423476",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0",
        "CustomerMessage": "Success. Request accepted for processing"
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
        .transaction_type(mpesa::CommandId::BusinessBuyGoods)
        .party_a("254708374149")
        .party_b("174379")
        .account_ref("test")
        .phone_number("254708374149")
        .amount(500)
        .pass_key("test")
        .try_callback_url("https://test.example.com/api")
        .unwrap()
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
        "Success. Request accepted for processing"
    );
}

#[tokio::test]
async fn stk_push_only_accepts_specific_tx_type() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "MerchantRequestID": "16813-1590513-1",
        "CheckoutRequestID": "ws_CO_DMZ_12321_23423476",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0",
        "CustomerMessage": "Success. Request accepted for processing"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/stkpush/v1/processrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    let err = client
        .express_request()
        .business_short_code("174379")
        .transaction_type(mpesa::CommandId::SalaryPayment)
        .party_a("254708374149")
        .party_b("174379")
        .account_ref("test")
        .phone_number("254708374149")
        .amount(500)
        .pass_key("test")
        .try_callback_url("https://test.example.com/api")
        .unwrap()
        .build()
        .unwrap_err();

    assert_eq!(
        err.to_string(),
        "Invalid transaction type. Expected BusinessBuyGoods or CustomerPayBillOnline"
    );
}

#[tokio::test]
async fn express_request_test_using_struct_initialization() {
    let (client, server) = get_mpesa_client!();

    let sample_response_body = json!({
        "MerchantRequestID": "16813-1590513-1",
        "CheckoutRequestID": "ws_CO_DMZ_12321_23423476",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0",
        "CustomerMessage": "Success. Request accepted for processing"
    });

    let password = MpesaExpress::<TestEnvironment>::encode_password("174379", None);

    let request = MpesaExpressRequest {
        business_short_code: "174379",
        transaction_type: CommandId::BusinessBuyGoods,
        amount: 500.0,
        party_a: "254708374149",
        party_b: "174379",
        phone_number: "254708374149",
        password,
        timestamp: chrono::Local::now(),
        call_back_url: "https://test.example.com/api".try_into().unwrap(),
        account_reference: "test",
        transaction_desc: None,
    };

    Mock::given(method("POST"))
        .and(path("/mpesa/stkpush/v1/processrequest"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;

    let request = MpesaExpress::from_request(&client, request, None);

    let response = request.send().await.unwrap();

    assert_eq!(response.merchant_request_id, "16813-1590513-1");
    assert_eq!(response.checkout_request_id, "ws_CO_DMZ_12321_23423476");
    assert_eq!(
        response.response_description,
        "Accept the service request successfully."
    );
    assert_eq!(
        response.customer_message,
        "Success. Request accepted for processing"
    );
}

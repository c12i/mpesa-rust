use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

#[tokio::test]
async fn dynamic_qr_code_test() {
    let (client, server) = get_mpesa_client!();

    let sample_response_body = json!({
        "QRCode": "A3F7B1H",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });

    Mock::given(method("POST"))
        .and(path("/mpesa/qrcode/v1/generate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .dynamic_qr()
        .amount(2000)
        .credit_party_identifier("17408")
        .merchant_name("SafaricomLTD")
        .ref_no("rf38f04")
        .size("300")
        .try_transaction_type("bg")
        // .try_transaction_type(TransactionType::BuyGoods) // This is the same as the above
        .unwrap()
        .build()
        .unwrap()
        .send()
        .await
        .unwrap();

    assert_eq!(
        response.response_description,
        "Accept the service request successfully."
    );
    assert_eq!(response.response_code, "0");
}

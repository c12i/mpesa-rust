use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

#[tokio::test]
async fn dynamic_qr_code_test() {
    let (client, server) = get_mpesa_client!();

    // let response = client
    //     .dynamic_qrcode()
    //     .amount(2000)
    //     .credit_party_identifier("17408")
    //     .merchant_name("SafaricomLTD")
    //     .ref_no("rf38f04")
    //     .trx_code(mpesa::TransactionType::BG)
    //     .send()
    //     .await;
    // eprintln!("RES {response:?}");
    // assert!(response.is_ok())

    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });

    Mock::given(method("POST"))
        .and(path("/mpesa/dynamicqr/v1/create"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .dynamic_qrcode()
        .amount(2000)
        .credit_party_identifier("17408")
        .merchant_name("SafaricomLTD")
        .ref_no("rf38f04")
        .trx_code(mpesa::TransactionType::BG)
        .send()
        .await
        .unwrap();

    // assert_eq!(response.originator_conversation_id, "29464-48063588-1");
}

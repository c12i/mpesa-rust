use crate::get_mpesa_client;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn c2b_register_test() {
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

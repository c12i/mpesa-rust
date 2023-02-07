use mpesa::IdentifierTypes;

use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

#[tokio::test]
async fn transaction_status_success() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/transactionstatus/v1/query"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .transaction_status("testapi496")
        .result_url("https://testdomain.com/ok")
        .timeout_url("https://testdomain.com/err")
        .transaction_id("OEI2AK4Q16")
        .identifier_type(IdentifierTypes::ShortCode)
        .party_a("600111")
        .remarks("status")
        .occasion("work")
        .send()
        .await
        .unwrap();
    assert_eq!(response.originator_conversation_id, "29464-48063588-1");
    assert_eq!(response.conversation_id, "AG_20230206_201056794190723278ff");
    assert_eq!(
        response.response_description,
        "Accept the service request successfully."
    );
}

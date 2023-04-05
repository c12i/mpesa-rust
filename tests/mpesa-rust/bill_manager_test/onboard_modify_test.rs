use crate::get_mpesa_client;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

fn sample_response() -> ResponseTemplate {
    let sample_response_body = json!({
        "rescode": "200",
        "resmsg": "Biller updated successfully"
    });
    ResponseTemplate::new(200).set_body_json(sample_response_body)
}

#[tokio::test]
async fn onboard_modify_success() {
    let (client, server) = get_mpesa_client!();
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/change-optin-details"))
        .respond_with(sample_response())
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .onboard_modify()
        .callback_url("https://testdomain.com/true")
        .email("email@test.com")
        .logo("https://file.domain/file.png")
        .send()
        .await
        .unwrap();
    assert_eq!(response.res_code, "200");
    assert_eq!(response.res_msg, "Biller updated successfully");
}

use mpesa::MpesaError;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

fn sample_response() -> ResponseTemplate {
    let sample_response = json!({
        "rescode": "200",
        "resmsg": "Success",
        "Status_Message": "Invoices cancelled successfully"
    });
    ResponseTemplate::new(200).set_body_json(sample_response)
}

#[tokio::test]
async fn cancel_single_invoice_success() {
    let (client, server) = get_mpesa_client!();
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/cancel-single-invoice"))
        .respond_with(sample_response())
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .cancel_single_invoice()
        .external_reference("87TH7JK1")
        .build()
        .unwrap()
        .send()
        .await
        .unwrap();
    assert_eq!(response.response_code, "200");
    assert_eq!(response.response_message, "Success");
    assert_eq!(response.status_message, "Invoices cancelled successfully");
}

#[tokio::test]
async fn cancel_single_invoice_fails_if_no_external_reference_is_provided() {
    let (client, server) = get_mpesa_client!();
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/cancel-single-invoice"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client.cancel_single_invoice().build() {
        let MpesaError::BuilderError(err) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(err.to_string(), "Field [external_reference] is required");
    } else {
        panic!("Expected error")
    }
}

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
async fn cancel_bulk_invoices_success() {
    let (client, server) = get_mpesa_client!();
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/cancel-bulk-invoices"))
        .respond_with(sample_response())
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .cancel_bulk_invoices()
        .external_references(vec!["9KLSS011"])
        .external_reference("87TH7JK1")
        .send()
        .await
        .unwrap();
    assert_eq!(response.response_code, "200");
    assert_eq!(response.response_message, "Success");
    assert_eq!(response.status_message, "Invoices cancelled successfully");
}

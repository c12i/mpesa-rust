use crate::get_mpesa_client;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

fn sample_response() -> ResponseTemplate {
    let sample_response = json!({
        "rescode": "200",
        "resmsg": "Success",
        "Status_Message": "Invoice cancelled successfully"
    });
    ResponseTemplate::new(200).set_body_json(sample_response)
}

#[tokio::test]
async fn bill_manager_cancel_invoice_success() {
    let (client, server) = get_mpesa_client!();
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/cancel-single-invoice"))
        .respond_with(sample_response())
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .bill_manager_cancel_invoice()
        .external_references(vec!["9KLSS011"])
        .send()
        .await
        .unwrap();
    assert_eq!(response.res_code, "200");
    assert_eq!(response.res_msg, "Success");
    assert_eq!(response.status_message, "Invoice cancelled successfully");
}

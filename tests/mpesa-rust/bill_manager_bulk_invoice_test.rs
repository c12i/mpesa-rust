use crate::get_mpesa_client;
use chrono::prelude::Utc;
use mpesa::{Invoice, InvoiceItem};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

fn sample_response() -> ResponseTemplate {
    let sample_response = json!({
        "rescode": "200",
        "resmsg": "Success",
        "Status_Message": "Invoice sent successfully"
    });
    ResponseTemplate::new(200).set_body_json(sample_response)
}

#[tokio::test]
async fn bill_manager_bulk_invoice_success() {
    let (client, server) = get_mpesa_client!();
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/bulk-invoicing"))
        .respond_with(sample_response())
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .bill_manager_bulk_invoice()
        .add_invoice(Invoice {
            amount: 1000.0,
            account_reference: "John Doe",
            billed_full_name: "John Doe",
            billed_period: "August 2021",
            billed_phone_number: "0712345678",
            due_date: Utc::now(),
            external_reference: "INV2345",
            invoice_items: Some(vec![InvoiceItem {
                amount: 1000.0,
                item_name: "An item",
            }]),
            invoice_name: "Invoice 001",
        })
        .send()
        .await
        .unwrap();
    assert_eq!(response.res_code, "200");
    assert_eq!(response.res_msg, "Success");
    assert_eq!(response.status_message, "Invoice sent successfully");
}

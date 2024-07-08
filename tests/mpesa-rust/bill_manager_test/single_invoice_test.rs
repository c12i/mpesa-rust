use chrono::prelude::Utc;
use mpesa::services::InvoiceItem;
use mpesa::MpesaError;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

fn sample_response() -> ResponseTemplate {
    let sample_response = json!({
        "rescode": "200",
        "resmsg": "Success",
        "Status_Message": "Invoice sent successfully"
    });
    ResponseTemplate::new(200).set_body_json(sample_response)
}

#[tokio::test]
async fn single_invoice_success() {
    let (client, server) = get_mpesa_client!();
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/single-invoicing"))
        .respond_with(sample_response())
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .single_invoice()
        .amount(1000.0)
        .account_reference("John Doe")
        .billed_full_name("John Doe")
        .billed_period("August 2021")
        .billed_phone_number("0712345678")
        .due_date(Utc::now())
        .external_reference("INV2345")
        .invoice_items(vec![InvoiceItem {
            amount: 1000.0,
            item_name: "An item",
        }])
        .invoice_name("Invoice 001")
        .send()
        .await
        .unwrap();
    assert_eq!(response.response_code, "200");
    assert_eq!(response.response_message, "Success");
    assert_eq!(response.status_message, "Invoice sent successfully");
}

#[tokio::test]
async fn single_invoice_fails_if_no_amount_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/single-invoicing"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .single_invoice()
        .account_reference("John Doe")
        .billed_full_name("John Doe")
        .billed_period("August 2021")
        .billed_phone_number("0712345678")
        .due_date(Utc::now())
        .external_reference("INV2345")
        .invoice_name("Invoice 001")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "amount is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn single_invoice_fails_if_no_account_reference_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/single-invoicing"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .single_invoice()
        .amount(1000.0)
        .billed_full_name("John Doe")
        .billed_period("August 2021")
        .billed_phone_number("0712345678")
        .due_date(Utc::now())
        .external_reference("INV2345")
        .invoice_name("Invoice 001")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "account_reference is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn single_invoice_fails_if_no_billed_full_name_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/single-invoicing"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .single_invoice()
        .amount(1000.0)
        .account_reference("John Doe")
        .billed_period("August 2021")
        .billed_phone_number("0712345678")
        .due_date(Utc::now())
        .external_reference("INV2345")
        .invoice_name("Invoice 001")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "billed_full_name is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn single_invoice_fails_if_no_billed_period_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/single-invoicing"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .single_invoice()
        .amount(1000.0)
        .account_reference("John Doe")
        .billed_full_name("John Doe")
        .billed_phone_number("0712345678")
        .due_date(Utc::now())
        .external_reference("INV2345")
        .invoice_name("Invoice 001")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "billed_period is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn single_invoice_fails_if_no_billed_phone_number_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/single-invoicing"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .single_invoice()
        .amount(1000.0)
        .account_reference("John Doe")
        .billed_full_name("John Doe")
        .billed_period("August 2021")
        .due_date(Utc::now())
        .external_reference("INV2345")
        .invoice_name("Invoice 001")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message but found {}", e);
        };
        assert_eq!(msg, "billed_phone_number is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn single_invoice_fails_if_no_due_date_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/single-invoicing"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .single_invoice()
        .amount(1000.0)
        .account_reference("John Doe")
        .billed_full_name("John Doe")
        .billed_period("August 2021")
        .billed_phone_number("0712345678")
        .external_reference("INV2345")
        .invoice_name("Invoice 001")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "due_date is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn single_invoice_fails_if_no_external_reference_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/single-invoicing"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .single_invoice()
        .amount(1000.0)
        .account_reference("John Doe")
        .billed_full_name("John Doe")
        .billed_period("August 2021")
        .billed_phone_number("0712345678")
        .due_date(Utc::now())
        .invoice_name("Invoice 001")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "external_reference is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn single_invoice_fails_if_no_invoice_name_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/single-invoicing"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .single_invoice()
        .amount(1000.0)
        .account_reference("John Doe")
        .billed_full_name("John Doe")
        .billed_period("August 2021")
        .billed_phone_number("0712345678")
        .due_date(Utc::now())
        .external_reference("INV2345")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "invoice_name is required");
    } else {
        panic!("Expected error")
    }
}

use crate::get_mpesa_client;
use chrono::prelude::Utc;
use mpesa::MpesaError;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

fn sample_response() -> ResponseTemplate {
    let sample_response = json!({
        "rescode": "200",
        "resmsg": "Success",
    });
    ResponseTemplate::new(200).set_body_json(sample_response)
}

#[tokio::test]
async fn reconciliation_success() {
    let (client, server) = get_mpesa_client!();
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/reconciliation"))
        .respond_with(sample_response())
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .reconciliation()
        .account_reference("John Doe")
        .external_reference("INV2345")
        .full_name("John Doe")
        .invoice_name("Invoice 001")
        .paid_amount(1000.0)
        .payment_date(Utc::now())
        .phone_number("0712345678")
        .transaction_id("TRANSACTION_ID")
        .send()
        .await
        .unwrap();
    assert_eq!(response.response_code, "200");
    assert_eq!(response.response_message, "Success");
}

#[tokio::test]
async fn reconciliation_fails_if_no_account_reference_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/reconciliation"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .reconciliation()
        .external_reference("INV2345")
        .full_name("John Doe")
        .invoice_name("Invoice 001")
        .paid_amount(1000.0)
        .payment_date(Utc::now())
        .phone_number("0712345678")
        .transaction_id("TRANSACTION_ID")
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
async fn reconciliation_fails_if_no_external_reference_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/reconciliation"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .reconciliation()
        .account_reference("John Doe")
        .full_name("John Doe")
        .invoice_name("Invoice 001")
        .paid_amount(1000.0)
        .payment_date(Utc::now())
        .phone_number("0712345678")
        .transaction_id("TRANSACTION_ID")
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
async fn reconciliation_fails_if_no_full_name_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/reconciliation"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .reconciliation()
        .account_reference("John Doe")
        .external_reference("INV2345")
        .invoice_name("Invoice 001")
        .paid_amount(1000.0)
        .payment_date(Utc::now())
        .phone_number("0712345678")
        .transaction_id("TRANSACTION_ID")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "full_name is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn reconciliation_fails_if_no_invoice_name_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/reconciliation"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .reconciliation()
        .account_reference("John Doe")
        .external_reference("INV2345")
        .full_name("John Doe")
        .paid_amount(1000.0)
        .payment_date(Utc::now())
        .phone_number("0712345678")
        .transaction_id("TRANSACTION_ID")
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

#[tokio::test]
async fn reconciliation_fails_if_no_paid_amount_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/reconciliation"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .reconciliation()
        .account_reference("John Doe")
        .external_reference("INV2345")
        .full_name("John Doe")
        .invoice_name("Invoice 001")
        .payment_date(Utc::now())
        .phone_number("0712345678")
        .transaction_id("TRANSACTION_ID")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "paid_amount is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn reconciliation_fails_if_no_payment_date_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/reconciliation"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .reconciliation()
        .account_reference("John Doe")
        .external_reference("INV2345")
        .full_name("John Doe")
        .invoice_name("Invoice 001")
        .paid_amount(1000.0)
        .phone_number("0712345678")
        .transaction_id("TRANSACTION_ID")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "payment_date is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn reconciliation_fails_if_no_phone_number_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/reconciliation"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .reconciliation()
        .account_reference("John Doe")
        .external_reference("INV2345")
        .full_name("John Doe")
        .invoice_name("Invoice 001")
        .paid_amount(1000.0)
        .payment_date(Utc::now())
        .transaction_id("TRANSACTION_ID")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "phone_number is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn reconciliation_fails_if_no_transaction_id_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/reconciliation"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .reconciliation()
        .account_reference("John Doe")
        .external_reference("INV2345")
        .full_name("John Doe")
        .invoice_name("Invoice 001")
        .paid_amount(1000.0)
        .payment_date(Utc::now())
        .phone_number("0712345678")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "transaction_id is required");
    } else {
        panic!("Expected error")
    }
}

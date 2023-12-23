use chrono::prelude::Utc;
use mpesa::MpesaError;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

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
        .date_created(Utc::now())
        .msisdn("0712345678")
        .paid_amount(1000.0)
        .short_code("600496")
        .transaction_id("TRANSACTION_ID")
        .build()
        .unwrap()
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
        .date_created(Utc::now())
        .msisdn("0712345678")
        .paid_amount(1000.0)
        .short_code("600496")
        .transaction_id("TRANSACTION_ID")
        .build()
    {
        let MpesaError::BuilderError(err) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(err.to_string(), "Field [account_reference] is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn reconciliation_fails_if_no_date_created_is_provided() {
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
        .msisdn("0712345678")
        .paid_amount(1000.0)
        .short_code("600496")
        .transaction_id("TRANSACTION_ID")
        .build()
    {
        let MpesaError::BuilderError(err) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(err.to_string(), "Field [date_created] is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn reconciliation_fails_if_no_msisdn_is_provided() {
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
        .date_created(Utc::now())
        .paid_amount(1000.0)
        .short_code("600496")
        .transaction_id("TRANSACTION_ID")
        .build()
    {
        let MpesaError::BuilderError(err) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(err.to_string(), "Field [msisdn] is required");
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
        .date_created(Utc::now())
        .msisdn("0712345678")
        .short_code("600496")
        .transaction_id("TRANSACTION_ID")
        .build()
    {
        let MpesaError::BuilderError(err) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(err.to_string(), "Field [paid_amount] is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn reconciliation_fails_if_no_short_code_is_provided() {
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
        .date_created(Utc::now())
        .msisdn("0712345678")
        .paid_amount(1000.0)
        .transaction_id("TRANSACTION_ID")
        .build()
    {
        let MpesaError::BuilderError(err) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(err.to_string(), "Field [short_code] is required");
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
        .date_created(Utc::now())
        .msisdn("0712345678")
        .short_code("600496")
        .paid_amount(1000.0)
        .build()
    {
        let MpesaError::BuilderError(err) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(err.to_string(), "Field [transaction_id] is required");
    } else {
        panic!("Expected error")
    }
}

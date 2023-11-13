use mpesa::MpesaError;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::get_mpesa_client;

fn sample_response() -> ResponseTemplate {
    let sample_response_body = json!({
        "app_key": "kfpB9X4o0H",
        "rescode": "200",
        "resmsg": "Success"
    });
    ResponseTemplate::new(200).set_body_json(sample_response_body)
}

#[tokio::test]
async fn onboard_success() {
    let (client, server) = get_mpesa_client!();
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/optin"))
        .respond_with(sample_response())
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .onboard()
        .callback_url("https://testdomain.com/true")
        .email("email@test.com")
        .logo("https://file.domain/file.png")
        .official_contact("0712345678")
        .short_code("600496")
        .send()
        .await
        .unwrap();
    assert_eq!(response.app_key, "kfpB9X4o0H");
    assert_eq!(response.response_code, "200");
    assert_eq!(response.response_message, "Success");
}

#[tokio::test]
async fn onboard_fails_if_no_callback_url_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/optin"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .onboard()
        .email("email@test.com")
        .logo("https://file.domain/file.png")
        .official_contact("0712345678")
        .short_code("600496")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "callback_url is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn onboard_fails_if_no_email_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/optin"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .onboard()
        .callback_url("https://testdomain.com/true")
        .logo("https://file.domain/file.png")
        .official_contact("0712345678")
        .short_code("600496")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "email is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn onboard_fails_if_no_logo_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/optin"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .onboard()
        .callback_url("https://testdomain.com/true")
        .email("email@test.com")
        .official_contact("0712345678")
        .short_code("600496")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "logo is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn onboard_fails_if_no_official_contact_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/optin"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .onboard()
        .callback_url("https://testdomain.com/true")
        .email("email@test.com")
        .logo("https://file.domain/file.png")
        .short_code("600496")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "official_contact is required");
    } else {
        panic!("Expected error")
    }
}

#[tokio::test]
async fn onboard_fails_if_short_code_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    Mock::given(method("POST"))
        .and(path("/v1/billmanager-invoice/optin"))
        .respond_with(sample_response())
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .onboard()
        .callback_url("https://testdomain.com/true")
        .email("email@test.com")
        .logo("https://file.domain/file.png")
        .official_contact("0712345678")
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "short_code is required");
    } else {
        panic!("Expected error")
    }
}

use crate::get_mpesa_client;
use derive_builder::UninitializedFieldError;
use mpesa::{BuilderError, IdentifierTypes, MpesaError};
use serde_json::json;
use url::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn transaction_reversal_success() {
    let (client, server) = get_mpesa_client!();
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
        "ResponseCode": "0"
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/reversal/v1/request"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(1)
        .mount(&server)
        .await;
    let response = client
        .transaction_reversal()
        .initiator("testapi496")
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .try_timeout_url("https://testdomain.com/err")
        .unwrap()
        .transaction_id("OEI2AK4Q16")
        .amount(1.0)
        .receiver_party("600111")
        .receiver_identifier_type(IdentifierTypes::ShortCode)
        .remarks("wrong recipient")
        .build()
        .unwrap()
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

#[tokio::test]
async fn transaction_reversal_fails_if_no_transaction_id_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/reversal/v1/request"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    let err = client
        .transaction_reversal()
        .initiator("testapi496")
        .result_url(Url::parse("https://testdomain.com/ok").unwrap())
        .try_timeout_url("https://testdomain.com/err")
        .unwrap()
        .amount(1.0)
        .receiver_party("600111")
        .build()
        .unwrap_err();

    assert_eq!(err.to_string(), "Field transaction_id is required");
}

#[tokio::test]
async fn transaction_reversal_fails_if_no_amount_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/reversal/v1/request"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .transaction_reversal()
        .initiator("testapi496")
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .try_timeout_url("https://testdomain.com/err")
        .unwrap()
        .transaction_id("OEI2AK4Q16")
        .receiver_party("600111")
        .build()
        .unwrap()
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "amount is required")
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn transaction_reversal_fails_if_no_result_url_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/reversal/v1/request"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .transaction_reversal()
        .initiator("testapi496")
        .transaction_id("OEI2AK4Q16")
        .amount(1.0)
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .receiver_party("600111")
        .build()
        .unwrap()
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "timeout_url is required")
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn transaction_reversal_fails_if_no_timeout_url_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/reversal/v1/request"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .transaction_reversal()
        .initiator("testapi496")
        .transaction_id("OEI2AK4Q16")
        .amount(1.0)
        .try_timeout_url("https://testdomain.com/err")
        .unwrap()
        .receiver_party("600111")
        .build()
        .unwrap()
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "result_url is required")
    } else {
        panic!("Expected error");
    }
}

#[tokio::test]
async fn transaction_reversal_fails_if_no_receiver_party_is_provided() {
    let (client, server) = get_mpesa_client!(expected_auth_requests = 0);
    let sample_response_body = json!({
        "OriginatorConversationID": "29464-48063588-1",
        "ConversationID": "AG_20230206_201056794190723278ff",
        "ResponseDescription": "Accept the service request successfully.",
    });
    Mock::given(method("POST"))
        .and(path("/mpesa/reversal/v1/request"))
        .respond_with(ResponseTemplate::new(200).set_body_json(sample_response_body))
        .expect(0)
        .mount(&server)
        .await;
    if let Err(e) = client
        .transaction_reversal()
        .initiator("testapi496")
        .transaction_id("OEI2AK4Q16")
        .amount(1.0)
        .try_result_url("https://testdomain.com/ok")
        .unwrap()
        .try_timeout_url("https://testdomain.com/err")
        .unwrap()
        .build()
        .unwrap()
        .send()
        .await
    {
        let MpesaError::Message(msg) = e else {
            panic!("Expected MpesaError::Message, but found {}", e);
        };
        assert_eq!(msg, "receiver_party is required")
    } else {
        panic!("Expected error");
    }
}

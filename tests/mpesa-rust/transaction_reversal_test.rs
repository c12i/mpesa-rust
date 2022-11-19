use crate::get_mpesa_client;

#[tokio::test]
async fn transaction_reversal_test() {
    let client = get_mpesa_client!();

    let response = client
        .transaction_reversal("testapi496")
        .result_url("https://testdomain.com/ok")
        .timeout_url("https://testdomain.com/err")
        .transaction_id("OEI2AK4Q16")
        .receiver_identifier_type("5")
        .amount(1.0)
        .receiver_party("600983")
        .remarks("wrong recipient")
        .send()
        .await;

    assert!(response.is_ok())
}

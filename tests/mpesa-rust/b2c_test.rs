use crate::get_mpesa_client;

#[tokio::test]
async fn b2c_test() {
    let client = get_mpesa_client!();

    let response = client
        .b2c("testapi496")
        .party_a("600496")
        .party_b("254708374149")
        .result_url("https://testdomain.com/ok")
        .timeout_url("https://testdomain.com/err")
        .amount(1000)
        .send()
        .await;

    assert!(response.is_ok())
}

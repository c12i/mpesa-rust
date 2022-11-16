use crate::get_mpesa_client;

#[tokio::test]
async fn account_balance_test() {
    let client = get_mpesa_client!();

    let response = client
        .account_balance("testapi496")
        .result_url("https://testdomain.com/ok")
        .timeout_url("https://testdomain.com/err")
        .party_a("600496")
        .send()
        .await;

    assert!(response.is_ok())
}

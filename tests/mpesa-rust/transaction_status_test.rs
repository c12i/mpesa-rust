use mpesa::IdentifierTypes;

use crate::get_mpesa_client;

#[tokio::test]
async fn transaction_reversal_test() {
    let client = get_mpesa_client!();

    let response = client
        .transaction_status("testapi496")
        .result_url("https://testdomain.com/ok")
        .timeout_url("https://testdomain.com/err")
        .transaction_id("OEI2AK4Q16")
        .identifier_type(IdentifierTypes::ShortCode)
        .party_a("600111")
        .remarks("status")
        .occasion("work")
        .send()
        .await;
    assert!(response.is_ok())
}

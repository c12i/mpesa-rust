use crate::get_mpesa_client;

#[tokio::test]
async fn c2b_simulate_test() {
    let client = get_mpesa_client!();

    let response = client
        .c2b_simulate()
        .short_code("600496")
        .msisdn("254700000000")
        .amount(1000)
        .send()
        .await;

    assert!(response.is_ok())
}

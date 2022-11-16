use crate::get_mpesa_client;

#[tokio::test]
async fn stk_push_test() {
    let client = get_mpesa_client!();

    let response = client
        .express_request("174379")
        .phone_number("254708374149")
        .amount(500)
        .callback_url("https://test.example.com/api")
        .send()
        .await;

    assert!(response.is_ok())
}

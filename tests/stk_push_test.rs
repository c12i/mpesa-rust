use dotenv;
use mpesa::Mpesa;
use std::env;

#[tokio::test]
async fn stk_push_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        "sandbox".parse().unwrap(),
    );

    let response = client
        .express_request("174379")
        .phone_number("254708374149")
        .amount(500)
        .callback_url("https://test.example.com/api")
        .send()
        .await;

    assert!(response.is_ok())
}

use dotenv;
use mpesa::{Mpesa, Sandbox};
use std::env;

#[test]
fn account_balance_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Sandbox,
    );

    let response = client
        .account_balance("testapi496")
        .result_url("https://testdomain.com/ok")
        .timeout_url("https://testdomain.com/err")
        .party_a("600496")
        .send();

    assert!(response.is_ok())
}

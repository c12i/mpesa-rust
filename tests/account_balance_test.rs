use dotenv;
use mpesa::{Environment, Mpesa};
use std::env;

#[test]
fn account_balance_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .account_balance("testapi496")
        .urls("https://testdomain.com/err", "https://testdomain.com/ok")
        .party_a("600496")
        .send();

    assert!(response.is_ok())
}

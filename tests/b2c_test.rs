use dotenv;
use mpesa::{Environment, Mpesa};
use std::env;

#[test]
#[ignore] // temporary since it's in maintenance
fn b2c_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .b2c("testapi496")
        .parties("600496", "254708374149")
        .urls("https://testdomain.com/err", "https://testdomain.com/res")
        .amount(1000)
        .send();

    assert!(response.is_ok())
}

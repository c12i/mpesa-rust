use dotenv;
use mpesa::Mpesa;
use std::env;

#[test]
fn b2b_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        "sandbox".parse().unwrap(),
    );

    let response = client
        .b2b("testapi496")
        .party_a("600496")
        .party_b("600000")
        .result_url("https://testdomain.com/ok")
        .timeout_url("https://testdomain.com/err")
        .account_ref("254708374149")
        .amount(1000)
        .send();

    assert!(response.is_ok())
}

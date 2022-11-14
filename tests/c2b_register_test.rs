use dotenv;
use mpesa::Mpesa;
use std::env;

#[test]
#[ignore = "depreciated"]
#[allow(deprecated)]
fn c2b_register_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        "sandbox".parse().unwrap(),
    );

    let response = client
        .c2b_register()
        .short_code("600496")
        .confirmation_url("https://testdomain.com/true")
        .validation_url("https://testdomain.com/valid")
        .send();

    assert!(response.is_ok())
}

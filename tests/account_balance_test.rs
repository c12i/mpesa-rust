use dotenv;
use mpesa::{CommandId, Environment, Mpesa};
use std::env;

#[test]
fn b2b_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox, // or environment variable
        env::var("INIT_PASSWORD").unwrap(),
    );

    let account_balance_response = client
        .account_balance(
            "600496",
            "none",
            "collins",
            "https://hell.world/api",
            "https://hello.world/api",
        )
        .unwrap();

    println!("B2b response -> {:#?}", account_balance_response);

    assert_eq!(account_balance_response.ResponseCode, "0".to_string());
}

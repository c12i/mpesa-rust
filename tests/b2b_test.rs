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

    let b2b_response = client
        .b2b(
            "testapi496",
            CommandId::BusinessToBusinessTransfer,
            1000,
            "600496",
            4,
            "600000",
            4,
            "gg",
            "https://muriuki.dev",
            "https://muriuki.dev/blog",
            "254708374149",
        )
        .unwrap();

    println!("B2b response -> {:#?}", b2b_response);

    assert_eq!(b2b_response.ResponseCode, "0".to_string());
}

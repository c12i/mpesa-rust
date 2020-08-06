use mpesa::{Environment,Mpesa,ResponseType};
use dotenv;
use std::env;

#[test]
fn c2b_register_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox, // or environment variable
        env::var("INIT_PASSWORD").unwrap(),
    );

    let c2b_register_response = client.c2b_register(
        "https://muriuki.dev/api",
        "https://muriuki.dev/verify",
        ResponseType::Complete,
        "600496"
    ).unwrap();

    assert_eq!("https://sandbox.safaricom.co.ke/mpesa/c2b/v1/registerurl", c2b_register_response.url().as_str());
}
use dotenv;
use std::collections::HashMap;
use std::env;
use reqwest::blocking::Client;

use mpesa::{Mpesa, Environment, CommandId};


fn main() {
    // auth_test().unwrap();
    b2c_test();
}

fn auth_test() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let environment: Environment = "sandbox".parse()?;

    let client = Mpesa::new(
        env::var("CLIENT_KEY")?,
        env::var("CLIENT_SECRET")?,
        Environment::Sandbox, // or environment variable
        env::var("INIT_PASSWORD")?,
    );

    let token = client.auth().unwrap();

    println!("token ==> {:?}", token);
    Ok(())
}

fn b2c_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox, // or environment variable
        env::var("INIT_PASSWORD").unwrap(),
    );

    let b2c_response = client.b2c(
        "testapi496",
        CommandId::BusinessPayment,
        1000,
        "600496",
        "254708374149",
        "gg",
        "https://muriuki.dev",
        "https://muriuki.dev/blog",
        "Test",
    ).unwrap();

    println!("{:?}", b2c_response);
}



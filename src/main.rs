#![allow(unused)]
use dotenv;
use std::collections::HashMap;
use std::env;
use reqwest::blocking::Client;

use mpesa::{Mpesa, Environment};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let environment: Environment = "sandbox".parse()?;

    let client = Mpesa::new(
        env::var("CLIENT_KEY")?,
        env::var("CLIENT_SECRET")?,
        Environment::Sandbox, // or environment variable
    );

    let token = client.auth().unwrap();

    println!("token ==> {:?}", token);

    // let client = Client::new();
    // let res = client.post("http://httpbin.org/post")
    //     .body("the exact body that is sent")
    //     .send()?;
    //
    // println!("{:#?}", res);
    Ok(())
}
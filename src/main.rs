#![allow(unused)]
extern crate reqwest;

use dotenv;
use std::collections::HashMap;
use std::env;

use mpesa::{map, Mpesa, Environment};


use reqwest::blocking::{Client, self};

fn generate_creds() -> std::collections::HashMap<&'static str,&'static str> {
    map!(
        "production" => "https://api.safaricom.co.ke",
        "sandbox" => "https://sandbox.safaricom.co.ke"
    )
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let constants = generate_creds();

    // safaricom credentials
    dotenv::dotenv().ok();
    let username = env::var("CLIENT_KEY").unwrap();
    let password = env::var("CLIENT_SECRET").unwrap();

    let client = Mpesa::new(username, password, Environment::Sandbox);

    let token = client.auth().unwrap();

    println!("token ==> {:?}", token);

    // let res = client.post("http://httpbin.org/post")
    //     .body("the exact body that is sent")
    //     .send()?;

    // println!("{:#?}", res);
    Ok(())
}
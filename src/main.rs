#![allow(unused)]
extern crate reqwest;

use dotenv;
use std::collections::HashMap;
use std::env;


use reqwest::blocking::{Client, self};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // safaricom credentials
    dotenv::dotenv().ok();
    let username = env::var("CLIENT_KEY").unwrap();
    let password = env::var("CLIENT_SECRET").unwrap();

    let client = Client::new();
    let resp: HashMap<String,String> = client.get("https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials")
        .basic_auth(username, Some(password))
        .send()?
        .json()?;
    println!("{:#?}", resp);

    let t =resp.get("access_token").unwrap();

    println!("token ==> {:?}", t);

    let res = client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()?;

    println!("{:#?}", res);
    Ok(())
}
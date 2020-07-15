#![allow(unused)]
use dotenv;
use std::collections::HashMap;
use std::env;

use mpesa::{Mpesa, Environment};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(), 
        env::var("CLIENT_SECRET").unwrap(), 
        Environment::Sandbox
    );

    let token = client.auth().unwrap();

    println!("token ==> {:?}", token);

    // let res = client.post("http://httpbin.org/post")
    //     .body("the exact body that is sent")
    //     .send()?;

    // println!("{:#?}", res);
    Ok(())
}
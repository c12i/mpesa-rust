#![allow(unused)]
extern crate reqwest;
use std::collections::HashMap;

use reqwest::blocking::{Client, self};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = blocking::get("https://httpbin.org/ip")?
    //     .json::<HashMap<String, String>>()?;
    // println!("{:#?}", resp);
    // Ok(())

    let client = Client::new();
    let res = client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .unwrap();

    println!("{:#?}", res);
    Ok(())
}
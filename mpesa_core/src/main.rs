#![allow(unused)]
#![allow(deprecated)]

use dotenv;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::env;

use chrono::prelude::*;
use mpesa::{CommandId, Environment, IdentifierTypes, Mpesa, MpesaError};

fn main() {
    // auth_test().unwrap();
    b2c_test();
    b2b_test();
    c2b_register_test();
    c2b_simulate_test();
    account_balance_test();
    stk_push_test();
}

fn b2c_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox, // or environment variable
    );

    let x = client
        .b2c("testapi496")
        .parties("600496", "254708374149")
        .urls("https://testdomain.com/err", "https://testdomain.com/res")
        .amount(1000)
        .send()
        .unwrap();

    println!("B2C response -> {:#?}", x);
}

fn b2b_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox, // or environment variable
    );

    let b2b_response = client
        .b2b("testapi496")
        .parties("600496", "600000")
        .urls("https://testdomain.com/err", "https://testdomain.com/api")
        .account_ref("254708374149")
        .amount(1000)
        .send()
        .unwrap();

    println!("B2b response -> {:#?}", b2b_response);
}

fn c2b_register_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox, // or environment variable
    );

    let c2b_register_response = client
        .c2b_register()
        .short_code("600496")
        .confirmation_url("https://testdomain.com/true")
        .validation_url("https://testdomain.com/valid")
        .send()
        .unwrap();

    println!("C2b register ==> {:#?}", c2b_register_response);
}

fn c2b_simulate_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox, // or environment variable
    );

    let c2b_simulate_response = client
        .c2b_simulate()
        .short_code("600496")
        .msisdn("254708374149")
        .amount(1000)
        .send()
        .unwrap();

    println!("C2b simulate ==> {:#?}", c2b_simulate_response);
}

fn account_balance_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox, // or environment variable
    );

    let account_balance_response = client
        .account_balance("testapi496")
        .urls("https://testdomain.com/err", "https://testdomain.com/ok")
        .party_a("600496")
        .send()
        .unwrap();

    println!("Account balance => {:#?}", account_balance_response);
}

fn stk_push_test() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env::var("CLIENT_KEY").unwrap(),
        env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox, // or environment variable
    );

    let res = client
        .express_request("174379")
        .phone_number("254705583540")
        .party_a("254705583540")
        .party_b("174379")
        .amount(10)
        .callback_url("https://test.example.com/api")
        .send()
        .unwrap();

    println!("STK Push response => {:#?}", res);
}

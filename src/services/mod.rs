//!# MPESA Services
//! Using the builder pattern in this to procedurally build Mpesa service specific payloads which are
//! ultimately consumed and the request sent by calling the `send` method.
//! Some of the builder methods for certain services are optional with default values standing in
//! their place when the builder gets consumed
//!
//! Here are the currently available services:
//! 1. [Account Balance](https://developer.safaricom.co.ke/docs#account-balance-api)
//! 2. [B2B](https://developer.safaricom.co.ke/docs#b2b-api)
//! 3. [B2C](https://developer.safaricom.co.ke/docs?shell#b2c-api)
//! 4. [C2B Register](https://developer.safaricom.co.ke/docs?shell#c2b-api)
//! 5. [C2B Simulate](https://developer.safaricom.co.ke/docs#account-balance-api)
//!
//! Also worth noting I am using `reqwest::blocking::Client` to make http requests. Research ongoing
//! on how to make this crate async.

#![allow(non_snake_case)]
mod account_balance;
mod b2b;
mod b2c;
mod c2b_register;
mod c2b_simulate;

pub use account_balance::AccountBalanceBuilder;
pub use b2b::B2bBuilder;
pub use b2c::B2cBuilder;
pub use c2b_register::C2bRegisterBuilder;
pub use c2b_simulate::C2bSimulateBuilder;

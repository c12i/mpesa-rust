//!## About
//!
//! An unofficial Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.
//!
//!## Install
//! `Cargo.toml`
//!
//! ```md
//! [dependencies]
//! mpesa = "0.3.5"
//! ```
//!
//! In your lib or binary crate:
//! ```rs
//! use mpesa::Mpesa;
//! ```
//!
//!## Usage
//!
//!### Creating a `Mpesa` client
//! You will first need to create an instance of the `Mpesa` instance (the client). You are required to provide a **CLIENT_KEY** and
//! **CLIENT_SECRET**. [Here](https://developer.safaricom.co.ke/test_credentials) is how you can get these credentials for the Safaricom sandbox
//! environment. It's worth noting that these credentials are only valid in the sandbox environment. To go live and get production keys
//! read the docs [here](https://developer.safaricom.co.ke/docs?javascript#going-live).
//!
//! These are the following ways you can instantiate `Mpesa`:
//!
//! _NOTE_:
//! * Only calling `unwrap` for demonstration purposes. Errors are handled appropriately in the lib via the `MpesaError` enum.
//! * Use of `dotenv` is optional.
//!
//! ```rust
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//! use dotenv::dotenv;
//!
//! dotenv().ok();
//!
//! let client: Mpesa = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     Environment::Sandbox,
//! );
//! assert!(client.is_connected())
//! ```
//!
//! Since the `Environment` enum implements `FromStr` and `TryFrom`, you can pass the name of the environment as a `&str` and call the `parse()` or `try_into()`
//! method to create an `Environment` type from the string slice (Pascal case or Uppercase string slices also valid):
//!
//! ```rust
//! use mpesa::Mpesa;
//! use std::env;
//! use dotenv::dotenv;
//!
//! dotenv().ok();
//!
//! let client: Mpesa = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(), // "production"
//! );
//! assert!(client.is_connected())
//! ```
//! If you intend to use in production, you will need to call a the `set_initiator_password` method from `Mpesa` after initially
//! creating the client. Here you provide your initiator password, which overrides the default password used in sandbox `"Safcom496!"`:
//!
//! ```rust
//! use mpesa::Mpesa;
//! use std::env;
//! use dotenv::dotenv;
//!
//! dotenv().ok();
//!
//! let client: Mpesa = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! client.set_initiator_password("new_password");
//!
//! assert!(client.is_connected())
//! ```
//!
//!### Services
//! The following services are currently available from the `Mpesa` client as methods that return builders:
//! * B2C
//! ```rust
//! use mpesa::{Mpesa, MpesaResult, B2cResponse};
//! use std::env;
//! use dotenv::dotenv;
//!
//! dotenv().ok();
//!
//! let client: Mpesa = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response: MpesaResult<B2cResponse> = client
//!     .b2c("testapi496")
//!     .party_a("600496")
//!     .party_b("254708374149")
//!     .result_url("https://testdomain.com/ok")
//!     .timeout_url("https://testdomain.com/err")
//!     .amount(1000)
//!     .send();
//! assert!(response.is_ok())
//! ```
//!
//! * B2B
//! ```rust
//! use mpesa::{Mpesa, MpesaResult, B2bResponse};
//! use std::env;
//! use dotenv::dotenv;
//!
//! dotenv().ok();
//!
//! let client: Mpesa = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response: MpesaResult<B2bResponse> = client
//!     .b2b("testapi496")
//!     .party_a("600496")
//!     .party_b("600000")
//!     .result_url("https://testdomain.com/ok")
//!     .timeout_url("https://testdomain.com/err")
//!     .account_ref("254708374149")
//!     .amount(1000)
//!     .send();
//! assert!(response.is_ok())
//! ```
//!
//! * C2B Register
//! ```rust
//! use mpesa::{Mpesa, MpesaResult, C2bRegisterResponse};
//! use serde_json::Value;
//! use std::env;
//! use dotenv::dotenv;
//!
//! dotenv().ok();
//!
//! let client: Mpesa = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response: MpesaResult<C2bRegisterResponse> = client
//!     .c2b_register()
//!     .short_code("600496")
//!     .confirmation_url("https://testdomain.com/true")
//!     .validation_url("https://testdomain.com/valid")
//!     .send();
//! assert!(response.is_ok())
//! ```
//!
//! * C2B Simulate
//! ```rust
//! use mpesa::{Mpesa, MpesaResult, C2bSimulateResponse};
//! use std::env;
//! use dotenv::dotenv;
//!
//! dotenv().ok();
//!
//! let client: Mpesa = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response: MpesaResult<C2bSimulateResponse> = client
//!     .c2b_simulate()
//!     .short_code("600496")
//!     .msisdn("254700000000")
//!     .amount(1000)
//!     .send();
//! assert!(response.is_ok())
//! ```
//!
//! * Account Balance
//!
//! ```rust
//! use mpesa::{Mpesa, MpesaResult, AccountBalanceResponse};
//! use std::env;
//! use dotenv::dotenv;
//!
//! dotenv().ok();
//!
//! let client: Mpesa = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response: MpesaResult<AccountBalanceResponse> = client
//!     .account_balance("testapi496")
//!     .result_url("https://testdomain.com/ok")
//!     .timeout_url("https://testdomain.com/err")
//!     .party_a("600496")
//!     .send();
//! assert!(response.is_ok())
//! ```
//!
//! * Mpesa Express Request / STK push/ Lipa na M-PESA online
//!
//! ```rust
//! use mpesa::{Mpesa, MpesaResult, MpesaExpressRequestResponse};
//! use std::env;
//! use dotenv::dotenv;
//!
//! dotenv().ok();
//!
//! let client: Mpesa = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response: MpesaResult<MpesaExpressRequestResponse> = client
//!     .express_request("174379")
//!     .phone_number("254708374149")
//!     .party_a("254708374149")
//!     .party_b("174379")
//!     .amount(500)
//!     .callback_url("https://testdomain.com/ok")
//!     .send();
//!
//! assert!(response.is_ok())
//! ```
//! More will be added progressively, pull requests welcome
//!## Author
//!
//! **Collins Muriuki**
//!
//! * Twitter: [@collinsmuriuki_](https://twitter.com/collinsmuriuki_)
//! * Not affiliated with Safaricom.
//!
//!## License
//! This project is MIT licensed

mod client;
mod constants;
pub mod environment;
mod errors;
mod mpesa_security;
pub mod services;

pub use client::{Mpesa, MpesaResult};
pub use constants::{CommandId, IdentifierTypes, MpesaResponseCode, ResponseType};
pub use environment::Environment;
pub use errors::MpesaError;
pub use mpesa_security::MpesaSecurity;
pub use services::{
    AccountBalanceResponse, B2bResponse, B2cResponse, C2bRegisterResponse, C2bSimulateResponse,
    MpesaExpressRequestResponse,
};

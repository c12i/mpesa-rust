//! ## About
//!
//! A Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.
//!
//! ## Disclaimer
//! **Warning!** WIP, not production ready
//!
//! ## Install & Usage
//! `Cargo.toml`
//!
//! ```md
//! [dependencies]
//! mpesa = "0.2.0"
//! ```
//!
//! In your lib or binary crate:
//! ```rs
//! use mpesa::Mpesa;
//! ```
//!
//! ## Usage
//!
//! ### Creating a `Client`
//! You will first need to create an instance of the `Mpesa` instance (the client). You are required to provide a **CLIENT_KEY**,
//! **CLIENT_SECRET** and **INIT_PASSWORD** (initiator password). [Here](https://developer.safaricom.co.ke/test_credentials) is how you can get these credentials for the Safaricom sandbox
//! environment.
//!
//! There are two ways you can instantiate `Mpesa`:
//! NOTE: only calling `unwrap` for demonstration purposes. Errors are handled appropriately in the lib via the `MpesaError` enum.
//!
//! ```rs
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//!
//! let client = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     Environment::Sandbox,
//! );
//! assert!(client.is_connected().unwrap())
//! ```
//!
//! Since the `Environment` enum implements `FromStr`, you can pass the name of the environment as a `&str` and call the `parse()`
//! method to create an `Environment` type from the string slice:
//!
//! ```rs
//! use mpesa::Mpesa;
//! use std::env;
//!
//! let client = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//! assert!(client.is_connected().unwrap())
//! ```
//!
//! ### Services
//! The following services are currently available from the `Mpesa` client as methods that return builders:
//! * B2C
//! ```rs
//! use mpesa::Mpesa;
//! use std::env;
//!
//! let client = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response = client
//!     .b2c("testapi496")
//!     .parties("600496", "254708374149")
//!     .urls("https://testdomain.com/err", "https://testdomain.com/res")
//!     .amount(1000)
//!     .send();
//! assert!(response.is_ok())
//! ```
//!
//! * B2B
//! ```rs
//! use mpesa::Mpesa;
//! use std::env;
//!
//! let client = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response = client
//!     .b2b("testapi496")
//!     .parties("600496", "600000")
//!     .urls("https://testdomain.com/err", "https://testdomain.com/api")
//!     .account_ref("254708374149")
//!     .amount(1000)
//!     .send();
//! assert!(response.is_ok())
//! ```
//!
//! * C2B Register
//! ```rs
//! use mpesa::Mpesa;
//! use std::env;
//!
//! let client = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response = client
//!     .c2b_register()
//!     .short_code("600496")
//!     .confirmation_url("https://testdomain.com/true")
//!     .validation_url("https://testdomain.com/valid")
//!     .send();
//! assert!(response.is_ok())
//! ```
//!
//! * C2B Simulate
//! ```rs
//! use mpesa::Mpesa;
//! use std::env;
//!
//! let client = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response = client
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
//! ```rs
//! use mpesa::Mpesa;
//! use std::env;
//!
//! let client = Mpesa::new(
//!     env::var("CLIENT_KEY").unwrap(),
//!     env::var("CLIENT_SECRET").unwrap(),
//!     "sandbox".parse().unwrap(),
//! );
//!
//! let response = client
//!     .account_balance("testapi496")
//!     .urls("https://testdomain.com/err", "https://testdomain.com/ok")
//!     .party_a("600496")
//!     .send();
//! assert!(response.is_ok())
//! ```
//!
//! More will be added progressively, pull requests welcome
//! ## Author
//!
//! **Collins Muriuki**
//!
//! * Twitter: [@collinsmuriuki\_](https://twitter.com/collinsmuriuki_)
//! * Not affiliated with Safaricom in any way.
//!
//! ## License
//! This project is MIT licensed

mod client;
mod constants;
mod environment;
mod errors;
mod mpesa_security;
pub mod services;

pub use client::Mpesa;
pub use constants::{CommandId, IdentifierTypes, ResponseType};
pub use environment::Environment;
pub use errors::MpesaError;
pub use mpesa_security::MpesaSecurity;

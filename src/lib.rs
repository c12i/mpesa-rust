//!## mpesa-rust
//!
//! An unofficial Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.
//!
//!## Install
//! `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! mpesa = "0.4.2"
//! ```
//! Optionally, you can disable default-features, which is basically the entire suite of MPESA APIs to conditionally select from either:
//! - `b2b`
//! - `b2c`
//! - `account_balance`
//! - `c2b_register`
//! - `c2b_simulate`
//! - `express_request`
//! - `transaction_reversal`
//!
//! Example:
//!
//! ```toml
//! [dependencies]
//! mpesa = { version = "0.4.2", default_features = false, features = ["b2b", "express_request"] }
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
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox,
//!     );
//!     assert!(client.is_connected().await)
//! }
//! ```
//!
//! Since the `Environment` enum implements `FromStr` and `TryFrom` for `String` and `&str` types, you can call `Environment::from_str` or `Environment::try_from` to create an `Environment` type. This is ideal if the environment values are
//! stored in a `.env` or any other configuration file
//!
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//! use dotenv::dotenv;
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::from_str("sandbox").unwrap()
//!     );
//!     assert!(client.is_connected().await)
//! }
//! ```
//! If you intend to use in production, you will need to call a the `set_initiator_password` method from `Mpesa` after initially
//! creating the client. Here you provide your initiator password, which overrides the default password used in sandbox `"Safcom496!"`:
//!
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     client.set_initiator_password("new_password");
//!
//!     assert!(client.is_connected().await)
//! }
//! ```
//!
//!### Services
//! The following services are currently available from the `Mpesa` client as methods that return builders:
//! * B2C
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response  = client
//!         .b2c("testapi496")
//!         .party_a("600496")
//!         .party_b("254708374149")
//!         .result_url("https://testdomain.com/ok")
//!         .timeout_url("https://testdomain.com/err")
//!         .amount(1000)
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * B2B
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .b2b("testapi496")
//!         .party_a("600496")
//!         .party_b("600000")
//!         .result_url("https://testdomain.com/ok")
//!         .timeout_url("https://testdomain.com/err")
//!         .account_ref("254708374149")
//!         .amount(1000)
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * Bill Manager Onboard
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment, SendRemindersTypes};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .bill_manager_onboard()
//!         .callback_url("https://testdomain.com/true")
//!         .email("email@test.com")
//!         .logo("https://file.domain/file.png")
//!         .official_contact("0712345678")
//!         .send_reminders(SendRemindersTypes::Enable)
//!         .short_code("600496")
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * Bill Manager Onboard Modify
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment, SendRemindersTypes};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .bill_manager_onboard_modify()
//!         .callback_url("https://testdomain.com/true")
//!         .email("email@test.com")
//!         .short_code("600496")
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * Bill Manager Bulk Invoice
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment, Invoice, InvoiceItem};
//! use std::env;
//! use chrono::prelude::Utc;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .bill_manager_bulk_invoice()
//!         .add_invoice(
//!             Invoice {
//!                 amount: 1000.0,
//!                 account_reference: "John Doe",
//!                 billed_full_name: "John Doe",
//!                 billed_period: "August 2021",
//!                 billed_phone_number: "0712345678",
//!                 due_date: Utc::now(),
//!                 external_reference: "INV2345",
//!                 invoice_items: Some(
//!                     vec![InvoiceItem {amount: 1000.0, item_name: "An item"}]
//!                 ),
//!                 invoice_name: "Invoice 001"
//!             }
//!         )
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * Bill Manager Single Invoice
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment, InvoiceItem};
//! use std::env;
//! use chrono::prelude::Utc;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .bill_manager_single_invoice()
//!         .amount(1000.0)
//!         .account_reference("John Doe")
//!         .billed_full_name("John Doe")
//!         .billed_period("August 2021")
//!         .billed_phone_number("0712345678")
//!         .due_date(Utc::now())
//!         .external_reference("INV2345")
//!         .invoice_items(vec![
//!             InvoiceItem {amount: 1000.0, item_name: "An item"}
//!         ])
//!         .invoice_name("Invoice 001")
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * Bill Manager Reconciliation
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment, InvoiceItem};
//! use std::env;
//! use chrono::prelude::Utc;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .bill_manager_reconciliation()
//!         .account_reference("John Doe")
//!         .external_reference("INV2345")
//!         .full_name("John Doe")
//!         .invoice_name("Invoice 001")
//!         .paid_amount(1000.0)
//!         .payment_date(Utc::now())
//!         .phone_number("0712345678")
//!         .transaction_id("TRANSACTION_ID")
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * C2B Register
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment};
//! use serde_json::Value;
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .c2b_register()
//!         .short_code("600496")
//!         .confirmation_url("https://testdomain.com/true")
//!         .validation_url("https://testdomain.com/valid")
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * C2B Simulate
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .c2b_simulate()
//!         .short_code("600496")
//!         .msisdn("254700000000")
//!         .amount(1000)
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * Account Balance
//!
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .account_balance("testapi496")
//!         .result_url("https://testdomain.com/ok")
//!         .timeout_url("https://testdomain.com/err")
//!         .party_a("600496")
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * Mpesa Express Request / STK push/ Lipa na M-PESA online
//!
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .express_request("174379")
//!         .phone_number("254708374149")
//!         .amount(500)
//!         .callback_url("https://testdomain.com/ok")
//!         .send()
//!         .await;
//!     assert!(response.is_ok())
//! }
//! ```
//!
//! * Transaction Reversal
//!
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment, IdentifierTypes};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .transaction_reversal("testapi496")
//!         .result_url("https://testdomain.com/ok")
//!         .timeout_url("https://testdomain.com/err")
//!         .transaction_id("OEI2AK4Q16")
//!         .receiver_identifier_type(IdentifierTypes::ShortCode)
//!         .amount(100.0)
//!         .receiver_party("600111")
//!         .send()
//!         .await;
//!     assert!(response.is_ok());
//!}
//! ```
//!
//! * Transaction Status
//!
//! ```rust,no_run
//! use mpesa::{Mpesa, Environment};
//! use std::env;
//! use dotenv::dotenv;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv().ok();
//!
//!     let client = Mpesa::new(
//!         env::var("CLIENT_KEY").unwrap(),
//!         env::var("CLIENT_SECRET").unwrap(),
//!         Environment::Sandbox
//!     );
//!
//!     let response = client
//!         .transaction_status("testapi496")
//!         .result_url("https://testdomain.com/ok")
//!         .timeout_url("https://testdomain.com/err")
//!         .transaction_id("OEI2AK4Q16")
//!         .party_a("600111")
//!         .send()
//!         .await;
//!     assert!(response.is_ok());
//! }
//! ```
//!
//! More will be added progressively, pull requests welcome
//!
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
pub mod services;

pub use client::{Mpesa, MpesaResult};
pub use constants::{
    CommandId, IdentifierTypes, Invoice, InvoiceItem, ResponseType, SendRemindersTypes,
};
pub use environment::ApiEnvironment;
pub use environment::Environment::{self, Production, Sandbox};
pub use errors::MpesaError;

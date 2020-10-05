//! ## About
//!
//! A Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.
// Currently, a work in progress project.
//
//! ## Notes
//!  **Warning!** v0.*. Expect bugs therefore not recommended for use in production. Pull requests and issues very welcome.
//!
//! ## Install & Usage
//! In your `Cargo.toml` file:
//!
//! ```md
//! [dependencies]
//! mpesa = "0.1.5"
//! ```
//!
//! In your lib or binary crate:
//! ```rs
//! use mpesa::Mpesa;
//! ```
//!
//! ## Examples
//!
//! Use [`dotenv`](https://docs.rs/dotenv/0.15.0/dotenv/fn.dotenv.html) crate to store your keys as environmental variables instead of hard coding them like done in the example below.
//!
//! ```rs
//! use mpesa::{Mpesa, Environment};
//!
//! let client = Mpesa::new(
//!       String::from("your_client_key"),
//!       String::from("your_client_secret"),
//!       Environment::Sandbox,
//!       String::from("your_initiator_password"),
//! );
//! ```
//!
//! ## Author
//!
//! **Collins Muriuki**
//!
//! ## License
//! This project is MIT licensed

mod client;
pub mod environment;
pub mod payloads;
pub mod constants;
pub mod mpesa_security;

pub use client::Mpesa;
pub use environment::Environment;
pub use constants::{CommandId,IdentifierTypes};
pub use payloads::ResponseType;
pub use mpesa_security::MpesaSecurity;

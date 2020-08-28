#![allow(dead_code)]
#![allow(unused_imports)]
//! # mpesa
//! A work in progress implementation
//! ## About
//! A Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.
//!
//! This will allow you to make hassle free requests to the daraja API to perform B2B, B2C, and C2B transactions.
//!
//! Currently work in progress, documentation will be added every step of the way.

mod utils;
mod client;
pub mod environment;
pub mod payloads;
pub mod constants;

pub use client::Mpesa;
pub use environment::Environment;
pub use constants::{CommandId,IdentifierTypes};
pub use payloads::ResponseType;

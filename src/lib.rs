//! # mpesa
//! WIP
//! ## About
//! A Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.
//!
//! This will allow you to make hassle free requests to the daraja API to perform B2B, B2C, and C2B transactions.
//!
//! Currently work in progress, documentation will be added every step of the way.

pub mod utils;
mod client;
mod environment;

pub use client::Mpesa;
pub use environment::Environment;

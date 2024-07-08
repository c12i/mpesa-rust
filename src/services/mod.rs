//!# MPESA Services
//!
//! Using the builder pattern in this to procedurally build Mpesa service specific payloads which are
//! ultimately consumed and the request sent by calling the `send` method.
//! Some of the builder methods for certain services are optional with default values standing in
//! their place when the builder gets consumed
//!
//! Here are the currently supported services:
//! 1. [Account Balance](https://developer.safaricom.co.ke/APIs/AccountBalance)
//! 2. [B2B](https://developer.safaricom.co.ke/APIs/BusinessPayBill)
//! 3. [B2C](https://developer.safaricom.co.ke/APIs/BusinessToCustomer)
//! 4. [C2B Register](https://developer.safaricom.co.ke/APIs/CustomerToBusinessRegisterURL)
//! 5. [C2B Simulate](https://developer.safaricom.co.ke/c2b/apis/post/simulate)
//! 6. [Mpesa Express/ STK Push](https://developer.safaricom.co.ke/APIs/MpesaExpressSimulate)
//! 7. [Transaction Reversal](https://developer.safaricom.co.ke/Documentation)
//! 8. [Bill Manager](https://developer.safaricom.co.ke/APIs/BillManager)
//! 9. [Transaction Status](https://developer.safaricom.co.ke/APIs/TransactionStatus)
//! 10. [Dynamic QR](https://developer.safaricom.co.ke/APIs/DynamicQRCode)

mod account_balance;
mod b2b;
mod b2c;
mod bill_manager;
mod c2b_register;
mod c2b_simulate;
mod dynamic_qr;
mod express;
mod transaction_reversal;
mod transaction_status;

#[cfg(feature = "account_balance")]
pub use account_balance::{AccountBalanceBuilder, AccountBalanceResponse};
#[cfg(feature = "b2b")]
pub use b2b::*;
#[cfg(feature = "b2c")]
pub use b2c::*;
#[cfg(feature = "bill_manager")]
pub use bill_manager::*;
#[cfg(feature = "c2b_register")]
pub use c2b_register::{C2bRegisterBuilder, C2bRegisterResponse};
#[cfg(feature = "c2b_simulate")]
pub use c2b_simulate::{C2bSimulateBuilder, C2bSimulateResponse};
#[cfg(feature = "dynamic_qr")]
pub use dynamic_qr::{DynamicQR, DynamicQRBuilder, DynamicQRRequest, DynamicQRResponse};
#[cfg(feature = "express")]
pub use express::*;
#[cfg(feature = "transaction_reversal")]
pub use transaction_reversal::*;
#[cfg(feature = "transaction_status")]
pub use transaction_status::*;

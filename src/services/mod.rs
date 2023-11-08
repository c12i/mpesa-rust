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
//! 6. [Mpesa Express/ STK Push](https://developer.safaricom.co.ke/docs#lipa-na-m-pesa-online-payment)
//! 7. [Transaction Reversal](https://developer.safaricom.co.ke/docs#reversal)
//! 8. [Bill Manager](https://developer.safaricom.co.ke/APIs/BillManager)

#[cfg(feature = "account_balance")]
mod account_balance;
#[cfg(feature = "b2b")]
mod b2b;
#[cfg(feature = "b2c")]
mod b2c;
#[cfg(feature = "bill_manager")]
mod bill_manager;
#[cfg(feature = "c2b_register")]
mod c2b_register;
#[cfg(feature = "c2b_simulate")]
mod c2b_simulate;
#[cfg(feature = "express_request")]
mod express_request;
#[cfg(feature = "transaction_reversal")]
mod transaction_reversal;
#[cfg(feature = "transaction_status")]
mod transaction_status;

#[cfg(feature = "account_balance")]
pub use account_balance::{AccountBalanceBuilder, AccountBalanceResponse};
#[cfg(feature = "b2b")]
pub use b2b::{B2bBuilder, B2bResponse};
#[cfg(feature = "b2c")]
pub use b2c::{B2cBuilder, B2cResponse};
#[cfg(feature = "bill_manager")]
pub use bill_manager::*;
#[cfg(feature = "c2b_register")]
pub use c2b_register::{C2bRegisterBuilder, C2bRegisterResponse};
#[cfg(feature = "c2b_simulate")]
pub use c2b_simulate::{C2bSimulateBuilder, C2bSimulateResponse};
#[cfg(feature = "express_request")]
pub use express_request::{MpesaExpressRequestBuilder, MpesaExpressRequestResponse};
#[cfg(feature = "transaction_reversal")]
pub use transaction_reversal::{TransactionReversalBuilder, TransactionReversalResponse};
#[cfg(feature = "transaction_status")]
pub use transaction_status::{TransactionStatusBuilder, TransactionStatusResponse};

use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::MpesaError;

/// Mpesa command ids
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommandId {
    TransactionReversal,
    SalaryPayment,
    BusinessPayment,
    PromotionPayment,
    AccountBalance,
    CustomerPayBillOnline,
    TransactionStatusQuery,
    CheckIdentity,
    BusinessPayBill,
    BusinessBuyGoods,
    DisburseFundsToBusiness,
    BusinessToBusinessTransfer,
    BusinessTransferFromMMFToUtility,
}

impl Display for CommandId {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{self:?}")
    }
}

/// Identifier types - both sender and receiver - identify an M-Pesa transaction’s sending and receiving party as
/// either a shortcode, a till number or a MSISDN (phone number).
/// There are three identifier types that can be used with M-Pesa APIs.
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone)]
#[repr(u16)]
pub enum IdentifierTypes {
    MSISDN = 1,
    TillNumber = 2,
    ShortCode = 4,
    Reversal = 11,
}

impl Display for IdentifierTypes {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", *self as u16)
    }
}

/// TODO: Enable deserializing of json numbers/ strings to `MpesaResponseCode`
/// M-pesa result and response codes
#[derive(Debug, Copy, Clone, Deserialize_repr)]
#[repr(u16)]
#[allow(unused)]
pub enum MpesaResponseCode {
    Success = 0,
    InsufficientFunds = 1,
    LessThanMinimum = 2,
    MoreThanMaximum = 3,
    ExceededDailyLimit = 4,
    ExceededMinimumBalance = 5,
    UnresolvedPrimaryParty = 6,
    UnresolvedReceiverParty = 7,
    ExceededMaximumBalance = 8,
    InvalidDebitAccount = 11,
    InvalidCreditAccount = 12,
    UnresolvedDebitAccount = 13,
    UnresolvedCreditAccount = 14,
    DuplicateDetected = 15,
    InternalFailure = 17,
    UnresolvedInitiator = 20,
    TrafficBlocking = 26,
}

impl Display for MpesaResponseCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", *self as u16)
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// C2B Register Response types
pub enum ResponseType {
    Completed,
    Cancelled,
}

impl Display for ResponseType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Copy, Clone)]
#[repr(u16)]
pub enum SendRemindersTypes {
    Disable = 0,
    Enable = 1,
}

impl Display for SendRemindersTypes {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", *self as u16)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum TransactionType {
    /// Send Money(Mobile number).
    SendMoney,
    /// Withdraw Cash at Agent Till
    Withdraw,
    /// Pay Merchant (Buy Goods)
    BG,
    /// Paybill or Business number
    PayBill,
    /// Sent to Business. Business number CPI in MSISDN format.
    SendBusiness,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{self:?}")
    }
}

impl TryFrom<&str> for TransactionType {
    type Error = MpesaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "bg" => Ok(TransactionType::BG),
            "wa" => Ok(TransactionType::Withdraw),
            "pb" => Ok(TransactionType::PayBill),
            "sm" => Ok(TransactionType::SendMoney),
            "sb" => Ok(TransactionType::SendBusiness),
            _ => Err(MpesaError::Message("Invalid transaction type")),
        }
    }
}

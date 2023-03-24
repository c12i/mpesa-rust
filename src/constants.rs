use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Mpesa command ids
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize)]
pub struct InvoiceItem<'invoice> {
    pub amount: f64,
    pub item_name: &'invoice str,
}

impl<'invoice> Display for InvoiceItem<'invoice> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{self:?}")
    }
}

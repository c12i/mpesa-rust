use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::Serialize;

/// Mpesa command ids
#[derive(Debug, Serialize)]
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
        write!(f, "{:?}", self)
    }
}

/// Identifier types - both sender and receiver - identify an M-Pesa transaction’s sending and receiving party as
/// either a shortcode, a till number or a MSISDN (phone number).
/// There are three identifier types that can be used with M-Pesa APIs.
#[derive(Debug)]
pub enum IdentifierTypes {
    MSISDN = 1,
    TillNumber = 2,
    Shortcode = 4,
}

impl IdentifierTypes {
    pub fn get_code(&self) -> &str {
        match self {
            IdentifierTypes::MSISDN => "1",
            IdentifierTypes::TillNumber => "2",
            IdentifierTypes::Shortcode => "4",
        }
    }
}

impl Display for IdentifierTypes {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self.get_code())
    }
}

/// M-pesa result and response codes
#[derive(Debug)]
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
        write!(f, "{:?}", self)
    }
}

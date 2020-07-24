/// Mpesa command ids
#[derive(Debug)]
pub enum CommandIds {
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

impl CommandIds {
    /// Data to be sent alongside the payloads
    pub fn get_command_id_str(&self) -> &'static str {
        match self {
            CommandIds::TransactionReversal => "TransactionReversal",
            CommandIds::SalaryPayment => "SalaryPayment",
            CommandIds::BusinessPayment => "BusinessPayment",
            CommandIds::PromotionPayment => "PromotionPayment",
            CommandIds::AccountBalance => "AccountBalance",
            CommandIds::CustomerPayBillOnline => "CustomerPayBillOnline",
            CommandIds::TransactionStatusQuery => "TransactionStatusQuery",
            CommandIds::CheckIdentity => "CheckIdentity",
            CommandIds::BusinessPayBill => "BusinessPayBill",
            CommandIds::BusinessBuyGoods => "BusinessBuyGoods",
            CommandIds::DisburseFundsToBusiness => "DisburseFundsToBusiness",
            CommandIds::BusinessToBusinessTransfer => "BusinessToBusinessTransfer",
            CommandIds::BusinessTransferFromMMFToUtility => "BusinessTransferFromMMFToUtility",
        }
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

/// M-pesa result and response codes
pub enum MpesaResponse {
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
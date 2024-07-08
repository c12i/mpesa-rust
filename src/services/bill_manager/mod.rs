mod bulk_invoice;
mod cancel_invoice;
mod onboard;
mod onboard_modify;
mod reconciliation;
mod single_invoice;

use std::fmt::{Display, Formatter, Result as FmtResult};

pub use bulk_invoice::{BulkInvoiceBuilder, BulkInvoiceResponse};
pub use cancel_invoice::{CancelInvoiceBuilder, CancelInvoiceResponse};
use chrono::{DateTime, Utc};
pub use onboard::{OnboardBuilder, OnboardResponse};
pub use onboard_modify::{OnboardModifyBuilder, OnboardModifyResponse};
pub use reconciliation::{ReconciliationBuilder, ReconciliationResponse};
use serde::Serialize;
pub use single_invoice::{SingleInvoiceBuilder, SingleInvoiceResponse};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice<'i> {
    pub amount: f64,
    pub account_reference: &'i str,
    pub billed_full_name: &'i str,
    pub billed_period: &'i str,
    pub billed_phone_number: &'i str,
    pub due_date: DateTime<Utc>,
    pub external_reference: &'i str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_items: Option<Vec<InvoiceItem<'i>>>,
    pub invoice_name: &'i str,
}

impl<'i> Display for Invoice<'i> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "amount: {}, account_reference: {}, due_date: {}, invoice_name: {}",
            self.amount,
            self.account_reference,
            self.due_date.format("%Y-%m-%d"),
            self.invoice_name,
        )
    }
}

#[derive(Debug, Serialize)]
pub struct InvoiceItem<'i> {
    pub amount: f64,
    pub item_name: &'i str,
}

impl<'i> Display for InvoiceItem<'i> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "amount: {}, item_name: {}", self.amount, self.item_name)
    }
}

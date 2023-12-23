#![doc = include_str!("../../../docs/client/bill_manager/single_invoice.md")]

use chrono::prelude::{DateTime, Utc};
use derive_builder::Builder;
use serde::Deserialize;

use crate::client::Mpesa;
use crate::constants::{Invoice, InvoiceItem};
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

const BILL_MANAGER_SINGLE_INVOICE_API_URL: &str = "v1/billmanager-invoice/single-invoicing";

#[derive(Clone, Debug, Deserialize)]
pub struct SingleInvoiceResponse {
    #[serde(rename(deserialize = "rescode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "resmsg"))]
    pub response_message: String,
    #[serde(rename(deserialize = "Status_Message"))]
    pub status_message: String,
}

#[derive(Builder, Clone, Debug)]
#[builder(build_fn(error = "MpesaError"))]
pub struct SingleInvoice<'mpesa, Env: ApiEnvironment> {
    #[builder(pattern = "immutable", private)]
    client: &'mpesa Mpesa<Env>,

    /// Total Invoice amount to be paid in Kenyan Shillings.
    #[builder(setter(into))]
    amount: f64,

    /// The account number being invoiced that uniquely identifies a customer.
    #[builder(setter(into))]
    account_reference: &'mpesa str,

    /// The name of the recipient to receive the invoice details.
    #[builder(setter(into))]
    billed_full_name: &'mpesa str,

    /// Month and Year. Must be in the format `Month Year` e.g. "December 2023"
    #[builder(setter(into))]
    billed_period: &'mpesa str,

    /// The phone number to receive invoice details via sms.
    /// Must be in the format 07XXXXXXXX.
    #[builder(setter(into))]
    billed_phone_number: &'mpesa str,

    /// Due date for this invoice.
    #[builder(setter(into), try_setter)]
    due_date: DateTime<Utc>,

    /// Unique identifier to attach to this invoice.
    #[builder(setter(into))]
    external_reference: &'mpesa str,

    /// Additional billable items in your invoice.
    #[builder(default = "None", setter(into), try_setter)]
    invoice_items: Option<Vec<InvoiceItem<'mpesa>>>,

    /// Descriptive name for this invoice.
    #[builder(setter(into))]
    invoice_name: &'mpesa str,
}

impl<'mpesa, Env: ApiEnvironment> From<SingleInvoice<'mpesa, Env>> for Invoice<'mpesa> {
    fn from(builder: SingleInvoice<'mpesa, Env>) -> Self {
        Invoice {
            amount: builder.amount,
            account_reference: builder.account_reference,
            billed_full_name: builder.billed_full_name,
            billed_period: builder.billed_period,
            billed_phone_number: builder.billed_phone_number,
            due_date: builder.due_date,
            external_reference: builder.external_reference,
            invoice_items: builder.invoice_items,
            invoice_name: builder.invoice_name,
        }
    }
}

impl<'mpesa, Env: ApiEnvironment> SingleInvoice<'mpesa, Env> {
    pub(crate) fn builder(client: &'mpesa Mpesa<Env>) -> SingleInvoiceBuilder<'mpesa, Env> {
        SingleInvoiceBuilder::default().client(client)
    }

    /// Builds SingleInvoice
    ///
    /// Returns a `SingleInvoice` which can be used to send a request.
    pub fn from_request(client: &'mpesa Mpesa<Env>, request: Invoice<'mpesa>) -> Self {
        SingleInvoice {
            client,
            amount: request.amount,
            account_reference: request.account_reference,
            billed_full_name: request.billed_full_name,
            billed_period: request.billed_period,
            billed_phone_number: request.billed_phone_number,
            due_date: request.due_date,
            external_reference: request.external_reference,
            invoice_items: request.invoice_items,
            invoice_name: request.invoice_name,
        }
    }

    /// Bill Manager Single Invoice API
    ///
    /// Creates and sends invoices to your customers
    ///
    /// A successful request returns a `SingleInvoiceResponse` type
    ///
    /// # Errors
    /// Returns an `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<SingleInvoiceResponse> {
        self.client
            .send::<Invoice, _>(crate::client::Request {
                method: reqwest::Method::POST,
                path: BILL_MANAGER_SINGLE_INVOICE_API_URL,
                body: self.into(),
            })
            .await
    }
}

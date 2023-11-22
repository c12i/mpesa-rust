use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::constants::CommandId;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

const C2B_SIMULATE_URL: &str = "mpesa/c2b/v1/simulate";

#[derive(Debug, Serialize)]
/// Payload to make payment requests from C2B.
/// See more: https://developer.safaricom.co.ke/docs#c2b-api
struct C2bSimulatePayload<'mpesa> {
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "Amount"))]
    amount: f64,
    #[serde(rename(serialize = "Msisdn"))]
    msisdn: &'mpesa str,
    #[serde(rename(serialize = "BillRefNumber"))]
    bill_ref_number: &'mpesa str,
    #[serde(rename(serialize = "ShortCode"))]
    short_code: &'mpesa str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct C2bSimulateResponse {
    #[serde(
        rename(deserialize = "ConversationID"),
        skip_serializing_if = "Option::is_none"
    )]
    pub conversation_id: Option<String>,
    #[serde(rename(deserialize = "OriginatorConversationID"))]
    pub originator_conversation_id: String,
    #[serde(rename(deserialize = "ResponseCode"))]
    pub response_code: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    pub response_description: String,
}

#[derive(Debug)]
pub struct C2bSimulateBuilder<'mpesa, Env: ApiEnvironment> {
    client: &'mpesa Mpesa<Env>,
    command_id: Option<CommandId>,
    amount: Option<f64>,
    msisdn: Option<&'mpesa str>,
    bill_ref_number: Option<&'mpesa str>,
    short_code: Option<&'mpesa str>,
}

impl<'mpesa, Env: ApiEnvironment> C2bSimulateBuilder<'mpesa, Env> {
    /// Creates a new C2B Simulate builder
    pub fn new(client: &'mpesa Mpesa<Env>) -> C2bSimulateBuilder<'mpesa, Env> {
        C2bSimulateBuilder {
            client,
            command_id: None,
            amount: None,
            msisdn: None,
            bill_ref_number: None,
            short_code: None,
        }
    }

    /// Adds `CommandId`. Defaults to `CommandId::CustomerPaybillOnline` if no value explicitly passed
    ///
    /// # Errors
    /// If `CommandId` is not valid
    pub fn command_id(mut self, command_id: CommandId) -> C2bSimulateBuilder<'mpesa, Env> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds an `amount` to the request
    ///
    /// # Errors
    /// If `Amount` is not provided
    pub fn amount<Number: Into<f64>>(mut self, amount: Number) -> C2bSimulateBuilder<'mpesa, Env> {
        self.amount = Some(amount.into());
        self
    }

    /// Adds the MSISDN(phone number) sending the transaction, start by country code without the `+`.
    /// This is a required field
    ///
    /// # Errors
    /// If `MSISDN` is invalid or not provided
    pub fn msisdn(mut self, msisdn: &'mpesa str) -> C2bSimulateBuilder<'mpesa, Env> {
        self.msisdn = Some(msisdn);
        self
    }

    /// Adds `ShortCode`; the 6 digit MPESA Till Number or PayBill Number
    ///
    /// # Errors
    /// If Till or PayBill number is invalid or not provided
    pub fn short_code(mut self, short_code: &'mpesa str) -> C2bSimulateBuilder<'mpesa, Env> {
        self.short_code = Some(short_code);
        self
    }

    /// Adds Bill reference number.
    ///
    /// # Errors
    /// If `BillRefNumber` is invalid or not provided
    pub fn bill_ref_number(
        mut self,
        bill_ref_number: &'mpesa str,
    ) -> C2bSimulateBuilder<'mpesa, Env> {
        self.bill_ref_number = Some(bill_ref_number);
        self
    }

    /// # C2B Simulate API
    ///
    /// Make payment requests from Client to Business
    ///
    /// This enables you to receive the payment requests in real time.
    /// See more [here](https://developer.safaricom.co.ke/c2b/apis/post/simulate)
    ///
    /// A successful request returns a `C2bSimulateResponse` type
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<C2bSimulateResponse> {
        let payload = C2bSimulatePayload {
            command_id: self.command_id.unwrap_or(CommandId::CustomerPayBillOnline),
            amount: self
                .amount
                .ok_or(MpesaError::Message("amount is required"))?,
            msisdn: self
                .msisdn
                .ok_or(MpesaError::Message("msisdn is required"))?,
            bill_ref_number: self
                .bill_ref_number
                .ok_or(MpesaError::Message("bill_ref_number is required"))?,
            short_code: self
                .short_code
                .ok_or(MpesaError::Message("short_code is required"))?,
        };

        self.client
            .send(crate::client::Request {
                method: reqwest::Method::POST,
                path: C2B_SIMULATE_URL,
                body: payload,
            })
            .await
    }
}

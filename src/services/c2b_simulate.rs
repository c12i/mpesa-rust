use crate::client::{Mpesa, MpesaResult};
use crate::constants::CommandId;
use crate::environment::ApiEnvironment;
use crate::errors::MpesaError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
/// Payload to make payment requests from C2B.
/// See more: https://developer.safaricom.co.ke/docs#c2b-api
struct C2bSimulatePayload<'a> {
    #[serde(rename(serialize = "CommandID"))]
    command_id: CommandId,
    #[serde(rename(serialize = "Amount"))]
    amount: f64,
    #[serde(rename(serialize = "Msisdn"), skip_serializing_if = "Option::is_none")]
    msisdn: Option<&'a str>,
    #[serde(
        rename(serialize = "BillRefNumber"),
        skip_serializing_if = "Option::is_none"
    )]
    bill_ref_number: Option<&'a str>,
    #[serde(
        rename(serialize = "ShortCode"),
        skip_serializing_if = "Option::is_none"
    )]
    short_code: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct C2bSimulateResponse {
    #[serde(
        rename(deserialize = "ConversationID"),
        skip_serializing_if = "Option::is_none"
    )]
    pub conversation_id: Option<String>,
    #[serde(rename(deserialize = "OriginatorCoversationID"))]
    pub originator_coversation_id: String,
    #[serde(rename(deserialize = "ResponseDescription"))]
    pub response_description: String,
}

#[derive(Debug)]
pub struct C2bSimulateBuilder<'a, Env: ApiEnvironment> {
    client: &'a Mpesa<Env>,
    command_id: Option<CommandId>,
    amount: Option<f64>,
    msisdn: Option<&'a str>,
    bill_ref_number: Option<&'a str>,
    short_code: Option<&'a str>,
}

impl<'a, Env: ApiEnvironment> C2bSimulateBuilder<'a, Env> {
    /// Creates a new C2B Simulate builder
    pub fn new(client: &'a Mpesa<Env>) -> C2bSimulateBuilder<'a, Env> {
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
    pub fn command_id(mut self, command_id: CommandId) -> C2bSimulateBuilder<'a, Env> {
        self.command_id = Some(command_id);
        self
    }

    /// Adds an `amount` to the request
    /// This is a required field
    pub fn amount<Number: Into<f64>>(mut self, amount: Number) -> C2bSimulateBuilder<'a, Env> {
        self.amount = Some(amount.into());
        self
    }

    /// Adds the MSISDN(phone number) sending the transaction, start by country code without the `+`.
    /// This is a required field
    ///
    /// # Errors
    /// If `MSISDN` is invalid
    pub fn msisdn(mut self, msisdn: &'a str) -> C2bSimulateBuilder<'a, Env> {
        self.msisdn = Some(msisdn);
        self
    }

    /// Adds `ShortCode`; the 6 digit MPESA Till Number or PayBill Number
    ///
    /// # Errors
    /// If Till or PayBill number is invalid
    pub fn short_code(mut self, short_code: &'a str) -> C2bSimulateBuilder<'a, Env> {
        self.short_code = Some(short_code);
        self
    }

    /// Adds Bull reference number. This field is optional and will by default be `"None"`.
    pub fn bill_ref_number(mut self, bill_ref_number: &'a str) -> C2bSimulateBuilder<'a, Env> {
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
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub async fn send(self) -> MpesaResult<C2bSimulateResponse> {
        let url = format!(
            "{}/mpesa/c2b/v1/simulate",
            self.client.environment().base_url()
        );

        let payload = C2bSimulatePayload {
            command_id: self
                .command_id
                .unwrap_or_else(|| CommandId::CustomerPayBillOnline),
            amount: self.amount.unwrap_or_default(),
            msisdn: self.msisdn,
            bill_ref_number: self.bill_ref_number,
            short_code: self.short_code,
        };

        let response = self
            .client
            .http_client
            .post(&url)
            .bearer_auth(self.client.auth().await?)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let value = response.json::<_>().await?;
            return Ok(value);
        }

        let value = response.json().await?;
        Err(MpesaError::C2bSimulateError(value))
    }
}

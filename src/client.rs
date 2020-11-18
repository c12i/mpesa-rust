use reqwest::blocking::{Client, Response};
use serde_json::Value;

use super::environment::Environment;
use super::services::{B2bBuilder, C2bRegisterResponse, C2bSimulateResponse};

use crate::services::ResponseType;
use crate::services::{AccountBalancePayload, AccountBalanceResponse};
use crate::services::{B2bPayload, B2cBuilder, C2bRegisterPayload, C2bSimulatePayload};
use crate::MpesaError;
use crate::{CommandId, IdentifierTypes};

pub type MpesaResult<T> = Result<T, MpesaError>;

/// Mpesa client that will facilitate communication with the Safaricom API
#[derive(Debug)]
pub struct Mpesa {
    client_key: String,
    client_secret: String,
    environment: Environment,
    initiator_password: String,
}

impl<'a> Mpesa {
    /// Constructs a new `Mpesa` instance.
    pub fn new(
        client_key: String,
        client_secret: String,
        environment: Environment,
        initiator_password: String,
    ) -> Self {
        Self {
            client_key,
            client_secret,
            environment,
            initiator_password,
        }
    }

    /// Gets the current `Environment`
    pub fn environment(&'a self) -> &Environment {
        &self.environment
    }

    /// Gets the initiator password
    pub fn initiator_password(&'a self) -> &String {
        &self.initiator_password
    }

    /// # Safaricom Oauth
    ///
    /// Generates an access token
    /// Sends `GET` request to Safaricom oauth to acquire token for token authentication
    /// The OAuth access token expires after an hour, after which, you will need to generate another access token
    ///
    /// Returns the auth token as a `String`.
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub fn auth(&self) -> MpesaResult<String> {
        let url = format!(
            "{}/oauth/v1/generate?grant_type=client_credentials",
            self.environment.base_url()
        );
        let resp = Client::new()
            .get(&url)
            .basic_auth(&self.client_key, Some(&self.client_secret))
            .send()?;
        if resp.status().is_success() {
            let value: Value = resp.json()?;
            // "value" -> value
            return Ok(value["access_token"].to_string().replace("\"", ""));
        }
        Err(MpesaError::Message(
            "Could not authenticate to Safaricom, please check your credentials",
        ))
    }

    /// # B2C Builder
    /// Creates a `B2cBuilder` for building a B2C transaction struct.
    /// The builder is consumed and request made by calling its `send` method.
    /// Requires an `initiator_name`.
    ///
    /// # Example
    /// ```
    /// let res = client
    ///     .b2c("testapi496")
    ///     .parties("600496", "254708374149")
    ///     .urls("https://testdomain.com/err", "https://testdomain.com/res")
    ///     .amount(1000)
    ///     .send();
    /// ```
    pub fn b2c(&'a self, initiator_name: &'a str) -> B2cBuilder<'a> {
        B2cBuilder::new(&self, initiator_name)
    }

    /// # B2B Builder
    /// Creates a `B2bBuilder` for building B2B transaction struct.
    /// Requires an `initiator_name`
    /// ```
    /// let b2b_response = client.b2b("testapi496")
    ///     .parties("600496", "600000")
    ///     .urls("https://testdomain.com/err", "https://testdomain.com/api")
    ///     .account_ref("254708374149")
    ///     .amount(1000)
    ///     .send();
    /// ```
    pub fn b2b(&'a self, initiator_name: &'a str) -> B2bBuilder<'a> {
        B2bBuilder::new(&self, initiator_name)
    }

    // /// Registers the the 3rd party’s confirmation and validation URLs to M-Pesa
    // ///
    // /// Registering maps these URLs to the 3rd party shortcode.
    // /// Whenever M-Pesa receives a transaction on the shortcode,
    // /// M-Pesa triggers a validation request against the validation URL and
    // /// the 3rd party system responds to M-Pesa with a validation response (either a success or an error code).
    // /// The response expected is the success code the 3rd party
    // ///
    // /// # Example
    // /// ```
    // /// dotenv::dotenv().ok();
    // ///
    // /// let client = mpesa::Mpesa::new(
    // ///    std::env::var("CLIENT_KEY").unwrap(),
    // ///    std::env::var("CLIENT_SECRET").unwrap(),
    // ///    mpesa::Environment::Sandbox,
    // ///    std::env::var("INIT_PASSWORD").unwrap(),
    // /// );
    // ///
    // /// let c2b_register_response = client.c2b_register(
    // ///         "https://muriuki.dev/api",
    // ///         "https://muriuki.dev/verify",
    // ///         mpesa::ResponseType::Complete,
    // ///         "600496"
    // ///     ).unwrap();
    // /// ```
    // ///
    // /// # Errors
    // /// TODO
    // pub fn c2b_register(
    //     &self,
    //     validation_url: &str,
    //     confirmation_url: &str,
    //     response_type: ResponseType,
    //     short_code: &str,
    // ) -> Result<Response, Box<dyn std::error::Error>> {
    //     let url = format!("{}/mpesa/c2b/v1/registerurl", self.environment.base_url());
    //
    //     let payload = C2bRegisterPayload {
    //         validation_url,
    //         confirmation_url,
    //         response_type,
    //         short_code,
    //     };
    //
    //     let data = json!({
    //         "ValidationURL": payload.validation_url,
    //         "ConfirmationURL": payload.confirmation_url,
    //         "ResponseType": payload.response_type.to_string(),
    //         "ShortCode": payload.short_code,
    //     });
    //
    //     let response = Client::new()
    //         .post(&url)
    //         .bearer_auth(self.auth()?)
    //         .json(&data)
    //         .send()?;
    //
    //     Ok(response)
    // }
    //
    // /// Make payment requests from Client to Business
    // ///
    // /// This enables you to receive the payment requests in real time.
    // /// See more here: https://developer.safaricom.co.ke/c2b/apis/post/simulate
    // ///
    // /// # Example
    // /// ```
    // /// dotenv::dotenv().ok();
    // ///
    // /// let client = mpesa::Mpesa::new(
    // ///    std::env::var("CLIENT_KEY").unwrap(),
    // ///    std::env::var("CLIENT_SECRET").unwrap(),
    // ///    mpesa::Environment::Sandbox,
    // ///    std::env::var("INIT_PASSWORD").unwrap(),
    // /// );
    // ///
    // /// let c2b_simulate_response = client.c2b_simulate(
    // ///         mpesa::CommandId::CustomerPayBillOnline,
    // ///         1,
    // ///         "254705583540",
    // ///         "123abc",
    // ///         "600496"
    // ///     ).unwrap();
    // /// ```
    // ///
    // /// # Errors
    // /// TODO
    // pub fn c2b_simulate(
    //     &self,
    //     command_id: CommandId,
    //     amount: u32,
    //     msisdn: &str,
    //     bill_ref_number: &str,
    //     short_code: &str,
    // ) -> Result<C2bSimulateResponse, Box<dyn std::error::Error>> {
    //     let url = format!("{}/mpesa/c2b/v1/simulate", self.environment.base_url());
    //
    //     let payload = C2bSimulatePayload {
    //         command_id,
    //         amount,
    //         msisdn,
    //         bill_ref_number,
    //         short_code,
    //     };
    //
    //     let data = json!({
    //         "CommandID": payload.command_id.to_string(),
    //         "Amount": payload.amount,
    //         "Msisdn": payload.msisdn,
    //         "BillRefNumber": payload.bill_ref_number,
    //         "ShortCode": short_code,
    //     });
    //
    //     let response: C2bSimulateResponse = Client::new()
    //         .post(&url)
    //         .bearer_auth(self.auth()?)
    //         .json(&data)
    //         .send()?
    //         .json()?;
    //
    //     Ok(response)
    // }
    //
    // /// Enquire the balance on an M-Pesa BuyGoods (Till Number).
    // ///
    // /// # Example
    // /// ```
    // /// dotenv::dotenv().ok();
    // ///
    // /// let client = mpesa::Mpesa::new(
    // ///    std::env::var("CLIENT_KEY").unwrap(),
    // ///    std::env::var("CLIENT_SECRET").unwrap(),
    // ///    mpesa::Environment::Sandbox,
    // ///    std::env::var("INIT_PASSWORD").unwrap(),
    // /// );
    // ///
    // /// let account_balance_response = client.account_balance(
    // ///         "600496",
    // ///         "none",
    // ///         "collins",
    // ///         "https://hell.world/api",
    // ///         "https://hello.world/api"
    // ///     ).unwrap();
    // /// ```
    // ///
    // /// # Errors
    // /// TODO
    // pub fn account_balance(
    //     &self,
    //     party_a: &str,
    //     remarks: &str,
    //     initiator_name: &str,
    //     queue_timeout_url: &str,
    //     result_url: &str,
    // ) -> Result<AccountBalanceResponse, Box<dyn std::error::Error>> {
    //     let url = format!(
    //         "{}/mpesa/accountbalance/v1/query",
    //         self.environment.base_url()
    //     );
    //     let credentials = self.gen_security_credentials()?;
    //
    //     let payload = AccountBalancePayload {
    //         command_id: CommandId::AccountBalance,
    //         party_a,
    //         identifier_type: IdentifierTypes::Shortcode,
    //         remarks,
    //         initiator_name,
    //         queue_timeout_url,
    //         result_url,
    //         security_credentials: &credentials,
    //     };
    //
    //     let data = json!({
    //         "CommandID": payload.command_id.to_string(),
    //         "PartyA": payload.party_a,
    //         "IdentifierType": "4", // FIXME
    //         "Remarks": payload.remarks,
    //         "Initiator": payload.initiator_name,
    //         "SecurityCredential": payload.security_credentials,
    //         "QueueTimeOutURL": payload.queue_timeout_url,
    //         "ResultURL": payload.result_url,
    //     });
    //
    //     let response = Client::new()
    //         .post(&url)
    //         .bearer_auth(self.auth()?)
    //         .json(&data)
    //         .send()?
    //         .json()?;
    //
    //     Ok(response)
    // }
}

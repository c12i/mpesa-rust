use super::environment::Environment;
use super::services::{B2bBuilder, B2cBuilder, C2bRegisterBuilder, C2bSimulateBuilder};
use crate::services::{AccountBalancePayload, AccountBalanceResponse};
use crate::MpesaError;
use crate::{CommandId, IdentifierTypes};
use reqwest::blocking::Client;
use serde_json::Value;

/// `Result` enum type alias
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

    /// # C2B Register builder
    /// Creates a `C2bRegisterBuilder` for registering URLs to the 3rd party shortcode.
    /// ```
    /// let response = client
    ///     .c2b_register()
    ///     .short_code("600496")
    ///     .confirmation_url("https://testdomain.com/true")
    ///     .validation_url("https://testdomain.com/valid")
    ///     .send();
    /// ```
    pub fn c2b_register(&'a self) -> C2bRegisterBuilder<'a> {
        C2bRegisterBuilder::new(&self)
    }

    /// # C2B Simulate Builder
    /// Creates a `C2bSimulateBuilder` for simulating C2B transactions
    /// ```
    /// let response = client.c2b_simulate()
    ///         .short_code("600496")
    ///         .msisdn("254700000000")
    ///         .amount(1000)
    ///         .send();
    /// ```
    pub fn c2b_simulate(&'a self) -> C2bSimulateBuilder<'a> {
        C2bSimulateBuilder::new(&self)
    }

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

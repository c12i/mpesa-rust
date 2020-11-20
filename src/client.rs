use super::environment::Environment;
use super::services::{
    AccountBalanceBuilder, B2bBuilder, B2cBuilder, C2bRegisterBuilder, C2bSimulateBuilder,
};
use crate::MpesaError;
use reqwest::blocking::Client;
use serde_json::Value;

/// `Result` enum type alias
pub type MpesaResult<T> = Result<T, MpesaError>;

/// Mpesa client that will facilitate communication with the Safaricom API
#[derive(Debug)]
pub struct Mpesa {
    client_key: String,
    client_secret: String,
    initiator_password: Option<String>,
    environment: Environment,
}

impl<'a> Mpesa {
    /// Constructs a new `Mpesa` instance.
    pub fn new(client_key: String, client_secret: String, environment: Environment) -> Self {
        Self {
            client_key,
            client_secret,
            initiator_password: None,
            environment,
        }
    }

    /// Gets the current `Environment`
    pub fn environment(&'a self) -> &Environment {
        &self.environment
    }

    /// Gets the initiator password as a byte slice
    /// If `None`, the default password is b"Safcom496!"
    pub fn initiator_password(&'a self) -> &'a [u8] {
        if let Some(p) = &self.initiator_password {
            return p.as_bytes();
        }
        b"Safcom496!"
    }

    /// Optional in development but required for production, you will need to call this method and set your production initiator password.
    /// If in development, default initiator password is already pre-set
    /// ```ignore
    /// use mpesa::Mpesa;
    ///
    /// let client: Mpesa = Mpesa::new(
    ///     env::var("CLIENT_KEY").unwrap(),
    ///     env::var("CLIENT_SECRET").unwrap(),
    ///     "sandbox".parse().unwrap(),
    /// ).set_initiator_password("your_initiator_password");
    /// ```
    pub fn set_initiator_password(mut self, initiator_password: &str) -> Self {
        self.initiator_password = Some(initiator_password.into());
        self
    }

    /// Checks if the client can be authenticated
    pub fn is_connected(&self) -> Option<bool> {
        let token = self.auth().ok();
        if let Some(_) = token {
            return Some(true);
        }
        None
    }

    /// **Safaricom Oauth**
    ///
    /// Generates an access token
    /// Sends `GET` request to Safaricom oauth to acquire token for token authentication
    /// The OAuth access token expires after an hour, after which, you will need to generate another access token
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/docs#authentication)
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

    /// **B2C Builder**
    /// Creates a `B2cBuilder` for building a B2C transaction struct.
    /// The builder is consumed and request made by calling its `send` method.
    /// See more from Safaricom the API docs [here](https://developer.safaricom.co.ke/docs?shell#b2c-api).
    ///
    /// Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///     .b2c("testapi496")
    ///     .parties("600496", "254708374149")
    ///     .urls("https://testdomain.com/err", "https://testdomain.com/res")
    ///     .amount(1000)
    ///     .remarks("Your Remark") // optional, defaults to "None"
    ///     .occasion("Your Occasion") // optional, defaults to "None"
    ///     .command_id(mpesa::CommandId::BusinessPayment) // optional, defaults to `CommandId::BusinessPayment`
    ///     .send();
    /// ```
    pub fn b2c(&'a self, initiator_name: &'a str) -> B2cBuilder<'a> {
        B2cBuilder::new(&self, initiator_name)
    }

    /// **B2B Builder**
    ///
    /// Creates a `B2bBuilder` for building B2B transaction struct.
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/docs#b2b-api)
    ///
    /// Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
    ///
    /// # Example
    /// ```ignore
    /// let response = client.b2b("testapi496")
    ///    .parties("600496", "600000")
    ///    .urls("https://testdomain.com/err", "https://testdomain.com/api")
    ///    .account_ref("254708374149")
    ///    .amount(1000)
    ///    .command_id(mpesa::CommandId::BusinessToBusinessTransfer) // optional, defaults to `CommandId::BusinessToBusinessTransfer`
    ///    .remarks("None") // optional, defaults to "None"
    ///    .sender_id(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
    ///    .receiver_id(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
    ///    .send();
    /// ```
    pub fn b2b(&'a self, initiator_name: &'a str) -> B2bBuilder<'a> {
        B2bBuilder::new(&self, initiator_name)
    }

    /// **C2B Register builder**
    ///
    /// Creates a `C2bRegisterBuilder` for registering URLs to the 3rd party shortcode.
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/docs?shell#c2b-api)
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///    .c2b_register()
    ///    .short_code("600496")
    ///    .confirmation_url("https://testdomain.com/true")
    ///    .validation_url("https://testdomain.com/valid")
    ///    .response_type(mpesa::ResponseTypes::Complete) // optional, defaults to `ResponseTypes::Complete`
    ///    .send();
    /// ```
    pub fn c2b_register(&'a self) -> C2bRegisterBuilder<'a> {
        C2bRegisterBuilder::new(&self)
    }

    /// **C2B Simulate Builder**
    ///
    /// Creates a `C2bSimulateBuilder` for simulating C2B transactions
    ///
    /// See more [here](https://developer.safaricom.co.ke/c2b/apis/post/simulate)
    ///
    /// # Example
    /// ```ignore
    /// let response = client.c2b_simulate()
    ///    .short_code("600496")
    ///    .msisdn("254700000000")
    ///    .amount(1000)
    ///    .command_id(mpesa::CommandId::CustomerPayBillOnline) // optional, defaults to `CommandId::CustomerPayBillOnline`
    ///    .bill_ref_number("Your_BillRefNumber>") // optional, defaults to "None"
    ///    .send();
    /// ```
    pub fn c2b_simulate(&'a self) -> C2bSimulateBuilder<'a> {
        C2bSimulateBuilder::new(&self)
    }

    /// **Account Balance Builder**
    ///
    /// Creates an `AccountBalanceBuilder` for enquiring the balance on an MPESA BuyGoods.
    /// Requires an `initiator_name`.
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/docs#account-balance-api)
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///    .account_balance("testapi496")
    ///    .urls("https://testdomain.com/err", "https://testdomain.com/ok")
    ///    .party_a("600496")
    ///    .command_id(mpesa::CommandId::AccountBalance) // optional, defaults to `CommandId::AccountBalance`
    ///    .identifier_type(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
    ///    .remarks("Your Remarks") // optional, defaults to "None"
    ///    .send();
    /// ```
    pub fn account_balance(&'a self, initiator_name: &'a str) -> AccountBalanceBuilder<'a> {
        AccountBalanceBuilder::new(&self, initiator_name)
    }
}

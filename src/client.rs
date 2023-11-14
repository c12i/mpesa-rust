use std::cell::RefCell;

use cached::Cached;
use openssl::base64;
use openssl::rsa::Padding;
use openssl::x509::X509;
use reqwest::Client as HttpClient;

use crate::auth::AUTH;
use crate::environment::ApiEnvironment;
use crate::services::{
    AccountBalanceBuilder, B2bBuilder, B2cBuilder, BulkInvoiceBuilder, C2bRegisterBuilder,
    C2bSimulateBuilder, CancelInvoiceBuilder, MpesaExpressRequestBuilder, OnboardBuilder,
    OnboardModifyBuilder, ReconciliationBuilder, SingleInvoiceBuilder, TransactionReversalBuilder,
    TransactionStatusBuilder,
};
use crate::{auth, MpesaResult};
use secrecy::{ExposeSecret, Secret};

/// Source: [test credentials](https://developer.safaricom.co.ke/test_credentials)
const DEFAULT_INITIATOR_PASSWORD: &str = "Safcom496!";
/// Get current package version from metadata
const CARGO_PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Mpesa client that will facilitate communication with the Safaricom API
#[derive(Clone, Debug)]
pub struct Mpesa<Env: ApiEnvironment> {
    client_key: String,
    client_secret: Secret<String>,
    initiator_password: RefCell<Option<Secret<String>>>,
    pub(crate) environment: Env,
    pub(crate) http_client: HttpClient,
}

impl<'mpesa, Env: ApiEnvironment> Mpesa<Env> {
    /// Constructs a new `Mpesa` instance.
    ///
    /// # Example
    /// ```ignore
    /// let client: Mpesa = Mpesa::new(
    ///     env!("CLIENT_KEY"),
    ///     env!("CLIENT_SECRET"),
    ///     Environment::Sandbox,
    /// );
    /// ```
    ///
    /// # Panics
    /// This method can panic if a TLS backend cannot be initialized for the internal http_client
    pub fn new<S: Into<String>>(client_key: S, client_secret: S, environment: Env) -> Self {
        let http_client = HttpClient::builder()
            .connect_timeout(std::time::Duration::from_millis(10_000))
            .user_agent(format!("mpesa-rust@{CARGO_PACKAGE_VERSION}"))
            // TODO: Potentialy return a `Result` enum from Mpesa::new?
            //       Making assumption that creation of http client cannot fail
            .build()
            .expect("Error building http client");
        Self {
            client_key: client_key.into(),
            client_secret: Secret::new(client_secret.into()),
            initiator_password: RefCell::new(None),
            environment,
            http_client,
        }
    }

    /// Gets the initiator password
    /// If `None`, the default password is `"Safcom496!"`
    pub(crate) fn initiator_password(&'mpesa self) -> String {
        let Some(p) = &*self.initiator_password.borrow() else {
            return DEFAULT_INITIATOR_PASSWORD.to_owned();
        };
        p.expose_secret().into()
    }

    /// Get the client key
    pub(crate) fn client_key(&self) -> &str {
        &self.client_key
    }

    /// Get the client secret
    pub(crate) fn client_secret(&self) -> &str {
        self.client_secret.expose_secret()
    }

    /// Optional in development but required for production, you will need to call this method and set your production initiator password.
    /// If in development, default initiator password is already pre-set
    /// ```ignore
    /// use mpesa::Mpesa;
    ///
    /// let client: Mpesa = Mpesa::new(
    ///     env::var("CLIENT_KEY").unwrap(),
    ///     env::var("CLIENT_SECRET").unwrap(),
    ///     Environment::Sandbox,
    /// );
    ///
    /// client.set_initiator_password("your_initiator_password");
    /// ```
    pub fn set_initiator_password<S: Into<String>>(&self, initiator_password: S) {
        *self.initiator_password.borrow_mut() = Some(Secret::new(initiator_password.into()));
    }

    /// Checks if the client can be authenticated
    pub async fn is_connected(&self) -> bool {
        self.auth().await.is_ok()
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
    pub(crate) async fn auth(&self) -> MpesaResult<String> {
        if let Some(token) = AUTH.lock().await.cache_get(&self.client_key) {
            return Ok(token.to_owned());
        }

        // Generate a new access token
        let new_token = match auth::auth_prime_cache(self).await {
            Ok(token) => token,
            Err(e) => return Err(e),
        };

        // Double-check if the access token is cached by another thread
        if let Some(token) = AUTH.lock().await.cache_get(&self.client_key) {
            return Ok(token.to_owned());
        }

        // Cache the new token
        AUTH.lock()
            .await
            .cache_set(self.client_key.clone(), new_token.to_owned());

        Ok(new_token)
    }

    /// **B2C Builder**
    ///
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
    ///     .party_a("600496")
    ///     .party_b("600000")
    ///     .result_url("https://testdomain.com/err")
    ///     .timeout_url("https://testdomain.com/ok")
    ///     .amount(1000)
    ///     .remarks("Your Remark") // optional, defaults to "None"
    ///     .occasion("Your Occasion") // optional, defaults to "None"
    ///     .command_id(mpesa::CommandId::BusinessPayment) // optional, defaults to `CommandId::BusinessPayment`
    ///     .send();
    /// ```
    #[cfg(feature = "b2c")]
    pub fn b2c(&'mpesa self, initiator_name: &'mpesa str) -> B2cBuilder<'mpesa, Env> {
        B2cBuilder::new(self, initiator_name)
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
    ///    .party_a("600496")
    ///    .party_b("600000")
    ///    .result_url("https://testdomain.com/err")
    ///    .timeout_url("https://testdomain.com/ok")
    ///    .account_ref("254708374149")
    ///    .amount(1000)
    ///    .command_id(mpesa::CommandId::BusinessToBusinessTransfer) // optional, defaults to `CommandId::BusinessToBusinessTransfer`
    ///    .remarks("None") // optional, defaults to "None"
    ///    .sender_id(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
    ///    .receiver_id(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
    ///    .send();
    /// ```
    #[cfg(feature = "b2b")]
    pub fn b2b(&'mpesa self, initiator_name: &'mpesa str) -> B2bBuilder<'mpesa, Env> {
        B2bBuilder::new(self, initiator_name)
    }

    /// **Bill Manager Onboard Builder**
    ///
    /// Creates a `OnboardBuilder` which allows you to opt in as a biller to the bill manager features.
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/APIs/BillManager)
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///     .onboard()
    ///     .callback_url("https://testdomain.com/true")
    ///     .email("email@test.com")
    ///     .logo("https://file.domain/file.png")
    ///     .official_contact("0712345678")
    ///     .send_reminders(SendRemindersTypes::Enable)
    ///     .short_code("600496")
    ///     .send()
    ///     .await;
    /// ```
    #[cfg(feature = "bill_manager")]
    pub fn onboard(&'mpesa self) -> OnboardBuilder<'mpesa, Env> {
        OnboardBuilder::new(self)
    }

    /// **Bill Manager Onboard Modify Builder**
    ///
    /// Creates a `OnboardModifyBuilder` which allows you to opt in as a biller to the bill manager features.
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/APIs/BillManager)
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///     .onboard_modify()
    ///     .callback_url("https://testdomain.com/true")
    ///     .email("email@test.com")
    ///     .logo("https://file.domain/file.png")
    ///     .official_contact("0712345678")
    ///     .send_reminders(SendRemindersTypes::Enable)
    ///     .short_code("600496")
    ///     .send()
    ///     .await;
    /// ```
    #[cfg(feature = "bill_manager")]
    pub fn onboard_modify(&'mpesa self) -> OnboardModifyBuilder<'mpesa, Env> {
        OnboardModifyBuilder::new(self)
    }

    /// **Bill Manager Bulk Invoice Builder**
    ///
    /// Creates a `BulkInvoiceBuilder` which allows you to send invoices to your customers in bulk.
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/APIs/BillManager)
    ///
    /// # Example
    /// ```ignore
    /// use chrone::prelude::Utc;
    ///
    /// let response = client
    ///     .bulk_invoice()
    ///
    ///     // Add multiple invoices at once
    ///     .invoices(vec![
    ///         Invoice {
    ///             amount: 1000.0,
    ///             account_reference: "John Doe",
    ///             billed_full_name: "John Doe",
    ///             billed_period: "August 2021",
    ///             billed_phone_number: "0712345678",
    ///             due_date: Utc::now(),
    ///             external_reference: "INV2345",
    ///             invoice_items: Some(
    ///                 vec![InvoiceItem {amount: 1000.0, item_name: "An item"}]
    ///             ),
    ///             invoice_name: "Invoice 001"
    ///         }
    ///     ])
    ///
    ///     // Add a single invoice
    ///     .invoice(
    ///         Invoice {
    ///             amount: 1000.0,
    ///             account_reference: "John Doe",
    ///             billed_full_name: "John Doe",
    ///             billed_period: "August 2021",
    ///             billed_phone_number: "0712345678",
    ///             due_date: Utc::now(),
    ///             external_reference: "INV2345",
    ///             invoice_items: Some(vec![InvoiceItem {
    ///                 amount: 1000.0,
    ///                 item_name: "An item",
    ///             }]),
    ///             invoice_name: "Invoice 001",
    ///         }
    ///     )
    ///     .send()
    ///     .await;
    /// ```
    #[cfg(feature = "bill_manager")]
    pub fn bulk_invoice(&'mpesa self) -> BulkInvoiceBuilder<'mpesa, Env> {
        BulkInvoiceBuilder::new(self)
    }

    /// **Bill Manager Single Invoice Builder**
    ///
    /// Creates a `SingleInvoiceBuilder` which allows you to create and send invoices to your customers.
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/APIs/BillManager)
    ///
    /// # Example
    /// ```ignore
    /// use chrono::prelude::Utc;
    ///
    /// let response = client
    ///     .single_invoice()
    ///     .amount(1000.0)
    ///     .account_reference("John Doe")
    ///     .billed_full_name("John Doe")
    ///     .billed_period("August 2021")
    ///     .billed_phone_number("0712345678")
    ///     .due_date(Utc::now())
    ///     .external_reference("INV2345")
    ///     .invoice_items(vec![
    ///         InvoiceItem {amount: 1000.0, item_name: "An item"}
    ///     ])
    ///     .invoice_name("Invoice 001")
    ///     .send()
    ///     .await;
    /// ```
    #[cfg(feature = "bill_manager")]
    pub fn single_invoice(&'mpesa self) -> SingleInvoiceBuilder<'mpesa, Env> {
        SingleInvoiceBuilder::new(self)
    }

    /// **Bill Manager Reconciliation Builder**
    ///
    /// Creates a `ReconciliationBuilder` which enables your customers to receive e-receipts for payments made to your paybill account.
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/APIs/BillManager)
    ///
    /// # Example
    /// ```ignore
    /// use chrono::prelude::Utc;
    ///
    /// let response = client
    ///     .reconciliation()
    ///     .account_reference("John Doe")
    ///     .external_reference("INV2345")
    ///     .full_name("John Doe")
    ///     .invoice_name("Invoice 001")
    ///     .paid_amount(1000.0)
    ///     .payment_date(Utc::now())
    ///     .phone_number("0712345678")
    ///     .transaction_id("TRANSACTION_ID")
    ///     .send()
    ///     .await;
    /// ```
    #[cfg(feature = "bill_manager")]
    pub fn reconciliation(&'mpesa self) -> ReconciliationBuilder<'mpesa, Env> {
        ReconciliationBuilder::new(self)
    }

    /// **Bill Manager Cancel Invoice Builder**
    ///
    /// Creates a `CancelInvoiceBuilder` which allows you to recall a sent invoice.
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/APIs/BillManager)
    ///
    /// # Example
    /// ```ignore
    /// use chrono::prelude::Utc;
    ///
    /// let response = client
    ///     .cancel_invoice()
    ///     .external_references(vec!["9KLSS011"])
    ///     .send()
    ///     .await;
    /// ```
    #[cfg(feature = "bill_manager")]
    pub fn cancel_invoice(&'mpesa self) -> CancelInvoiceBuilder<'mpesa, Env> {
        CancelInvoiceBuilder::new(self)
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
    #[cfg(feature = "c2b_register")]
    pub fn c2b_register(&'mpesa self) -> C2bRegisterBuilder<'mpesa, Env> {
        C2bRegisterBuilder::new(self)
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
    #[cfg(feature = "c2b_simulate")]
    pub fn c2b_simulate(&'mpesa self) -> C2bSimulateBuilder<'mpesa, Env> {
        C2bSimulateBuilder::new(self)
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
    ///    .result_url("https://testdomain.com/err")
    ///    .timeout_url("https://testdomain.com/ok")
    ///    .party_a("600496")
    ///    .command_id(mpesa::CommandId::AccountBalance) // optional, defaults to `CommandId::AccountBalance`
    ///    .identifier_type(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
    ///    .remarks("Your Remarks") // optional, defaults to "None"
    ///    .send();
    /// ```
    #[cfg(feature = "account_balance")]
    pub fn account_balance(
        &'mpesa self,
        initiator_name: &'mpesa str,
    ) -> AccountBalanceBuilder<'mpesa, Env> {
        AccountBalanceBuilder::new(self, initiator_name)
    }

    /// **Mpesa Express Request/ STK push Builder**
    ///
    /// Creates a `MpesaExpressRequestBuilder` struct
    /// Requires a `business_short_code` - The organization shortcode used to receive the transaction
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/docs#lipa-na-m-pesa-online-payment)
    ///
    /// # Example
    ///```ignore
    /// let response = client
    ///    .express_request("174379")
    ///    .phone_number("254708374149")
    ///    .party_a("254708374149")
    ///    .party_b("174379")
    ///    .amount(500)
    ///    .callback_url("https://test.example.com/api")
    ///    .transaction_type(CommandId::CustomerPayBillOnline) // Optional, defaults to `CommandId::CustomerPayBillOnline`
    ///    .transaction_desc("Description") // Optional, defaults to "None"
    ///    .send();
    /// ```
    #[cfg(feature = "express_request")]
    pub fn express_request(
        &'mpesa self,
        business_short_code: &'mpesa str,
    ) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        MpesaExpressRequestBuilder::new(self, business_short_code)
    }

    ///**Transaction Reversal Builder**
    /// Reverses a B2B, B2C or C2B M-Pesa transaction.
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/Documentation)
    #[cfg(feature = "transaction_reversal")]
    pub fn transaction_reversal(
        &'mpesa self,
        initiator_name: &'mpesa str,
    ) -> TransactionReversalBuilder<'mpesa, Env> {
        TransactionReversalBuilder::new(self, initiator_name)
    }

    ///**Transaction Status Builder**
    /// Queries the status of a B2B, B2C or C2B M-Pesa transaction.
    ///
    /// See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/Documentation)
    /// # Example
    /// ```ignore
    /// let response = client
    ///   .transaction_status("testapi496")
    ///   .party_a("600496")
    ///   .identifier_type(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
    ///   .remarks("Your Remarks") // optional, defaults to "None"
    ///   .result_url("https://testdomain.com/err")
    ///   .timeout_url("https://testdomain.com/ok")
    ///   .send()
    ///   .await;
    /// ```
    #[cfg(feature = "transaction_status")]
    pub fn transaction_status(
        &'mpesa self,
        initiator_name: &'mpesa str,
    ) -> TransactionStatusBuilder<'mpesa, Env> {
        TransactionStatusBuilder::new(self, initiator_name)
    }

    /// Generates security credentials
    /// M-Pesa Core authenticates a transaction by decrypting the security credentials.
    /// Security credentials are generated by encrypting the base64 encoded initiator password with M-Pesa’s public key, a X509 certificate.
    /// Returns base64 encoded string.
    ///
    /// # Errors
    /// Returns `EncryptionError` variant of `MpesaError`
    pub(crate) fn gen_security_credentials(&self) -> MpesaResult<String> {
        let pem = self.environment.get_certificate().as_bytes();
        let cert = X509::from_pem(pem)?;
        // getting the public and rsa keys
        let pub_key = cert.public_key()?;
        let rsa_key = pub_key.rsa()?;
        // configuring the buffer
        let buf_len = pub_key.size();
        let mut buffer = vec![0; buf_len];

        rsa_key.public_encrypt(
            self.initiator_password().as_bytes(),
            &mut buffer,
            Padding::PKCS1,
        )?;
        Ok(base64::encode_block(&buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sandbox;

    #[test]
    fn test_setting_initator_password() {
        let client = Mpesa::new("client_key", "client_secret", Sandbox);
        assert_eq!(client.initiator_password(), DEFAULT_INITIATOR_PASSWORD);
        client.set_initiator_password("foo_bar");
        assert_eq!(client.initiator_password(), "foo_bar".to_string());
    }

    struct TestEnvironment;

    impl ApiEnvironment for TestEnvironment {
        fn base_url(&self) -> &str {
            "https://example.com"
        }

        fn get_certificate(&self) -> &str {
            // not a valid pem
            "certificate"
        }
    }

    #[test]
    fn test_custom_environment() {
        let client = Mpesa::new("client_key", "client_secret", TestEnvironment);
        assert_eq!(client.environment.base_url(), "https://example.com");
        assert_eq!(client.environment.get_certificate(), "certificate");
    }

    #[test]
    #[should_panic]
    fn test_gen_security_credentials_fails_with_invalid_pem() {
        let client = Mpesa::new("client_key", "client_secret", TestEnvironment);
        let _ = client.gen_security_credentials().unwrap();
    }
}

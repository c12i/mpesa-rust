use std::cell::RefCell;

use cached::Cached;
use openssl::base64;
use openssl::rsa::Padding;
use openssl::x509::X509;
use reqwest::Client as HttpClient;
use secrecy::{ExposeSecret, Secret};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::auth::AUTH;
use crate::environment::ApiEnvironment;
use crate::services::{
    AccountBalanceBuilder, B2bBuilder, B2cBuilder, BulkInvoiceBuilder, C2bRegisterBuilder,
    C2bSimulateBuilder, CancelInvoiceBuilder, DynamicQR, DynamicQRBuilder,
    MpesaExpressRequestBuilder, OnboardBuilder, OnboardModifyBuilder, ReconciliationBuilder,
    SingleInvoiceBuilder, TransactionReversalBuilder, TransactionStatusBuilder,
};
use crate::{auth, MpesaResult};

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
    /// Constructs a new `Mpesa` client.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mpesa::{Mpesa, Environment};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    dotenv::dotenv().ok();
    ///
    ///    let client = Mpesa::new(
    ///         env!("CLIENT_KEY"),
    ///         env!("CLIENT_SECRET"),
    ///         Environment::Sandbox,
    ///    );
    ///
    ///    assert!(client.is_connected().await);
    /// }
    /// ```
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

    /// Optional in development but required for production for the following apis:
    /// - `account_balance`
    /// - `b2b`
    /// - `b2c`
    /// - `transaction_reversal`
    /// - `transaction_status`
    ///
    /// You will need to call this method and set your production initiator password.
    /// If in development, a default initiator password from the test credentials is already pre-set
    ///
    /// # Example
    /// ```rust
    /// use mpesa::{Mpesa, Environment};
    ///
    /// fn main() {
    ///     dotenv::dotenv().ok();
    ///
    ///     let client = Mpesa::new(
    ///         env!("CLIENT_KEY"),
    ///         env!("CLIENT_SECRET"),
    ///         Environment::Sandbox,
    ///     );
    ///     client.set_initiator_password("your_initiator_password");
    /// }
    /// ```
    pub fn set_initiator_password<S: Into<String>>(&self, initiator_password: S) {
        *self.initiator_password.borrow_mut() = Some(Secret::new(initiator_password.into()));
    }

    /// Checks if the client can be authenticated
    pub async fn is_connected(&self) -> bool {
        self.auth().await.is_ok()
    }

    /// This API generates the tokens for authenticating your API calls. This is the first API you will engage with within the set of APIs available because all the other APIs require authentication information from this API to work.
    ///
    /// Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/Authorization)
    ///
    /// Returns auth token as a `String` that is ttl-cached in memory for subsequent requests.
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

    #[cfg(feature = "b2c")]
    #[doc = include_str!("../docs/client/b2c.md")]
    pub fn b2c(&'mpesa self, initiator_name: &'mpesa str) -> B2cBuilder<'mpesa, Env> {
        B2cBuilder::new(self, initiator_name)
    }

    #[cfg(feature = "b2b")]
    #[doc = include_str!("../docs/client/b2b.md")]
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

    #[cfg(feature = "c2b_register")]
    #[doc = include_str!("../docs/client/c2b_register.md")]
    pub fn c2b_register(&'mpesa self) -> C2bRegisterBuilder<'mpesa, Env> {
        C2bRegisterBuilder::new(self)
    }

    #[cfg(feature = "c2b_simulate")]
    #[doc = include_str!("../docs/client/c2b_simulate.md")]
    pub fn c2b_simulate(&'mpesa self) -> C2bSimulateBuilder<'mpesa, Env> {
        C2bSimulateBuilder::new(self)
    }

    #[cfg(feature = "account_balance")]
    #[doc = include_str!("../docs/client/account_balance.md")]
    pub fn account_balance(
        &'mpesa self,
        initiator_name: &'mpesa str,
    ) -> AccountBalanceBuilder<'mpesa, Env> {
        AccountBalanceBuilder::new(self, initiator_name)
    }

    #[cfg(feature = "express_request")]
    #[doc = include_str!("../docs/client/express_request.md")]
    pub fn express_request(
        &'mpesa self,
        business_short_code: &'mpesa str,
    ) -> MpesaExpressRequestBuilder<'mpesa, Env> {
        MpesaExpressRequestBuilder::new(self, business_short_code)
    }

    #[cfg(feature = "transaction_reversal")]
    #[doc = include_str!("../docs/client/transaction_reversal.md")]
    pub fn transaction_reversal(
        &'mpesa self,
        initiator_name: &'mpesa str,
    ) -> TransactionReversalBuilder<'mpesa, Env> {
        TransactionReversalBuilder::new(self, initiator_name)
    }

    #[cfg(feature = "transaction_status")]
    #[doc = include_str!("../docs/client/transaction_status.md")]
    pub fn transaction_status(
        &'mpesa self,
        initiator_name: &'mpesa str,
    ) -> TransactionStatusBuilder<'mpesa, Env> {
        TransactionStatusBuilder::new(self, initiator_name)
    }

    #[cfg(feature = "dynamic_qr")]
    #[doc = include_str!("../docs/client/dynamic_qr.md")]
    pub fn dynamic_qr(&'mpesa self) -> DynamicQRBuilder<'mpesa, Env> {
        DynamicQR::builder(self)
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

    /// Sends a request to the Safaricom API
    /// This method is used by all the builders to send requests to the
    /// Safaricom API
    pub(crate) async fn send<Req, Res>(&self, req: Request<'_, Req>) -> MpesaResult<Res>
    where
        Req: Serialize + Send,
        Res: DeserializeOwned,
    {
        let url = format!("{}/{}", self.environment.base_url(), req.path);

        let req = self
            .http_client
            .request(req.method, url)
            .bearer_auth(self.auth().await?)
            .json(&req.body);

        let res = req.send().await?;

        if res.status().is_success() {
            let body = res.json().await?;

            Ok(body)
        } else {
            let err = res.json::<crate::ResponseError>().await?;

            Err(crate::MpesaError::Service(err))
        }
    }
}

pub struct Request<'a, Body: Serialize + Send> {
    pub method: reqwest::Method,
    pub path: &'a str,
    pub body: Body,
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

    #[derive(Clone)]
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

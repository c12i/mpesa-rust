use std::cell::RefCell;
use std::time::Duration;

use cached::Cached;
#[cfg(any(
    feature = "account_balance",
    feature = "b2b",
    feature = "b2c",
    feature = "transaction_reversal",
    feature = "transaction_status"
))]
use openssl::base64;
#[cfg(any(
    feature = "account_balance",
    feature = "b2b",
    feature = "b2c",
    feature = "transaction_reversal",
    feature = "transaction_status"
))]
use openssl::rsa::Padding;
#[cfg(any(
    feature = "account_balance",
    feature = "b2b",
    feature = "b2c",
    feature = "transaction_reversal",
    feature = "transaction_status"
))]
use openssl::x509::X509;
use reqwest::Client as HttpClient;
use secrecy::{ExposeSecret, Secret};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::auth::AUTH;
use crate::environment::ApiEnvironment;
#[cfg(feature = "account_balance")]
use crate::services::AccountBalanceBuilder;
#[cfg(feature = "b2b")]
use crate::services::B2bBuilder;
#[cfg(feature = "b2c")]
use crate::services::B2cBuilder;
#[cfg(feature = "c2b_register")]
use crate::services::C2bRegisterBuilder;
#[cfg(feature = "c2b_simulate")]
use crate::services::C2bSimulateBuilder;
#[cfg(feature = "transaction_status")]
use crate::services::TransactionStatusBuilder;
#[cfg(feature = "bill_manager")]
use crate::services::{
    BulkInvoiceBuilder, CancelInvoiceBuilder, OnboardBuilder, OnboardModifyBuilder,
    ReconciliationBuilder, SingleInvoiceBuilder,
};
#[cfg(feature = "dynamic_qr")]
use crate::services::{DynamicQR, DynamicQRBuilder};
#[cfg(feature = "express")]
use crate::services::{
    MpesaExpress, MpesaExpressBuilder, MpesaExpressQuery, MpesaExpressQueryBuilder,
};
#[cfg(feature = "transaction_reversal")]
use crate::services::{TransactionReversal, TransactionReversalBuilder};
use crate::{auth, MpesaError, MpesaResult, ResponseError};

/// Source: [test credentials](https://developer.safaricom.co.ke/test_credentials)
const DEFAULT_INITIATOR_PASSWORD: &str = "Safaricom999!*!";
/// Get current package version from metadata
const CARGO_PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Mpesa client that will facilitate communication with the Safaricom API
#[derive(Clone, Debug)]
pub struct Mpesa {
    consumer_key: String,
    consumer_secret: Secret<String>,
    initiator_password: RefCell<Option<Secret<String>>>,
    pub(crate) base_url: String,
    certificate: String,
    pub(crate) http_client: HttpClient,
}

impl Mpesa {
    /// Constructs a new `Mpesa` client.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mpesa::{Mpesa, Environment};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    dotenvy::dotenv().ok();
    ///
    ///    let client = Mpesa::new(
    ///         dotenvy::var("CONSUMER_KEY").unwrap(),
    ///         dotenvy::var("CONSUMER_SECRET").unwrap(),
    ///         Environment::Sandbox,
    ///    );
    ///
    ///    assert!(client.is_connected().await);
    /// }
    /// ```
    /// # Panics
    /// This method can panic if a TLS backend cannot be initialized for the internal http_client
    pub fn new<S: Into<String>>(
        consumer_key: S,
        consumer_secret: S,
        environment: impl ApiEnvironment,
    ) -> Self {
        let http_client = HttpClient::builder()
            .connect_timeout(Duration::from_secs(10))
            .user_agent(format!("mpesa-rust@{CARGO_PACKAGE_VERSION}"))
            .build()
            .expect("Error building http client");

        let base_url = environment.base_url().to_owned();
        let certificate = environment.get_certificate().to_owned();

        Self {
            consumer_key: consumer_key.into(),
            consumer_secret: Secret::new(consumer_secret.into()),
            initiator_password: RefCell::new(None),
            base_url,
            certificate,
            http_client,
        }
    }

    /// Gets the initiator password
    /// If `None`, the default password is `"Safcom496!"`
    pub(crate) fn initiator_password(&self) -> String {
        self.initiator_password
            .borrow()
            .as_ref()
            .map(|password| password.expose_secret().into())
            .unwrap_or(DEFAULT_INITIATOR_PASSWORD.to_owned())
    }

    /// Get the consumer key
    pub(crate) fn consumer_key(&self) -> &str {
        &self.consumer_key
    }

    /// Get the consumer secret
    pub(crate) fn consumer_secret(&self) -> &str {
        self.consumer_secret.expose_secret()
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
    ///
    /// ```rust
    /// use mpesa::{Mpesa, Environment};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenvy::dotenv().ok();
    ///
    ///     let client = Mpesa::new(
    ///         dotenvy::var("CONSUMER_KEY").unwrap(),
    ///         dotenvy::var("CONSUMER_SECRET").unwrap(),
    ///         Environment::Sandbox,
    ///     );
    ///     client.set_initiator_password("your_initiator_password");
    ///     assert!(client.is_connected().await);
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
        if let Some(token) = AUTH.lock().await.cache_get(&self.consumer_key) {
            return Ok(token.to_owned());
        }

        // Generate a new access token
        let new_token = auth::auth(self).await?;

        // Double-check if the access token is cached by another thread
        if let Some(token) = AUTH.lock().await.cache_get(&self.consumer_key) {
            return Ok(token.to_owned());
        }

        // Cache the new token
        AUTH.lock()
            .await
            .cache_set(self.consumer_key.clone(), new_token.to_owned());

        Ok(new_token)
    }

    #[cfg(feature = "b2c")]
    #[doc = include_str!("../docs/client/b2c.md")]
    pub fn b2c<'a>(&'a self, initiator_name: &'a str) -> B2cBuilder {
        B2cBuilder::new(self, initiator_name)
    }

    #[cfg(feature = "b2b")]
    #[doc = include_str!("../docs/client/b2b.md")]
    pub fn b2b<'a>(&'a self, initiator_name: &'a str) -> B2bBuilder {
        B2bBuilder::new(self, initiator_name)
    }

    #[cfg(feature = "bill_manager")]
    #[doc = include_str!("../docs/client/bill_manager/onboard.md")]
    pub fn onboard(&self) -> OnboardBuilder {
        OnboardBuilder::new(self)
    }

    #[cfg(feature = "bill_manager")]
    #[doc = include_str!("../docs/client/bill_manager/onboard_modify.md")]
    pub fn onboard_modify(&self) -> OnboardModifyBuilder {
        OnboardModifyBuilder::new(self)
    }

    #[cfg(feature = "bill_manager")]
    #[doc = include_str!("../docs/client/bill_manager/bulk_invoice.md")]
    pub fn bulk_invoice(&self) -> BulkInvoiceBuilder {
        BulkInvoiceBuilder::new(self)
    }

    #[cfg(feature = "bill_manager")]
    #[doc = include_str!("../docs/client/bill_manager/single_invoice.md")]
    pub fn single_invoice(&self) -> SingleInvoiceBuilder {
        SingleInvoiceBuilder::new(self)
    }

    #[cfg(feature = "bill_manager")]
    #[doc = include_str!("../docs/client/bill_manager/reconciliation.md")]
    pub fn reconciliation(&self) -> ReconciliationBuilder {
        ReconciliationBuilder::new(self)
    }

    #[cfg(feature = "bill_manager")]
    #[doc = include_str!("../docs/client/bill_manager/cancel_invoice.md")]
    pub fn cancel_invoice(&self) -> CancelInvoiceBuilder {
        CancelInvoiceBuilder::new(self)
    }

    #[cfg(feature = "c2b_register")]
    #[doc = include_str!("../docs/client/c2b_register.md")]
    pub fn c2b_register(&self) -> C2bRegisterBuilder {
        C2bRegisterBuilder::new(self)
    }

    #[cfg(feature = "c2b_simulate")]
    #[doc = include_str!("../docs/client/c2b_simulate.md")]
    pub fn c2b_simulate(&self) -> C2bSimulateBuilder {
        C2bSimulateBuilder::new(self)
    }

    #[cfg(feature = "account_balance")]
    #[doc = include_str!("../docs/client/account_balance.md")]
    pub fn account_balance<'a>(&'a self, initiator_name: &'a str) -> AccountBalanceBuilder {
        AccountBalanceBuilder::new(self, initiator_name)
    }

    #[cfg(feature = "express")]
    #[doc = include_str!("../docs/client/express.md")]
    pub fn express_request(&self) -> MpesaExpressBuilder {
        MpesaExpress::builder(self)
    }

    #[cfg(feature = "express")]
    #[doc = include_str!("../docs/client/express.md")]
    pub fn express_query(&self) -> MpesaExpressQueryBuilder {
        MpesaExpressQuery::builder(self)
    }

    #[cfg(feature = "transaction_reversal")]
    #[doc = include_str!("../docs/client/transaction_reversal.md")]
    pub fn transaction_reversal(&self) -> TransactionReversalBuilder {
        TransactionReversal::builder(self)
    }

    #[cfg(feature = "transaction_status")]
    #[doc = include_str!("../docs/client/transaction_status.md")]
    pub fn transaction_status<'a>(&'a self, initiator_name: &'a str) -> TransactionStatusBuilder {
        TransactionStatusBuilder::new(self, initiator_name)
    }

    #[cfg(feature = "dynamic_qr")]
    #[doc = include_str!("../docs/client/dynamic_qr.md")]
    pub fn dynamic_qr(&self) -> DynamicQRBuilder {
        DynamicQR::builder(self)
    }

    /// Generates security credentials
    /// M-Pesa Core authenticates a transaction by decrypting the security credentials.
    /// Security credentials are generated by encrypting the base64 encoded initiator password with M-Pesa’s public key, a X509 certificate.
    /// Returns base64 encoded string.
    ///
    /// # Errors
    /// Returns `EncryptionError` variant of `MpesaError`
    #[cfg(any(
        feature = "account_balance",
        feature = "b2b",
        feature = "b2c",
        feature = "transaction_reversal",
        feature = "transaction_status"
    ))]
    pub(crate) fn gen_security_credentials(&self) -> MpesaResult<String> {
        let pem = self.certificate.as_bytes();
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
    pub(crate) async fn send<Req, Res>(&self, req: Request<Req>) -> MpesaResult<Res>
    where
        Req: Serialize + Send,
        Res: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, req.path);

        let res = self
            .http_client
            .request(req.method, url)
            .bearer_auth(self.auth().await?)
            .json(&req.body)
            .send()
            .await?;

        if res.status().is_success() {
            let body = res.json().await?;
            Ok(body)
        } else {
            let err = res.json::<ResponseError>().await?;
            Err(MpesaError::Service(err))
        }
    }
}

pub struct Request<Body: Serialize + Send> {
    pub method: reqwest::Method,
    pub path: &'static str,
    pub body: Body,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sandbox;

    #[test]
    fn test_setting_initator_password() {
        let client = Mpesa::new("consumer_key", "consumer_secret", Sandbox);
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
        let client = Mpesa::new("consumer_key", "consumer_secret", TestEnvironment);
        assert_eq!(&client.base_url, "https://example.com");
        assert_eq!(&client.certificate, "certificate");
    }

    #[test]
    #[should_panic]
    fn test_gen_security_credentials_fails_with_invalid_pem() {
        let client = Mpesa::new("consumer_key", "consumer_secret", TestEnvironment);
        let _ = client.gen_security_credentials().unwrap();
    }
}

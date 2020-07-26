use std::collections::HashMap;
use std::error::Error;
use reqwest::blocking::{Client, Response};
use openssl::x509::X509;
use openssl::rsa::Padding;
use base64::encode;
use serde_json::json;

use super::utils::extract_auth_token;
use super::environment::Environment;
use super::payloads::B2cPayload;
use crate::CommandId;
use crate::payloads::B2cResponse;

/// Mpesa client that will facilitate communication with the Safaricom API
#[derive(Debug)]
pub struct Mpesa {
    client_key: String,
    client_secret: String,
    environment: Environment,
    initiator_password: String,
}

impl Mpesa {
    /// Constructs a new `Mpesa` instance.
    pub fn new(client_key: String, client_secret: String, environment: Environment, initiator_password: String) -> Self {
        Self {
            client_key,
            client_secret,
            environment,
            initiator_password,
        }
    }

    /// Generates an access token
    /// Sends `GET` request to Safaricom oauth to acquire token for token authentication
    /// The OAuth access token expires after an hour, after which, you will need to generate another access token
    fn auth(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/oauth/v1/generate?grant_type=client_credentials", self.environment.base_url());

        let resp: HashMap<String, String> = Client::new().get(&url)
            .basic_auth(&self.client_key, Some(&self.client_secret))
            .send()?
            .json()?;

        Ok(extract_auth_token(&resp)?)
    }

    /// Generates security credentials
    /// M-Pesa Core authenticates a transaction by decrypting the security credentials.
    /// Security credentials are generated by encrypting the base64 encoded initiator password with M-Pesa’s public key, a X509 certificate.
    /// Returns base64 encoded string.
    fn gen_security_credentials(&self) -> Result<String, Box<dyn Error>> {
        let pem = self.environment.get_certificate().as_bytes();
        let cert = X509::from_pem(pem).expect("error extracting X509 from pem");
        // getting the public and rsa keys
        let pub_key = cert.public_key().expect("error getting public key");
        let rsa_key = pub_key.rsa().expect("error getting rsa key from pub_key");
        // configuring the buffer
        let buf_len = pub_key.size();
        let mut buffer = vec![0; buf_len];

        rsa_key.public_encrypt(
            self.initiator_password.as_bytes(),
            &mut buffer,
            Padding::PKCS1,
        )?;
        Ok(encode(buffer))
    }

    /// # B2C API
    /// Sends b2c payment request.
    ///
    /// This API enables Business to Customer (B2C) transactions between a company and
    /// customers who are the end-users of its products or services. Use of this API requires a
    /// valid and verified B2C M-Pesa Short code.
    /// See more at: https://developer.safaricom.co.ke/docs?shell#b2c-api
    ///
    /// # Errors
    /// TODO
    pub fn b2c(
        &self,
        initiator_name: &str,
        command_id: CommandId,
        amount: u32,
        party_a: &str,
        party_b: &str,
        remarks: &str,
        queue_timeout_url: &str,
        result_url: &str,
        occasion: &str
    ) -> Result<B2cResponse, Box<dyn Error>> {
        let url = format!("{}/mpesa/b2c/v1/paymentrequest", self.environment.base_url());
        let credentials = self.gen_security_credentials()?;

        let payload = B2cPayload {
            initiator_name,
            security_credentials: &credentials,
            command_id,
            amount,
            party_a,
            party_b,
            remarks,
            queue_timeout_url,
            result_url,
            occasion,
        };

        let data = json!({
            "InitiatorName": payload.initiator_name,
            "SecurityCredential": payload.security_credentials,
            "CommandID": payload.command_id.get_command_id_str(),
            "Amount": payload.amount,
            "PartyA": payload.party_a,
            "PartyB": payload.party_b,
            "Remarks": payload.remarks,
            "QueueTimeOutURL": payload.queue_timeout_url,
            "ResultURL": payload.result_url,
            "Occasion": payload.occasion,
        });

        let response: B2cResponse = Client::new().post(&url)
            .bearer_auth(self.auth().unwrap())
            .json(&data)
            .send()?
            .json()?;

        Ok(response)
    }

    /// # B2B API
    /// Sends b2b payment request.
    ///
    /// This API enables Business to Business (B2B) transactions between a business and another
    /// business. Use of this API requires a valid and verified B2B M-Pesa short code for the
    /// business initiating the transaction and the both businesses involved in the transaction
    /// See more at https://developer.safaricom.co.ke/docs?shell#b2b-api
    ///
    /// # Errors
    /// TODO
    pub fn b2b(
        &self,
        initiator_name: &str,
        command_id: CommandId,
        amount: u32,
        party_a: &str,
        sender_id: &str,
        party_b: &str,
        receiver_id: &str,
        remarks: &str,
        queue_timeout_url: &str,
        result_url: &str,
        account_ref: &str,
    ) -> Result<HashMap<String,String>,Box<dyn Error>> {
        let url = format!("{}/mpesa/b2b/v1/paymentrequest", self.environment.base_url());
        let credentials = self.gen_security_credentials()?;

        let data = json!({});
        unimplemented!()
    }
}
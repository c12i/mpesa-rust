use std::collections::HashMap;
use std::error::Error;
use reqwest::blocking::Client;

use super::utils::extract_auth_token;

#[derive(Debug)]
pub enum Environment {
    Production,
    Sandbox,
}

#[derive(Debug)]
pub struct Mpesa {
    client_key: String,
    client_secret: String,
    base_url: String 
}

impl Mpesa {
    pub fn new(client_key: String, client_secret: String, environemt: Environment) -> Mpesa {
        let base_url = match environemt {
            Environment::Production => String::from("https://api.safaricom.co.ke"),
            Environment::Sandbox => String::from("https://sandbox.safaricom.co.ke"),
        };

        Mpesa {
            client_key,
            client_secret,
            base_url,
        }
    }

    pub fn auth(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/oauth/v1/generate?grant_type=client_credentials", &self.base_url);

        let resp: HashMap<String, String> = Client::new().get(&url)
            .basic_auth(&self.client_key, Some(&self.client_secret))
            .send()?
            .json()?;
        
        Ok(extract_auth_token(&resp)?)
    }
}
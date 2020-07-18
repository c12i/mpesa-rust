///! # environment
///! Code related to setting up the desired Safaricom API environment


/// Enum to match to either the production or sandbox base url
/// Use when instantiating the `Mpesa` struct.\

use std::str::FromStr;

#[derive(Debug)]
pub enum Environment {
    Production,
    Sandbox,
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "production" => Ok(Self::Production),
            "sandbox" => Ok(Self::Sandbox),
            _ => Err(String::from("error"))
        }
    }
}

impl Environment {
    /// Matches to intended base_url depending on Environment variant
    /// 
    /// ## Example
    /// ```
    /// use mpesa::Environment;
    /// 
    /// let env: Environment = Environment::Production;
    /// let base_url: &str = env.base_url();
    /// assert_eq!("https://api.safaricom.co.ke", base_url);
    /// ```
    pub fn base_url(&self) -> &'static str{
        match self {
            Environment::Production => "https://api.safaricom.co.ke",
            Environment::Sandbox => "https://sandbox.safaricom.co.ke"
        }
    }
}
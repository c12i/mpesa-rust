///! # environment
///! Code related to setting up the desired Safaricom API environment


/// Enum to match to either the production or sandox base url
/// Use when instantiating the `Mpesa` struct.
#[derive(Debug)]
pub enum Environment {
    Production,
    Sandbox,
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
        let base_url = match self {
            Environment::Production => "https://api.safaricom.co.ke",
            Environment::Sandbox => "https://sandbox.safaricom.co.ke"
        };
        base_url
    }
}
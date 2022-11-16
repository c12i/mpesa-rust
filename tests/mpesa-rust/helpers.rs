use mpesa::{ApiEnvironment, Mpesa};

pub struct TestEnvironment;

// TODO: Implement mock server for testing
impl ApiEnvironment for TestEnvironment {
    fn base_url(&self) -> &'static str {
        let _client = Mpesa::new("foo".to_string(), "bar".to_string(), TestEnvironment);
        "https://mock_server_url.com"
    }

    fn get_certificate(&self) -> &'static str {
        include_str!("../../src/certificates/sandbox")
    }
}

#[macro_export]
macro_rules! get_mpesa_client {
    () => {{
        use std::str::FromStr;
        dotenv::dotenv().ok();
        mpesa::Mpesa::new(
            std::env::var("CLIENT_KEY").unwrap(),
            std::env::var("CLIENT_SECRET").unwrap(),
            mpesa::Environment::from_str("sandbox").unwrap(),
        )
    }};

    ($client_key:expr, $client_secret:expr) => {{
        use std::str::FromStr;
        dotenv::dotenv().ok();
        mpesa::Mpesa::new(
            $client_key,
            $client_secret,
            mpesa::Environment::from_str("sandbox").unwrap(),
        )
    }};

    ($client_key:expr, $client_secret:expr, $environment:expr) => {{
        use std::str::FromStr;
        dotenv::dotenv().ok();
        mpesa::Mpesa::new(
            $client_key,
            $client_secret,
            mpesa::Environment::from_str($environment).unwrap(),
        )
    }};
}

#[cfg(test)]
mod tests {
    use crate::get_mpesa_client;

    #[tokio::test]
    async fn test_client_is_created_successfuly_with_correct_credentials() {
        let client = get_mpesa_client!();
        assert!(client.is_connected().await);
    }

    #[tokio::test]
    async fn test_client_will_not_authenticate_with_wrong_credentials() {
        let client = get_mpesa_client!(
            "not a client key".to_string(),
            "not a client secret".to_string()
        );
        assert!(!client.is_connected().await);
    }

    #[tokio::test]
    async fn test_client_will_not_authenticate_with_sandbox_credentials_in_production() {
        let client = get_mpesa_client!(
            std::env::var("CLIENT_KEY").unwrap(),
            std::env::var("CLIENT_SECRET").unwrap(),
            "production"
        );
        assert!(!client.is_connected().await);
    }
}

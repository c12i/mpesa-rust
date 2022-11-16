use mpesa::ApiEnvironment;
use wiremock::MockServer;

pub struct TestEnvironment {
    pub server: MockServer,
    pub server_url: String,
}

impl TestEnvironment {
    #[allow(unused)]
    pub async fn new() -> Self {
        let mock_server = MockServer::start().await;
        TestEnvironment {
            server_url: mock_server.uri(),
            server: mock_server,
        }
    }
}

// TODO: Implement mock server for testing
impl ApiEnvironment for TestEnvironment {
    fn base_url(&self) -> &str {
        &self.server_url
    }

    fn get_certificate(&self) -> &str {
        include_str!("../../src/certificates/sandbox")
    }
}

#[macro_export]
macro_rules! get_mpesa_client {
    () => {{
        use mpesa::{Environment, Mpesa};
        use std::str::FromStr;
        dotenv::dotenv().ok();
        let client = Mpesa::new(
            std::env::var("CLIENT_KEY").unwrap(),
            std::env::var("CLIENT_SECRET").unwrap(),
            Environment::from_str("sandbox").unwrap(),
        );
        client
    }};

    ($client_key:expr, $client_secret:expr) => {{
        use mpesa::{Environment, Mpesa};
        use std::str::FromStr;
        dotenv::dotenv().ok();
        let client = Mpesa::new(
            $client_key,
            $client_secret,
            Environment::from_str("sandbox").unwrap(),
        );
        client
    }};

    ($client_key:expr, $client_secret:expr, $environment:expr) => {{
        use mpesa::{Environment, Mpesa};
        use std::str::FromStr;
        dotenv::dotenv().ok();
        let client = Mpesa::new(
            $client_key,
            $client_secret,
            Environment::from_str($environment).unwrap(),
        );

        client
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

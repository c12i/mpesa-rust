use mpesa::ApiEnvironment;
use wiremock::MockServer;

pub struct TestEnvironment {
    pub server_url: String,
}

impl TestEnvironment {
    #[allow(unused)]
    pub async fn new(server: &MockServer) -> Self {
        TestEnvironment {
            server_url: server.uri(),
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
        use crate::helpers::TestEnvironment;
        use mpesa::Mpesa;
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use serde_json::json;
        use wiremock::matchers::{path, query_param, method};

        dotenv::dotenv().ok();
        let server = MockServer::start().await;
        let test_environment = TestEnvironment::new(&server).await;
        let client = Mpesa::new(
            std::env::var("CLIENT_KEY").unwrap(),
            std::env::var("CLIENT_SECRET").unwrap(),
            test_environment,
        );
        Mock::given(method("GET"))
            .and(path("/oauth/v1/generate"))
            .and(query_param("grant_type", "client_credentials"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "dummy_access_token"
            })))
            .expect(1)
            .mount(&server)
            .await;
        (client, server)
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
        let client = Mpesa::new($client_key, $client_secret, $environment);
        client
    }};
}

#[cfg(test)]
mod tests {
    use crate::get_mpesa_client;

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
            Environment::from_str("production").unwrap()
        );
        assert!(!client.is_connected().await);
    }
}

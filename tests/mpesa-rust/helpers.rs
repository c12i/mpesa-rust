use mpesa::ApiEnvironment;
use wiremock::MockServer;

#[derive(Debug, Clone)]
pub struct TestEnvironment {
    pub server_url: String,
}

impl TestEnvironment {
    pub async fn new(server: &MockServer) -> Self {
        TestEnvironment {
            server_url: server.uri(),
        }
    }
}

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
        use $crate::helpers::TestEnvironment;
        use mpesa::Mpesa;
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use serde_json::json;
        use wiremock::matchers::{path, query_param, method};

        dotenvy::dotenv().ok();
        let server = MockServer::start().await;
        let test_environment = TestEnvironment::new(&server).await;
        let client = Mpesa::new(
            dotenvy::var("CONSUMER_KEY").unwrap(),
            dotenvy::var("CONSUMER_SECRET").unwrap(),
            test_environment,
        );
        Mock::given(method("GET"))
            .and(path("/oauth/v1/generate"))
            .and(query_param("grant_type", "client_credentials"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "dummy_access_token",
                "expires_in": "3600"
            })))
            .mount(&server)
            .await;
        (client, server)
    }};

    (expected_auth_requests = $expected_requests: expr) => {{
        use $crate::helpers::TestEnvironment;
        use mpesa::Mpesa;
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use serde_json::json;
        use wiremock::matchers::{path, query_param, method};

        dotenvy::dotenv().ok();
        let server = MockServer::start().await;
        let test_environment = TestEnvironment::new(&server).await;
        let client = Mpesa::new(
            dotenvy::var("CONSUMER_KEY").unwrap(),
            dotenvy::var("CONSUMER_SECRET").unwrap(),
            test_environment,
        );
        Mock::given(method("GET"))
            .and(path("/oauth/v1/generate"))
            .and(query_param("grant_type", "client_credentials"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "dummy_access_token",
                "expires_in": "3600"
            })))
            .mount(&server)
            .await;
        (client, server)
    }};

    ($consumer_key:expr, $consumer_secret:expr) => {{
        use mpesa::{Environment, Mpesa};
        use std::str::FromStr;
        dotenvy::dotenv().ok();
        let client = Mpesa::new(
            $consumer_key,
            $consumer_secret,
            Environment::from_str("sandbox").unwrap(),
        );
        client
    }};

    ($consumer_key:expr, $consumer_secret:expr, $environment:expr) => {{
        use mpesa::{Environment, Mpesa};
        use std::str::FromStr;
        dotenvy::dotenv().ok();
        let client = Mpesa::new($consumer_key, $consumer_secret, $environment);
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
}

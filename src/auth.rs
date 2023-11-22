use cached::proc_macro::cached;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::{ApiEnvironment, Mpesa, MpesaError, MpesaResult, ResponseError};

const AUTHENTICATION_URL: &str = "/oauth/v1/generate?grant_type=client_credentials";

#[cached(
    size = 1,
    time = 3600,
    key = "String",
    result = true,
    convert = r#"{ format!("{}", client.client_key()) }"#
)]
pub(crate) async fn auth(client: &Mpesa<impl ApiEnvironment>) -> MpesaResult<String> {
    let url = format!("{}{}", client.environment.base_url(), AUTHENTICATION_URL);

    let response = client
        .http_client
        .get(&url)
        .basic_auth(client.client_key(), Some(&client.client_secret()))
        .send()
        .await?;

    if response.status().is_success() {
        let value = response.json::<AuthenticationResponse>().await?;
        let access_token = value.access_token;

        return Ok(access_token);
    }

    let error = response.json::<ResponseError>().await?;
    Err(MpesaError::Service(error))
}

/// Response returned from the authentication function
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    /// Access token which is used as the Bearer-Auth-Token
    pub access_token: String,
    /// Expiry time in seconds
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub expires_in: u64,
}

impl std::fmt::Display for AuthenticationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "token :{} expires in: {}",
            self.access_token, self.expires_in
        )
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer};

    use super::*;

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
            include_str!("../src/certificates/sandbox")
        }
    }

    #[tokio::test]
    async fn test_cached_auth() {
        use cached::Cached;

        use crate::Mpesa;

        let server = MockServer::start().await;

        let env = TestEnvironment::new(&server).await;

        let client = Mpesa::new("test_api_key", "test_public_key", env);

        Mock::given(wiremock::matchers::method("GET"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(
                AuthenticationResponse {
                    access_token: "test_token".to_string(),
                    expires_in: 3600,
                },
            ))
            .expect(1)
            .mount(&server)
            .await;

        auth_prime_cache(&client).await.unwrap();

        let mut cache = AUTH.lock().await;

        assert!(cache.cache_get(&client.client_key().to_string()).is_some());
        assert_eq!(cache.cache_hits().unwrap(), 1);
        assert_eq!(cache.cache_capacity().unwrap(), 1);
    }
}

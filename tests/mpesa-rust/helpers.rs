#[macro_export]
macro_rules! get_mpesa_client {
    () => {{
        dotenv::dotenv().ok();
        mpesa::Mpesa::new(
            std::env::var("CLIENT_KEY").unwrap(),
            std::env::var("CLIENT_SECRET").unwrap(),
            "sandbox".parse().unwrap(),
        )
    }};

    ($client_key:expr, $client_secret:expr) => {{
        dotenv::dotenv().ok();
        mpesa::Mpesa::new($client_key, $client_secret, "sandbox".parse().unwrap())
    }};

    ($client_key:expr, $client_secret:expr, $environment:expr) => {{
        dotenv::dotenv().ok();
        mpesa::Mpesa::new($client_key, $client_secret, $environment)
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
            "production".parse().unwrap()
        );
        assert!(!client.is_connected().await);
    }
}

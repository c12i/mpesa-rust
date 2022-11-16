use crate::get_mpesa_client;

#[tokio::test]
#[ignore = "c2b_register always fails on sandbox with status 503"]
async fn c2b_register_test() {
    let client = get_mpesa_client!();

    let response = client
        .c2b_register()
        .short_code("600496")
        .confirmation_url("https://testdomain.com/true")
        .validation_url("https://testdomain.com/valid")
        .send()
        .await;

    assert!(response.is_ok())
}

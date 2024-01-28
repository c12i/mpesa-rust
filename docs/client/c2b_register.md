Register URL API works hand in hand with Customer to Business (C2B) APIs and allows receiving payment notifications to your paybill. This API enables you to register the callback URLs via which you shall receive notifications for payments to your pay bill/till number.

There are two URLs required for Register URL API: Validation URL and Confirmation URL.

Returns a `C2bRegisterBuilder`

See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/APIs/CustomerToBusinessRegisterURL)

# Example

```rust,no_run
use mpesa::{Mpesa, Environment};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env!("CONSUMER_KEY"),
        env!("CONSUMER_SECRET"),
        Environment::Sandbox,
    );

    let response = client
        .c2b_register()
        .short_code("600496")
        .confirmation_url("https://testdomain.com/true")
        .validation_url("https://testdomain.com/valid")
        .response_type(mpesa::ResponseType::Completed) // optional, defaults to `ResponseTypes::Complete`
        .send()
        .await;

    assert!(response.is_ok())
}
```

# Onboard

Creates a `OnboardBuilder` which allows you to opt in as a biller to the bill manager features.

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/BillManager)

## Example

```rust,ignore
use mpesa::{Mpesa, Environment, SendRemindersTypes};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("env variables not found");

    let client = Mpesa::new(
        dotenvy::var("CLIENT_KEY").unwrap(),
        dotenvy::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .onboard()
        .callback_url("https://testdomain.com/true")
        .email("email@test.com")
        .logo("https://file.domain/file.png")
        .official_contact("0712345678")
        .send_reminders(SendRemindersTypes::Enable)
        .short_code("718003")
        .send()
        .await;

    assert!(response.is_ok());
}
```

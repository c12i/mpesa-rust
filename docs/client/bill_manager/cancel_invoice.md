# Cancel Invoice

Creates a `CancelInvoiceBuilder` which allows you to recall a sent invoice.

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/BillManager)

## Example

```rust,ignore
use mpesa::{Mpesa, Environment, SendRemindersTypes};
use chrono::prelude::Utc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client = Mpesa::new(
        dotenvy::var("CONSUMER_KEY").unwrap(),
        dotenvy::var("CONSUMER_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .cancel_invoice()
        .external_references(vec!["9KLSS011"])
        .send()
        .await;

    assert!(response.is_ok());
}
```

Creates a `CancelBulkInvoicesBuilder` which allows you to recall a list of sent invoices.

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/BillManager)

# Example
```rust,ignore
use mpesa::{Mpesa, Environment, SendRemindersTypes};
use chrono::prelude::Utc;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env!("CLIENT_KEY"),
        env!("CLIENT_SECRET"),
        Environment::Sandbox,
    );

    let response = client
        .cancel_bulk_invoice()
        .external_references(vec!["9KLSS011"])
        .send()
        .await;

    assert!(response.is_ok());
}
```

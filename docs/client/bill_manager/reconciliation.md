# Reconciliation

Creates a `ReconciliationBuilder` which enables your customers to receive e-receipts for payments made to your paybill account.

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/BillManager)

## Example

```rust,ignore
use mpesa::{Mpesa, Environment};
use chrono::prelude::Utc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("env variables not found");

    let client = Mpesa::new(
        std::env::var("CLIENT_KEY").unwrap(),
        std::env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .reconciliation()
        .account_reference("John Doe")
        .external_reference("INV2345")
        .full_name("John Doe")
        .invoice_name("Invoice 001")
        .paid_amount(1000.0)
        .payment_date(Utc::now())
        .phone_number("0712345678")
        .transaction_id("TRANSACTION_ID")
        .send()
        .await;

    assert!(response.is_ok());
}
```

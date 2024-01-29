# Dynamic QR

Generates a QR code that can be scanned by a M-Pesa customer to make
payments.

Returns a `DynamicQRBuilder`

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/DynamicQRCode)

## Example

```rust
use mpesa::{Mpesa, Environment};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client = Mpesa::new(
        dotenvy::var("CONSUMER_KEY").unwrap(),
        dotenvy::var("CONSUMER_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .dynamic_qr()
        .amount(2000)
        .credit_party_identifier("373132")
        .merchant_name("TEST SUPERMARKET")
        .ref_no("Invoice Test")
        .size("300")
        .transaction_type(mpesa::TransactionType::BG)
        .build()
        .unwrap()
        .send()
        .await;

    assert!(response.is_ok())
}
```

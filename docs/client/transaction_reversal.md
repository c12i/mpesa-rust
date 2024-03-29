# Transaction Reversal

## Reverses a C2B M-Pesa transaction

Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
Returns a `TransactionReversalBuilder`

See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/APIs/Reversal)

## Example

```rust
use mpesa::{Mpesa, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Mpesa::new(
        dotenvy::var("CONSUMER_KEY").unwrap(),
        dotenvy::var("CONSUMER_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .transaction_reversal()
        .initiator("testapi496")
        .try_result_url("https://testdomain.com/ok")?
        .try_timeout_url("https://testdomain.com/err")?
        .transaction_id("OEI2AK4Q16")
        .receiver_identifier_type(mpesa::IdentifierTypes::Reversal)
        .remarks("test")
        .amount(100)
        .receiver_party("600111")
        .build()?
        .send()
        .await;

    assert!(response.is_ok());

    Ok(())
}
```

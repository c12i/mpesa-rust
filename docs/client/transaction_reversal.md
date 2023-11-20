Reverses a C2B M-Pesa transaction.

Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
Returns a `TransactionReversalBuilder`

See more from the Safaricom API docs [here](https://developer.safaricom.co.ke/Documentation)

# Example
```rust
use mpesa::{Mpesa, Environment};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env!("CLIENT_KEY"),
        env!("CLIENT_SECRET"),
        Environment::Sandbox,
    );

    let response = client
    .transaction_reversal("testapi496")
    .result_url("https://testdomain.com/ok")
    .timeout_url("https://testdomain.com/err")
    .transaction_id("OEI2AK4Q16")
        .command_id(mpesa::CommandId::TransactionReversal) // optional will default to CommandId::TransactionReversal
    .receiver_identifier_type(mpesa::IdentifierTypes::Reversal) // optional will default to IdentifierTypes::Reversal
    .amount(100)
    .receiver_party("600111")
    .send()
    .await;

    assert!(response.is_ok())
}
```

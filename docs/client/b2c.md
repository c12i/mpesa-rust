# B2C

Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
Returns a `B2cBuilder` for building a B2C transaction struct.

Safaricom the API docs [reference](https://developer.safaricom.co.ke/APIs/BusinessToCustomer).

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
        .b2c("testapi496")
        .party_a("600496")
        .party_b("254708374149")
        .result_url("https://testdomain.com/err")
        .timeout_url("https://testdomain.com/ok")
        .amount(1000)
        .remarks("Your Remark") // optional, defaults to "None"
        .occasion("Your Occasion") // optional, defaults to "None"
        .command_id(mpesa::CommandId::BusinessPayment) // optional, defaults to `CommandId::BusinessPayment`
        .send()
        .await;
    assert!(response.is_ok())
}
```

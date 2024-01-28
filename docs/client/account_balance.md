The Account Balance API is used to request the account balance of a short code. This can be used for both B2C, buy goods and pay bill accounts.

Requires an `initiator_name`.
Returns an `AccountBalanceBuilder` for enquiring the balance on an MPESA BuyGoods.

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/AccountBalance)

# Example

```rust
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
        .account_balance("testapi496")
        .result_url("https://testdomain.com/err")
        .timeout_url("https://testdomain.com/ok")
        .party_a("600496")
        .command_id(mpesa::CommandId::AccountBalance) // optional, defaults to `CommandId::AccountBalance`
        .identifier_type(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
        .remarks("Your Remarks") // optional, defaults to "None"
        .send()
        .await;

    assert!(response.is_ok())
}
```

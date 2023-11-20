Queries the status of a B2B, B2C or C2B M-Pesa transaction.

Requires an `initiator_name`, the credential/ username used to authenticate the transaction request
Returns a `TransactionStatusBuilder`.

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
	    .transaction_status("testapi496")
	    .transaction_id("OEI2AK4Q16")
	    .party_a("600496")
	    .identifier_type(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
        .remarks("Your Remarks") // optional, defaults to "None"
        .result_url("https://testdomain.com/err")
        .timeout_url("https://testdomain.com/ok")
        .send()
        .await;

    assert!(response.is_ok())
}
```

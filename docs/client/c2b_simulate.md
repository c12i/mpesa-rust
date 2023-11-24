Creates a `C2bSimulateBuilder` for simulating C2B transactions

See more [here](https://developer.safaricom.co.ke/c2b/apis/post/simulate)

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

	let response = client.c2b_simulate()
		.short_code("600496")
		.msisdn("254700000000")
		.amount(1000)
		.command_id(mpesa::CommandId::CustomerPayBillOnline) // optional, defaults to `CommandId::CustomerPayBillOnline`
		.bill_ref_number("Your_BillRefNumber") // optional, defaults to "None"
		.send()
		.await;

	assert!(response.is_ok())
}
```

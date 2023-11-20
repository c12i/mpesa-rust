Generates a QR code that can be scanned by a M-Pesa customer to make
payments.

Returns a `DynamicQRBuilder`

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/DynamicQRCode)

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
		.dynamic_qr()
		.amount(1000)
		.ref_no("John Doe")
		.size("300")
		.merchant_name("John Doe")
		.credit_party_identifier("600496")
		.try_transaction_type("bg")
		.unwrap()
		.build()
		.unwrap()
		.send()
		.await;

	println!("{:?}", response);
	assert!(response.is_ok())
}
```
# Express Request

Lipa na M-PESA online API also known as M-PESA express (STK Push/NI push) is a Merchant/Business initiated C2B (Customer to Business) Payment.

Once you, our merchant integrate with the API, you will be able to send a payment prompt on the customer's phone (Popularly known as STK Push Prompt) to your customer's M-PESA registered phone number requesting them to enter their M-PESA pin to authorize and complete payment.

Requires a `business_short_code` - The organization shortcode used to receive the transaction and
returns a `MpesaExpressRequestBuilder` struct

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/MpesaExpressSimulate)

## Example

```rust
use mpesa::{Mpesa, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenvy::dotenv().expect("env variables not found");

    let client = Mpesa::new(
        std::env::var("CLIENT_KEY").unwrap(),
        std::env::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client
        .express_request()
        .business_short_code("174379")
        .phone_number("254708374149")
        .party_a("254708374149")
        .party_b("174379")
        .amount(500)
        .try_callback_url("https://test.example.com/api")?
        .transaction_type(mpesa::CommandId::CustomerPayBillOnline) // Optional, defaults to `CommandId::CustomerPayBillOnline`
        .transaction_desc("Description") // Optional, defaults to "None"
        .build()?
        .send()
        .await;

    assert!(response.is_ok());

    Ok(())
}
```

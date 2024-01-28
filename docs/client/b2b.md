# B2B

This API enables you to pay bills directly from your business account to a pay bill number, or a paybill store. You can use this API to pay on behalf of a consumer/requester.

The transaction moves money from your MMF/Working account to the recipient’s utility account.

Returns a `B2bBuilder`.
Requires an `initiator_name`, the credential/ username used to authenticate the transaction request

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/BusinessPayBill)

## Example

```rust
use mpesa::{Mpesa, Environment};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client = Mpesa::new(
        dotenvy::var("CLIENT_KEY").unwrap(),
        dotenvy::var("CLIENT_SECRET").unwrap(),
        Environment::Sandbox,
    );

    let response = client.b2b("testapi496")
        .party_a("600496")
        .party_b("600000")
        .result_url("https://testdomain.com/err")
        .timeout_url("https://testdomain.com/ok")
        .account_ref("254708374149")
        .amount(1000)
        .command_id(mpesa::CommandId::BusinessToBusinessTransfer) // optional, defaults to `CommandId::BusinessToBusinessTransfer`
        .remarks("None") // optional, defaults to "None"
        .sender_id(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
        .receiver_id(mpesa::IdentifierTypes::ShortCode) // optional, defaults to `IdentifierTypes::ShortCode`
        .send()
        .await;

    assert!(response.is_ok());
}
```

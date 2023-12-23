Creates a `SingleInvoiceBuilder` which allows you to create and send invoices to your customers.

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/BillManager)

# Example
```rust,ignore
use mpesa::{Mpesa, Environment, InvoiceItem};
use chrono::prelude::Utc;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env!("CLIENT_KEY"),
        env!("CLIENT_SECRET"),
        Environment::Sandbox,
    );

    let response = client
        .single_invoice()
        .amount(1000.0)
        .account_reference("John Doe")
        .billed_full_name("John Doe")
        .billed_period("August 2021")
        .billed_phone_number("0712345678")
        .due_date(Utc::now())
        .external_reference("INV2345")
        .invoice_items(vec![
            InvoiceItem {amount: 1000.0, item_name: "An item"}
        ])
        .invoice_name("Invoice 001")
        .build()
        .unwrap()
        .send()
        .await;

    assert!(response.is_ok());
}
```

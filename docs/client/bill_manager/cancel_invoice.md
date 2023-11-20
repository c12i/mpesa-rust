Creates a `CancelInvoiceBuilder` which allows you to recall a sent invoice.

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/BillManager)

# Example
```rust,ignore
use chrono::prelude::Utc;

let response = client
    .cancel_invoice()
    .external_references(vec!["9KLSS011"])
    .send()
    .await;
```

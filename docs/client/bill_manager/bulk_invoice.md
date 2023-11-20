Creates a `BulkInvoiceBuilder` which allows you to send invoices to your customers in bulk.

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/BillManager)

# Example
```rust,ignore
use chrone::prelude::Utc;

let response = client
    .bulk_invoice()

    // Add multiple invoices at once
    .invoices(vec![
        Invoice {
            amount: 1000.0,
            account_reference: "John Doe",
            billed_full_name: "John Doe",
            billed_period: "August 2021",
            billed_phone_number: "0712345678",
            due_date: Utc::now(),
            external_reference: "INV2345",
            invoice_items: Some(
                vec![InvoiceItem {amount: 1000.0, item_name: "An item"}]
            ),
            invoice_name: "Invoice 001"
        }
    ])

    // Add a single invoice
    .invoice(
        Invoice {
            amount: 1000.0,
            account_reference: "John Doe",
            billed_full_name: "John Doe",
            billed_period: "August 2021",
            billed_phone_number: "0712345678",
            due_date: Utc::now(),
            external_reference: "INV2345",
            invoice_items: Some(vec![InvoiceItem {
                amount: 1000.0,
                item_name: "An item",
            }]),
            invoice_name: "Invoice 001",
        }
    )
    .send()
    .await;
```

Creates a `OnboardBuilder` which allows you to opt in as a biller to the bill manager features.

Safaricom API docs [reference](https://developer.safaricom.co.ke/APIs/BillManager)

# Example
```rust,ignore
let response = client
    .onboard()
    .callback_url("https://testdomain.com/true")
    .email("email@test.com")
    .logo("https://file.domain/file.png")
    .official_contact("0712345678")
    .send_reminders(SendRemindersTypes::Enable)
    .short_code("600496")
    .send()
    .await;
```

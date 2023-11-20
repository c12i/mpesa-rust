Optional in development but required for production, you will need to call this method and set your production initiator password.
If in development, default initiator password is already pre-set

```rust
use mpesa::{Mpesa, Environment};

fn main() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env!("CLIENT_KEY"),
        env!("CLIENT_SECRET"),
        Environment::Sandbox,
    );
    client.set_initiator_password("your_initiator_password");
}
```

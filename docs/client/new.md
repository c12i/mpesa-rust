Constructs a new `Mpesa` client.

# Example
```rust
use mpesa::{Mpesa, Environment};

fn main() {
    dotenv::dotenv().ok();

    let client = Mpesa::new(
        env!("CLIENT_KEY"),
        env!("CLIENT_SECRET"),
        Environment::Sandbox,
    );
}
```

 # Panics
 This method can panic if a TLS backend cannot be initialized for the internal http_client

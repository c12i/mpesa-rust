# mpesa-rust

[![Rust](https://github.com/collinsmuriuki/mpesa-rust/actions/workflows/general.yml/badge.svg)](https://github.com/collinsmuriuki/mpesa-rust/actions/workflows/general.yml)
[![](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![](https://img.shields.io/crates/v/mpesa)](https://crates.io/crates/mpesa)

## About

An unofficial Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.

## Install

`Cargo.toml`

```toml
[dependencies]
mpesa = { git = "https://github.com/collinsmuriuki/mpesa-rust" }
```

Optionally, you can disable default-features, which is basically the entire suite of MPESA APIs to conditionally select from either:

- `b2b`
- `b2c`
- `account_balance`
- `c2b_register`
- `c2b_simulate`
- `express_request`
- `transaction_reversal`
- `transaction_status`

Example:

```toml
[dependencies]
mpesa = { git = "https://github.com/collinsmuriuki/mpesa-rust", default_features = false, features = ["b2b", "express_request"] }
```

In your lib or binary crate:

```rust,no_run
use mpesa::Mpesa;
```

## Usage

### Creating a `Mpesa` client

You will first need to create an instance of the `Mpesa` instance (the client). You are required to provide a **CLIENT_KEY** and
**CLIENT_SECRET**. [Here](https://developer.safaricom.co.ke/test_credentials) is how you can get these credentials for the Safaricom sandbox
environment. It's worth noting that these credentials are only valid in the sandbox environment. To go live and get production keys
read the docs [here](https://developer.safaricom.co.ke/docs?javascript#going-live).

These are the following ways you can instantiate `Mpesa`:

```rust,no_run
use mpesa::{Mpesa, Environment};

let client = Mpesa::new(
      env!("CLIENT_KEY"),
      env!("CLIENT_SECRET"),
      Environment::Sandbox,
);

assert!(client.is_connected().await)
```

Since the `Environment` enum implements `FromStr` and `TryFrom` for `String` and `&str` types, you can call `Environment::from_str` or `Environment::try_from` to create an `Environment` type. This is ideal if the environment values are
stored in a `.env` or any other configuration file:

```rust,no_run
use mpesa::{Mpesa, Environment};
use std::str::FromStr;
use std::convert::TryFrom;

let client0 = Mpesa::new(
      env!("CLIENT_KEY"),
      env!("CLIENT_SECRET"),
      Environment::from_str("sandbox")? // "Sandbox" and "SANDBOX" also valid
);
assert!(client0.is_connected().await)

let client1 = Mpesa::new(
      env!("CLIENT_KEY"),
      env!("CLIENT_SECRET"),
      Environment::try_from("production")? // "Production" and "PRODUCTION" also valid
);
assert!(client1.is_connected().await)
```

The `Mpesa` struct's `environment` parameter is generic over any type that implements the `ApiEnvironment` trait. This trait
expects the following methods to be implemented for a given type:

```rust
pub trait ApiEnvironment {
    fn base_url(&self) -> &str;
    fn get_certificate(&self) -> &str;
}
```

This trait allows you to create your own type to pass to the `environment` parameter. With this in place, you are able to mock http requests (for testing purposes) from the MPESA api by returning a mock server uri from the `base_url` method as well as using your own certificates, required to sign select requests to the MPESA api, by providing your own `get_certificate` implementation.

See the example below (and [here](./src/environment.rs) so see how the trait is implemented for the `Environment` enum):

```rust,no_run
use mpesa::{Mpesa, ApiEnvironment};
use std::str::FromStr;
use std::convert::TryFrom;

pub struct MyCustomEnvironment;

impl ApiEnvironment for MyCustomEnvironment {
    fn base_url(&self) -> &str {
        // your base url here
        "https://your_base_url.com"
    }

    fn get_certificate(&self) -> &str {
        // your certificate here
        r#"..."#
    }
}

let client: Mpesa<MyCustomEnvironment> = Mpesa::new(
    env!("CLIENT_KEY"),
    env!("CLIENT_SECRET"),
    MyCustomEnvironment // ✔ valid
);

//...
```

If you intend to use in production, you will need to call a the `set_initiator_password` method from `Mpesa` after initially
creating the client. Here you provide your initiator password, which overrides the default password used in sandbox `"Safcom496!"`:

```rust,no_run
use mpesa::Mpesa;

let client = Mpesa::new(
      env!("CLIENT_KEY"),
      env!("CLIENT_SECRET"),
      Environment::Sandbox,
);

client.set_initiator_password("new_password");

assert!(client.is_connected().await)
```

### Services

The following services are currently available from the `Mpesa` client as methods that return builders:

- B2C

```rust,no_run
let response = client
    .b2c("testapi496")
    .party_a("600496")
    .party_b("254708374149")
    .result_url("https://testdomain.com/ok")
    .timeout_url("https://testdomain.com/err")
    .amount(1000)
    .send()
    .await;
assert!(response.is_ok())
```

- B2B

```rust,no_run
let response = client
    .b2b("testapi496")
    .party_a("600496")
    .party_b("254708374149")
    .result_url("https://testdomain.com/ok")
    .timeout_url("https://testdomain.com/err")
    .account_ref("254708374149")
    .amount(1000)
    .send()
    .await;
assert!(response.is_ok())
```

- C2B Register

```rust,no_run
let response = client
    .c2b_register()
    .short_code("600496")
    .confirmation_url("https://testdomain.com/true")
    .validation_url("https://testdomain.com/valid")
    .send()
    .await;
assert!(response.is_ok())
```

- C2B Simulate

```rust,no_run
let response = client
    .c2b_simulate()
    .short_code("600496")
    .msisdn("254700000000")
    .amount(1000)
    .send()
    .await;
assert!(response.is_ok())
```

- Account Balance

```rust,no_run
let response = client
    .account_balance("testapi496")
    .result_url("https://testdomain.com/ok")
    .timeout_url("https://testdomain.com/err")
    .party_a("600496")
    .send()
    .await;
assert!(response.is_ok())
```

- Mpesa Express Request / STK push / Lipa na M-PESA online

```rust,no_run
let response = client
    .express_request("174379")
    .phone_number("254708374149")
    .amount(500)
    .callback_url("https://test.example.com/api")
    .send()
    .await;
assert!(response.is_ok())
```

- Transaction Reversal:

```rust,no_run
let response = client
    .transaction_reversal("testapi496")
    .result_url("https://testdomain.com/ok")
    .timeout_url("https://testdomain.com/err")
    .transaction_id("OEI2AK4Q16")
    .receiver_identifier_type(IdentifierTypes::ShortCode)
    .amount(100.0)
    .receiver_party("600111")
    .send()
    .await;
assert!(response.is_ok())
```

- Transaction Status

```rust,no_run
let response = client
    .transaction_status("testapi496")
    .result_url("https://testdomain.com/ok")
    .timeout_url("https://testdomain.com/err")
    .transaction_id("OEI2AK4Q16")
    .identifier_type(IdentifierTypes::ShortCode)
    .party_a("600111")
    .remarks("status")
    .occasion("work")
    .send()
    .await;
assert!(response.is_ok())
```

More will be added progressively, pull requests welcome

## Author

**Collins Muriuki**

- Twitter: [@collinsmuriuki\_](https://twitter.com/collinsmuriuki_)
- Not affiliated with Safaricom.

## Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check [issues page](https://github.com/collinsmuriuki/mpesa-rust/issues). You can also take a look at the [contributing guide](https://raw.githubusercontent.com/collinsmuriuki/mpesa-rust/master/CONTRIBUTING.md).

Copyright © 2023 [Collins Muriuki](https://github.com/collinsmuriuki).<br />
This project is [MIT](https://raw.githubusercontent.com/collinsmuriuki/mpesa-rust/master/LICENSE) licensed.

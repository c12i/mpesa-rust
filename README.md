# mpesa-rust

<p>
   <a href="https://crates.io/crates/mpesa" target="_blank">
     <img alt="Version" src="https://img.shields.io/crates/v/mpesa" />
   </a>
  <a href="https://docs.rs/mpesa" target="_blank">
    <img alt="Documentation" src="https://docs.rs/mpesa/badge.svg" />
  </a>
  <a href="https://travis-ci.com/collinsmuriuki/mpesa-rust" target="_blank">
      <img alt="mpesa travis-ci" src="https://travis-ci.com/collinsmuriuki/mpesa-rust.svg?branch=master" />
   </a>
  <a href="LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
</p>

## About

An unofficial Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.

## Notes

**Warning!** WIP, not recommended for use in production

## Install

`Cargo.toml`

```md
[dependencies]
mpesa = "0.2.4"
```

In your lib or binary crate:

```rs
use mpesa::Mpesa;
```

## Usage

### Creating a `Client`

You will first need to create an instance of the `Mpesa` instance (the client). You are required to provide a **CLIENT_KEY** and
**CLIENT_SECRET**. [Here](https://developer.safaricom.co.ke/test_credentials) is how you can get these credentials for the Safaricom sandbox
environment.

_NOTE_: only calling `unwrap` for demonstration purposes. Errors are handled appropriately in the lib via the `MpesaError` enum.

There are two ways you can instantiate `Mpesa`:

```rust
use mpesa::{Mpesa, Environment};
use std::env;

let client = Mpesa::new(
      env::var("CLIENT_KEY")?,
      env::var("CLIENT_SECRET")?,
      Environment::Sandbox,
);
assert!(client.is_connected().unwrap())
```

Since the `Environment` enum implements `FromStr`, you can pass the name of the environment as a `&str` and call the `parse()`
method to create an `Environment` type from the string slice:

```rust
use mpesa::Mpesa;
use std::env;

let client = Mpesa::new(
      env::var("CLIENT_KEY")?,
      env::var("CLIENT_SECRET")?,
      "sandbox".parse()?,
);
assert!(client.is_connected().unwrap())
```

### Services

The following services are currently available from the `Mpesa` client as methods that return builders:

- B2C

```rust
let response = client
    .b2c("testapi496")
    .parties("600496", "254708374149")
    .urls("https://testdomain.com/err", "https://testdomain.com/res")
    .amount(1000)
    .send();
assert!(response.is_ok())
```

- B2B

```rust
let response = client
    .b2b("testapi496")
    .parties("600496", "600000")
    .urls("https://testdomain.com/err", "https://testdomain.com/api")
    .account_ref("254708374149")
    .amount(1000)
    .send();
assert!(response.is_ok())
```

- C2B Register

```rust
let response = client
    .c2b_register()
    .short_code("600496")
    .confirmation_url("https://testdomain.com/true")
    .validation_url("https://testdomain.com/valid")
    .send();
assert!(response.is_ok())
```

- C2B Simulate

```rust

let response = client
    .c2b_simulate()
    .short_code("600496")
    .msisdn("254700000000")
    .amount(1000)
    .send();
assert!(response.is_ok())
```

- Account Balance

```rust
let response = client
    .account_balance("testapi496")
    .urls("https://testdomain.com/err", "https://testdomain.com/ok")
    .party_a("600496")
    .send();
assert!(response.is_ok())
```

More will be added progressively, pull requests welcome

## Author

**Collins Muriuki**

- Twitter: [@collinsmuriuki\_](https://twitter.com/collinsmuriuki_)
- Not affiliated with Safaricom in any way

## Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check [issues page](https://github.com/collinsmuriuki/mpesa-rust/issues). You can also take a look at the [contributing guide](CONTRIBUTING.md).

Copyright © 2020 [Collins Muriuki](https://github.com/collinsmuriuki).<br />
This project is [MIT](LICENSE) licensed.

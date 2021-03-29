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

## Install

`Cargo.toml`

```md
[dependencies]
mpesa = "0.3.3"
```

In your lib or binary crate:

```rs
use mpesa::Mpesa;
```

## Usage

### Creating a `Mpesa` client

You will first need to create an instance of the `Mpesa` instance (the client). You are required to provide a **CLIENT_KEY** and
**CLIENT_SECRET**. [Here](https://developer.safaricom.co.ke/test_credentials) is how you can get these credentials for the Safaricom sandbox
environment. It's worth noting that these credentials are only valid in the sandbox environment. To go live and get production keys
read the docs [here](https://developer.safaricom.co.ke/docs?javascript#going-live).

_NOTE_:

- Only calling `unwrap` for demonstration purposes. Errors are handled appropriately in the lib via the `MpesaError` enum.

These are the following ways you can instantiate `Mpesa`:

```rust
use mpesa::{Mpesa, Environment};
use std::env;

let client = Mpesa::new(
      env::var("CLIENT_KEY")?,
      env::var("CLIENT_SECRET")?,
      Environment::Sandbox,
);

assert!(client.is_connected())
```

Since the `Environment` enum implements `FromStr` and `TryFrom`, you can pass the name of the environment as a `&str` and call the `parse()` or `try_into()`
method to create an `Environment` type from the string slice (Pascal case and Uppercase string slices also valid):

```rust
use mpesa::Mpesa;
use std::env;

let client = Mpesa::new(
      env::var("CLIENT_KEY")?,
      env::var("CLIENT_SECRET")?,
      "sandbox".parse()?, // "production"
);
assert!(client.is_connected())
```

If you intend to use in production, you will need to call a the `set_initiator_password` method from `Mpesa` after initially
creating the client. Here you provide your initiator password, which overrides the default password used in sandbox `"Safcom496!"`:

```rust
use mpesa::Mpesa;
use std::env;

let client = Mpesa::new(
      env::var("CLIENT_KEY")?,
      env::var("CLIENT_SECRET")?,
      "production".parse()?,
);

client.set_initiator_password("new_password");

assert!(client.is_connected())
```

### Services

The following services are currently available from the `Mpesa` client as methods that return builders:

- B2C

```rust
let response = client
    .b2c("testapi496")
    .party_a("600496")
    .party_b("254708374149")
    .result_url("https://testdomain.com/ok")
    .timeout_url("https://testdomain.com/err")
    .amount(1000)
    .send();
assert!(response.is_ok())
```

- B2B

```rust
let response = client
    .b2b("testapi496")
    .party_a("600496")
    .party_b("254708374149")
    .result_url("https://testdomain.com/ok")
    .timeout_url("https://testdomain.com/err")
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
    .result_url("https://testdomain.com/ok")
    .timeout_url("https://testdomain.com/err")
    .party_a("600496")
    .send();
assert!(response.is_ok())
```

- Mpesa Express Request / STK push / Lipa na M-PESA online

```rust
let response = client
    .express_request("174379")
    .phone_number("254708374149")
    .party_a("254708374149")
    .party_b("174379")
    .amount(500)
    .callback_url("https://test.example.com/api")
    .send();
assert!(response.is_ok())
```

More will be added progressively, pull requests welcome

## Author

**Collins Muriuki**

- Twitter: [@collinsmuriuki\_](https://twitter.com/collinsmuriuki_)
- Not affiliated with Safaricom.

## Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check [issues page](https://github.com/collinsmuriuki/mpesa-rust/issues). You can also take a look at the [contributing guide](CONTRIBUTING.md).

Copyright © 2021 [Collins Muriuki](https://github.com/collinsmuriuki).<br />
This project is [MIT](LICENSE) licensed.

# mpesa-rust

[![Rust](https://github.com/c12i/mpesa-rust/actions/workflows/general.yml/badge.svg?branch=master)](https://github.com/c12i/mpesa-rust/actions/workflows/general.yml)
[![](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![](https://img.shields.io/crates/v/mpesa)](https://crates.io/crates/mpesa)

[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/xswEKrrVGE)

## About

An unofficial Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.

## Install

`Cargo.toml`

```toml
[dependencies]
mpesa = { version = "1" }
```

Optionally, you can disable default-features, which is basically the entire suite of MPESA APIs to conditionally select individual features. (See [Services](#services) table for the full list of Cargo features)

Example:

```toml
[dependencies]
mpesa = { version = "1", default_features = false, features = ["b2b", "express_request"] }
```

In your lib or binary crate:

```rust
use mpesa::Mpesa;
```

## Usage

### Creating a `Mpesa` client

You will first need to create an instance of the `Mpesa` instance (the client). You are required to provide a **CLIENT_KEY** and
**CLIENT_SECRET**. [Here](https://developer.safaricom.co.ke/test_credentials) is how you can get these credentials for the Safaricom sandbox
environment. It's worth noting that these credentials are only valid in the sandbox environment. To go live and get production keys
read the docs [here](https://developer.safaricom.co.ke/docs?javascript#going-live).

These are the following ways you can instantiate `Mpesa`:

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

    assert!(client.is_connected().await);
}
```

Since the `Environment` enum implements `FromStr` and `TryFrom` for `String` and `&str` types, you can call `Environment::from_str` or `Environment::try_from` to create an `Environment` type. This is ideal if the environment values are
stored in a `.env` or any other configuration file:

```rust
use mpesa::{Mpesa, Environment};
use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let client = Mpesa::new(
        dotenvy::var("CLIENT_KEY").unwrap(),
        dotenvy::var("CLIENT_SECRET").unwrap(),
        Environment::from_str("sandbox")?, // or
        // Environment::try_from("sandbox")?,
    );

    assert!(client.is_connected().await);
    Ok(())
}
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

```rust
use mpesa::{Mpesa, ApiEnvironment};

#[derive(Clone)]
pub struct CustomEnvironment;

impl ApiEnvironment for CustomEnvironment {
    fn base_url(&self) -> &str {
        // your base url here
        "https://your_base_url.com"
    }

    fn get_certificate(&self) -> &str {
        // your certificate here
        r#"..."#
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client = Mpesa::new(
        dotenvy::var("CLIENT_KEY").unwrap(),
        dotenvy::var("CLIENT_SECRET").unwrap(),
        CustomEnvironment,
    );
}
```

If you intend to use in production, you will need to call a the `set_initiator_password` method from `Mpesa` after initially
creating the client. Here you provide your initiator password, which overrides the default password used in sandbox `"Safcom496!"`:

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

    client.set_initiator_password("new_password");
    assert!(client.is_connected().await)
}
```

### Services

The table below shows all the MPESA APIs from Safaricom and those supported by the crate along with their cargo features and usage examples

| API                                                                                                         | Cargo Feature          | Status          | Example                                                              |
| ----------------------------------------------------------------------------------------------------------- | ---------------------- | --------------- | -------------------------------------------------------------------- |
| [Account Balance](https://developer.safaricom.co.ke/APIs/AccountBalance)                                    | `account_balance`      | Stable ✅       | [account balance example](/docs/client/account_balance.md)           |
| [B2B Express Checkout](https://developer.safaricom.co.ke/APIs/B2BExpressCheckout)                           | N/A                    | Unimplemented   | N/A                                                                  |
| [Bill Manager](https://developer.safaricom.co.ke/APIs/BillManager)                                          | `bill_manager`         | Unstable ⚠️     | [bill manager examples](/docs/client/bill_manager/)                  |
| [Business Buy Goods](https://developer.safaricom.co.ke/APIs/BusinessBuyGoods)                               | `b2b`                  | Stable ✅       | [business buy goods example](/docs/client/b2b.md)                    |
| [Business Pay Bill](https://developer.safaricom.co.ke/APIs/BusinessPayBill)                                 | N/A                    | Unimplemented   | N/A                                                                  |
| [Business To Customer (B2C)](https://developer.safaricom.co.ke/APIs/BusinessToCustomer)                     | `b2c`                  | Stable ✅️      | [b2c example](/docs/client/b2c.md)                                   |
| [Customer To Business (Register URL)](https://developer.safaricom.co.ke/APIs/CustomerToBusinessRegisterURL) | `c2b_register`         | Stable ✅️      | [c2b register example](/docs/client/c2b_register.md)                 |
| [Customer To Business (Simulate)](#)                                                                        | `c2b_simulate`         | Stable ✅️      | [c2b simulate example](/docs/client/c2b_simulate.md)                 |
| [Dynamic QR](https://developer.safaricom.co.ke/APIs/DynamicQRCode)                                          | `dynamic_qr`           | Stable ✅️      | [dynamic qr example](/docs/client/dynamic_qr.md)                     |
| [M-PESA Express (Query)](https://developer.safaricom.co.ke/APIs/MpesaExpressQuery)                          | N/A                    | Unimplemented ️ | N/A                                                                  |
| [M-PESA Express (Simulate)/ STK push](https://developer.safaricom.co.ke/APIs/MpesaExpressSimulate)          | `express_request`      | Stable ✅️      | [express request example](/docs/client/express_request.md)           |
| [Transaction Status](https://developer.safaricom.co.ke/APIs/TransactionStatus)                              | `transaction_status`   | Stable ✅️      | [transaction status example](/docs/client/transaction_status.md)     |
| [Transaction Reversal](https://developer.safaricom.co.ke/APIs/Reversal)                                     | `transaction_reversal` | Stable ✅️      | [transaction reversal example](/docs/client/transaction_reversal.md) |
| [Tax Remittance](https://developer.safaricom.co.ke/APIs/TaxRemittance)                                      | N/A                    | Unimplemented   | N/A                                                                  |

## Author

**Collins Muriuki**

- Twitter: [@c12i\_](https://twitter.com/c12i_)
- Not affiliated with Safaricom.

## Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check [issues page](https://github.com/collinsmuriuki/mpesa-rust/issues). You can also take a look at the [contributing guide](https://raw.githubusercontent.com/collinsmuriuki/mpesa-rust/master/CONTRIBUTING.md).

<a href="https://github.com/c12i/mpesa-rust/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=c12i/mpesa-rust" />
</a>

Made with [contrib.rocks](https://contrib.rocks).

---

Copyright © 2023 [Collins Muriuki](https://github.com/collinsmuriuki).<br />
This project is [MIT](https://raw.githubusercontent.com/collinsmuriuki/mpesa-rust/master/LICENSE) licensed.

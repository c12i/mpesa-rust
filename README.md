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

A Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.

## Notes
 **Warning!** WIP, not production ready

## Install & Usage
`Cargo.toml`

```md
[dependencies]
mpesa = "0.1.6"
```

In your lib or binary crate:
```rs
use mpesa::Mpesa;
```

## Examples

```rs
use mpesa::{Mpesa, Environment};
use std::env;

let client = Mpesa::new(
      env::var("CLIENT_KEY")?,
      env::var("CLIENT_SECRET")?,
      Environment::Sandbox,
      env::var("INIT_PASSWORD")?,
);
```

## Author

**Collins Muriuki**

* Twitter: [@collinsmuriuki\_](https://twitter.com/collinsmuriuki_)

## Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check [issues page](https://github.com/collinsmuriuki/mpesa-rust/issues). You can also take a look at the [contributing guide](CONTRIBUTING.md).

Copyright © 2020 [Collins Muriuki](https://github.com/collinsmuriuki).<br />
This project is [MIT](LICENSE) licensed.
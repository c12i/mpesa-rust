<h1 align="center">mpesa-rust</h1>
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
  <a href="https://twitter.com/collinsmuriuki_" target="_blank">
    <img alt="Twitter: collinsmuriuki_" src="https://img.shields.io/twitter/follow/collinsmuriuki_.svg?style=social" />
  </a>
</p>

## About

A Rust wrapper around the [Safaricom API](https://developer.safaricom.co.ke/docs?shell#introduction) for accessing M-Pesa services.
Currently, a work in progress project.

## Notes
 **Warning!** v0.*. Expect bugs therefore not recommended for use in production. Pull requests and issues very welcome.

## Install & Usage
In your `Cargo.toml` file:

```md
[dependencies]
mpesa = "0.1.5"
```

In your lib or binary crate:
```rs
use mpesa::Mpesa;
```

## Examples

Use [`dotenv`](https://docs.rs/dotenv/0.15.0/dotenv/fn.dotenv.html) crate to store your keys as environmental variables instead of hard coding them like done in the example below.

```rs
use mpesa::{Mpesa, Environment};

let client = Mpesa::new(
      String::from("your_client_key"),
      String::from("your_client_secret"),
      Environment::Sandbox,
      String::from("your_initiator_password"),
);
```

## Author

**Collins Muriuki**

* Website: https://muriuki.dev
* Twitter: [@collinsmuriuki\_](https://twitter.com/collinsmuriuki_)
* Github: [@collinsmuriuki](https://github.com/collinsmuriuki)
* LinkedIn: [@collinsmuriuki](https://linkedin.com/in/collinsmuriuki)

## Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check [issues page](https://github.com/collinsmuriuki/mpesa-rust/issues). You can also take a look at the [contributing guide](CONTRIBUTING.md).

## Show your support

Give a ⭐️ if this project helped you!

## License

Copyright © 2020 [Collins Muriuki](https://github.com/collinsmuriuki).<br />
This project is [MIT](LICENSE) licensed.
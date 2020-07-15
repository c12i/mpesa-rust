<h1 align="center">mpesa-rust</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.0.1-blue.svg?cacheSeconds=2592000" />
  <a href="README.md" target="_blank">
    <img alt="Documentation" src="https://img.shields.io/badge/documentation-yes-brightgreen.svg" />
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
Currently a work in progress project.

## Notes
> Using reqwest::blocking for early build (will finally switch to async or have both options)

## RoadMap

- [x] Create Mpesa Client struct
- [x] Implement Auth
- [ ] Implement B2C payment
- [ ] Query transaction status
- [ ] Simulate C2B Payment
- [ ] Query status of Lipa na M-Pesa
- [ ] Initiate Lipa na M-Pesa online w/ STK push
- [ ] Register C2B Confirmation and Validation URLs
- [ ] Integration tests
- [ ] Rewrite in async
- [ ] Publish on https://crates.io
- [ ] Setup travis-ci

## Install & Usage
In your `Cargo.toml` file:

```md
[dependencies]
mpesa = "0.0.1"
```

In your lib or binary crate:
```rs
extern crate mpesa;

use mpesa::Mpesa;
```

## Examples

Use [`dotenv`](https://docs.rs/dotenv/0.15.0/dotenv/fn.dotenv.html) crate to store your keys as environmental variables instead of hard coding them like done in the example below.

```rs
use mpesa::{Mpesa, Environment};

let client = Mpesa::new(
      String::from("your_client_key"),
      String::from("your_client_secret"),
      Environment::Sandbox
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
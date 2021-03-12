# Contributing guide

> Instructions for local project setup and contribution.

## Rust

You will need Rust installed to run this project. You can find a guide to install on any os [here](https://www.rust-lang.org/tools/install)

## Install

```sh
git clone https://github.com/collinsmuriuki/mpesa-rust.git
cd mpesa-rust
```

## App Keys
Copy your safaricom credentials to an `.env` file in the root of the project, check `.env.example` for reference or simply run.

See [here](https://developer.safaricom.co.ke/docs#developer-sign-up) if you need to acquire app keys.
```sh
echo CLIENT_KEY="<your_client_key>" >> .env
echo CLIENT_SECRET="<your_client_secret>" >> .env
```

## Test Credentials
You can get test credentials [here](https://developer.safaricom.co.ke/test_credentials)

## Run tests

```sh
cargo test
```

## RoadMap

- [x] Create Mpesa Client struct
- [x] Implement Auth
- [x] Error handling
- [x] Generate security credentials
- [x] Implement B2C payment
- [x] Implement B2B payment
- [x] Query transaction status
- [x] Simulate C2B Payment
- [ ] Query status of Lipa na M-Pesa
- [x] Initiate Lipa na M-Pesa online w/ STK push
- [x] Register C2B Confirmation and Validation URLs
- [x] Integration tests
- [ ] Rewrite in async
- [x] Publish on https://crates.io
- [x] Setup travis-ci
- [ ] Improve documentation


## Pull Requests

Fork the repo and create a feature branch. Push your changes and make a PR.
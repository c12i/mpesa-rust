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
echo CLIENT_KEY="<your_client_key>" >> .env && echo CLIENT_SECRET="<your_client_secret>" >> .env
```

## Run tests

```sh
cargo test
```

## RoadMap

- [x] Create Mpesa Client struct
- [x] Implement Auth
- [ ] Error handling
- [x] Generate security credentials
- [x] Implement B2C payment
- [ ] Query transaction status
- [ ] Simulate C2B Payment
- [ ] Query status of Lipa na M-Pesa
- [ ] Initiate Lipa na M-Pesa online w/ STK push
- [ ] Register C2B Confirmation and Validation URLs
- [ ] Integration tests
- [ ] Rewrite in async
- [ ] Publish on https://crates.io
- [ ] Setup travis-ci


## Pull Requests

Fork the repo and create a feature branch. Push your changes and make a PR.
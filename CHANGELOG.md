# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.1](https://github.com/collinsmuriuki/mpesa-rust/compare/v1.1.0...v1.1.1) - 2024-01-28

### Fixed
- fix spelling
- fix formatting

### Other
- Merge pull request [#104](https://github.com/collinsmuriuki/mpesa-rust/pull/104) from c12i/chore/update-mpesa-variables
- follow API variable naming as per Daraja
- move express-request and transaction reversal to builder pattern ([#88](https://github.com/collinsmuriuki/mpesa-rust/pull/88))
- Add discord badge
- Merge pull request [#95](https://github.com/collinsmuriuki/mpesa-rust/pull/95) from Tevinthuku/improvements-and-less-noise
- static lifetime for request path
- remove unneeded lifetime
- refactor impl of getting the pass
- remove unused imports
- Add module level docs
- Update dynamic qr code setter and request amount field's type to u32
- Update twitter url
- Remove nextest config
- Simplify readme table
- Update readme
- Add no_run marker to c2b register
- Add main function to bill manager docs code snippets
- Move bill manager docs to md
- Remove unnecessary lazy evaluations
- Fix readme failing doc test
- Update docs
- Fix typo in docs
- Enable doc tests in readme
- Fix clippy warnings
- Further fix code snippet indentation in docs
- Only have core apis in doc tests
- Add editor config, fix indentation inconsistencies in docs
- Fix transaction reversal api 400 errors
- Fix dynamic qr code doc tests
- Update transaction apis docs
- Fix env vars not read in doc tests
- Enable and move doc tests to markdown files
- *(client)* Generic `send` implementation ([#89](https://github.com/collinsmuriuki/mpesa-rust/pull/89))

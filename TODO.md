# TODO

- [x] Unify `mpesa_derive` and `mpesa_core`
- [ ] Add missing services: `reversal`, `transaction_status`, `bill_manager`, `dynamic_qr_code`, `c2b_simulate_v2`
- [x] Clean up `Cargo.toml`: Correctly use Cargo features and declare optional and dev dependencies
- [ ] Convert library to async and update tests
- [ ] Migrate to `thiserror` and remove `failure`
- [ ] Refine tests: test more edge cases
- [ ] Address security issues in cargo security audit
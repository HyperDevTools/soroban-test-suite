# Soroban Test Suite 🧪

A Rust test-helper crate for Soroban smart contracts on Stellar, with fixture addresses, ledger-time controls, basic assertions, and CPU/memory budget measurements.

**Documentation site:** [Soroban Test Suite field guide](https://hyperdevtools.github.io/soroban-test-suite/)

> **Implementation note:** Some examples below describe planned helpers that are not in the current crate yet. In particular, contract mocking and balance/event/storage assertions are not implemented. See the [field guide](https://hyperdevtools.github.io/soroban-test-suite/) and current source before relying on an API.

## 🎯 Overview

`soroban-test-suite` provides ready-to-use helpers built on top of `soroban-sdk`'s `testutils`:
- **Test Context (`TestContext`)**: Manage a Soroban `Env` reference and generate tracked test `Address` accounts.
- **Contract Assertions (`assert_contract_eq!`, `assert_contract_gt!`, `assert_contract_gte!`)**: Expressive equality and comparison assertion macros with custom failure messages.
- **Soroban Contract Mocking (`MockContract`, `OnChainMockContract`)**: Register isolated mock contract dependencies in a Soroban `Env` with deterministic call expectations and explicit failure on unexpected calls.
- **Gas Budget Tracking (`GasTracker`, `GasUsage`)**: Snapshot and measure Soroban test budget CPU instructions and memory bytes across an interval.
- **Ledger Time Manipulation (`TimeController`)**: Inspect and advance the Soroban ledger timestamp and sequence number.
- **Reusable Test Fixtures (`BasicFixture`)**: Quickly provision an `admin` address and an indexed list of `users`.

## 📦 Installation

Add `soroban-test-suite` and `soroban-sdk` (with `testutils` enabled) to your `Cargo.toml`:

```toml
[dev-dependencies]
soroban-test-suite = "0.1"
soroban-sdk = { version = "21.0.0", features = ["testutils"] }
```

## 🔧 Quick Start

```rust
use soroban_sdk::{testutils::Address as _, Env};
use soroban_test_suite::prelude::*;

#[test]
fn quick_start_test() {
    let env = Env::default();
    let mut test = TestContext::new(&env);

    // Create tracked test accounts
    let alice = test.create_account();
    let bob = test.create_account();
    assert_contract_eq!(test.accounts().len(), 2, "Two accounts should be tracked");

    // Register a Soroban-compatible mock contract dependency
    let oracle = MockContract::register(&env, "price_oracle");
    oracle.expect_call(&env, "get_price", 12_345).unwrap();
    assert_contract_eq!(oracle.call(&env, "get_price").unwrap(), 12_345);

    // Measure Soroban CPU and memory budget usage across an interval
    let mut tracker = GasTracker::new(&env);
    tracker.start().expect("gas tracking should start");
    let _extra = soroban_sdk::Address::generate(&env);
    let usage = tracker.stop().expect("gas tracking should stop");

    assert_contract_gt!(usage.cpu_instructions, 0, "CPU instructions should be recorded");
    assert_ne!(alice, bob);
}
```

## 📚 Mocking External Contract Dependencies (`soroban_test_suite::mock`)

`MockContract::register(&env, id)` registers an isolated `OnChainMockContract` instance inside the Soroban `Env` and stores deterministic expectations in that contract's instance storage. Unconfigured or unexpected calls return `Err(TestError::MockError(...))` (or panic on cross-contract invocation) rather than silently returning default values:

```rust
use soroban_sdk::Env;
use soroban_test_suite::prelude::*;

#[test]
fn test_with_soroban_mocks() {
    let env = Env::default();

    let oracle = MockContract::register(&env, "oracle");
    oracle.expect_call_times(&env, "get_price", 12_345, 1).unwrap();

    let price = oracle.call(&env, "get_price").unwrap();
    assert_contract_eq!(price, 12_345);
    assert_contract_eq!(oracle.call_count("get_price"), 1);
    oracle.verify().unwrap();

    // Unconfigured calls fail explicitly
    assert!(oracle.call(&env, "unconfigured_fn").is_err());
}
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

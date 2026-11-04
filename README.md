# Soroban Test Suite

[![Crates.io](https://img.shields.io/crates/v/soroban-test-suite.svg)](https://crates.io/crates/soroban-test-suite)
[![Documentation](https://docs.rs/soroban-test-suite/badge.svg)](https://docs.rs/soroban-test-suite)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive testing framework for Soroban smart contracts on the Stellar network. Write cleaner, faster, and more reliable tests with minimal boilerplate.

## ✨ Features

- 🎯 **Simple API**: Intuitive helpers for common testing patterns
- ⚡ **Fast Setup**: Pre-configured test environments and fixtures
- 🔍 **Rich Assertions**: Detailed error messages and state inspection
- 🎭 **Mock Utilities**: Easy mocking of external contracts and accounts
- 📊 **Gas Tracking**: Built-in gas consumption measurement
- 🔄 **Time Manipulation**: Control ledger timestamps and sequence numbers

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
soroban-test-suite = "0.1"
soroban-sdk = "21.0"
```

## 🔧 Quick Start

```rust
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};
use soroban_test_suite::prelude::*;

const COUNT: Symbol = symbol_short!("COUNT");

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    pub fn increment(env: Env, _caller: Address, step: u32) -> u32 {
        let current: u32 = env.storage().instance().get(&COUNT).unwrap_or(0);
        let next = current + step;
        env.storage().instance().set(&COUNT, &next);
        next
    }
}

#[test]
fn test_counter_contract() {
    let env = Env::default();
    let mut test = TestContext::new(&env);

    // Setup accounts
    let alice = test.create_account();

    // Register contract using Soroban SDK
    let contract_id = env.register_contract(None, CounterContract);
    let client = CounterContractClient::new(&env, &contract_id);

    // Track gas while invoking the contract
    let mut tracker = GasTracker::new(&env);
    tracker.start().expect("gas tracking should start");
    let value = client.increment(&alice, &5);
    let usage = tracker.stop().expect("gas tracking should stop");

    // Assert contract output and gas usage
    assert_contract_eq!(value, 5);
    assert_contract_gt!(usage.cpu_instructions, 0);
}
```

## 📚 Documentation

### Test Context

The `TestContext` provides a convenient testing environment:

```rust
use soroban_sdk::Env;
use soroban_test_suite::prelude::*;

#[test]
fn example_test() {
    let env = Env::default();
    let mut test = TestContext::new(&env);

    // Create test accounts
    let admin = test.create_account();
    let users = test.create_accounts(2);

    assert_eq!(users.len(), 2);
    assert_ne!(admin, users[0]);
}
```

### Assertions

```rust
use soroban_test_suite::{assert_contract_eq, assert_contract_gt, assert_contract_gte};

assert_contract_eq!(1000, 1000, "Balances should match");
assert_contract_gt!(1000, 500, "Balance should exceed threshold");
assert_contract_gte!(500, 500, "Balance should meet minimum");
```

### Mocking

```rust
use soroban_sdk::Env;
use soroban_test_suite::prelude::*;

#[test]
fn test_with_mocks() {
    let env = Env::default();

    // Create a mock contract address for external dependency testing
    let oracle = MockContract::new(&env);
    assert!(!oracle.address().to_string().is_empty());
}
```

### Gas Tracking

```rust
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_test_suite::prelude::*;

#[test]
fn test_gas_usage() {
    let env = Env::default();
    let mut tracker = GasTracker::new(&env);

    tracker.start().expect("gas tracking should start");

    // Execute Soroban host operation
    let _address = Address::generate(&env);

    let usage = tracker.stop().expect("gas tracking should stop");

    // Assert gas usage is within acceptable range
    assert!(usage.cpu_instructions < 1_000_000);
    println!("Gas used: {} CPU instructions", usage.cpu_instructions);
}
```

### Time Manipulation

```rust
use soroban_sdk::Env;
use soroban_test_suite::prelude::*;

#[test]
fn test_time_lock() {
    let env = Env::default();
    let time = TimeController::new(&env);

    time.set(1_000);
    assert_contract_eq!(time.now(), 1_000);

    // Fast forward 24 hours (86,400 seconds)
    time.advance(86_400);
    assert_contract_eq!(time.now(), 87_400);
}
```

### Test Fixtures

```rust
use soroban_test_suite::prelude::*;

#[test]
fn test_with_basic_fixture() {
    let fixture = BasicFixture::new();

    assert_eq!(fixture.users.len(), 3);
    assert_ne!(fixture.admin, fixture.users[0]);
}
```

## 🏃 Running the Example

Run the bundled Cargo example directly with:

```bash
cargo test --example basic_contract_test
cargo run --example basic_contract_test
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📋 Roadmap

- [ ] High-level balance, storage, and event assertion helpers on `TestContext`
- [ ] Function-level call expectations and return stubbing on `MockContract`
- [ ] Domain-specific DeFi fixtures (`DeFiFixture`)
- [ ] Snapshot testing support
- [ ] Property-based testing helpers
- [ ] Performance benchmarking utilities
- [ ] Coverage reporting integration

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Developers](https://developers.stellar.org/)

# Soroban Test Suite 🧪

A lightweight testing helper crate for Soroban smart contracts on Stellar (`soroban-sdk = "21.0.0"`).

## 🎯 Overview

`soroban-test-suite` provides ready-to-use helpers built on top of `soroban-sdk`'s `testutils`:
- **Test Context (`TestContext`)**: Manage a Soroban `Env` reference and generate tracked test `Address` accounts.
- **Contract Assertions (`assert_contract_eq!`, `assert_contract_gt!`, `assert_contract_gte!`)**: Expressive equality and comparison assertion macros with custom failure messages.
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

Every snippet below matches the implemented crate APIs and compiles against `soroban-sdk = "21.0.0"`. You can also run the included Cargo example directly with:

```bash
cargo test --all-targets
cargo run --example basic_contract_test
```

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

    // Measure Soroban CPU and memory budget usage across an interval
    let mut tracker = GasTracker::new(&env);
    tracker.start().expect("gas tracking should start");
    let _extra = soroban_sdk::Address::generate(&env);
    let usage = tracker.stop().expect("gas tracking should stop");

    assert_contract_gt!(usage.cpu_instructions, 0, "CPU instructions should be recorded");
    assert_contract_gte!(usage.memory_bytes, 1, "Memory bytes should be at least 1");
    assert_ne!(alice, bob);
}
```

## 📚 Implemented API Guide

### 1. `TestContext` (`soroban_test_suite::context::TestContext`)

`TestContext` wraps a `&Env` and tracks generated Soroban `Address` accounts:

```rust
use soroban_sdk::Env;
use soroban_test_suite::prelude::*;

#[test]
fn context_example() {
    let env = Env::default();
    let mut ctx = TestContext::new(&env);

    let admin = ctx.create_account();
    let participants = ctx.create_accounts(3);

    assert_contract_eq!(participants.len(), 3);
    assert_contract_eq!(ctx.accounts().len(), 4);
    assert_contract_eq!(ctx.accounts()[0], admin);
}
```

### 2. Assertion Macros (`soroban_test_suite::assertions`)

Use `assert_contract_eq!`, `assert_contract_gt!`, and `assert_contract_gte!` with default or custom diagnostic messages:

```rust
use soroban_test_suite::prelude::*;

#[test]
fn assertions_example() {
    let balance = 1_000_i128;
    let minimum = 500_i128;

    assert_contract_eq!(balance, 1_000);
    assert_contract_eq!(balance, 1_000, "Initial balance must equal 1000");
    assert_contract_gt!(balance, minimum, "Balance must exceed minimum");
    assert_contract_gte!(balance, 1_000, "Balance must be at least 1000");
}
```

### 3. `GasTracker` and `GasUsage` (`soroban_test_suite::gas`)

Measure CPU instructions and memory bytes reported by `env.budget()` during a test interval:

```rust
use soroban_sdk::{testutils::Address as _, Env};
use soroban_test_suite::prelude::*;

#[test]
fn gas_tracking_example() {
    let env = Env::default();
    let mut tracker = GasTracker::new(&env);

    tracker.start().expect("gas tracking should start");
    let _addr = soroban_sdk::Address::generate(&env);
    let usage: GasUsage = tracker.stop().expect("gas tracking should stop");

    assert_contract_gt!(usage.cpu_instructions, 0);
    assert_contract_gt!(usage.memory_bytes, 0);
}
```

### 4. `TimeController` (`soroban_test_suite::time`)

Manipulate the Soroban ledger timestamp and sequence number in time-dependent tests:

```rust
use soroban_sdk::Env;
use soroban_test_suite::prelude::*;

#[test]
fn time_control_example() {
    let env = Env::default();
    let time = TimeController::new(&env);

    time.set(1_700_000_000);
    assert_contract_eq!(time.now(), 1_700_000_000);

    // Fast-forward 1 hour (3600 seconds)
    time.advance(3_600);
    assert_contract_eq!(time.now(), 1_700_003_600);

    // Advance to the next ledger (+1 sequence number, +5 seconds)
    let seq_before = env.ledger().sequence();
    time.next_ledger();
    assert_contract_eq!(env.ledger().sequence(), seq_before + 1);
    assert_contract_eq!(time.now(), 1_700_003_605);
}
```

### 5. `BasicFixture` (`soroban_test_suite::fixtures`)

Create a reusable fixture containing an `admin` account and `N` user accounts:

```rust
use soroban_sdk::Env;
use soroban_test_suite::prelude::*;

#[test]
fn fixture_example() {
    let env = Env::default();
    let fixture = BasicFixture::new(&env, 2);

    assert_contract_eq!(fixture.users.len(), 2);
    assert_ne!(&fixture.admin, fixture.user(0));
    assert_ne!(fixture.user(0), fixture.user(1));
}
```

## 📋 Roadmap (Not Yet Implemented)

The following capabilities are planned roadmap items and are **not** part of the current `0.1.0` API:
- [ ] Soroban contract registration & token balance assertion helpers on `TestContext`
- [ ] Contract event & storage inspection assertions
- [ ] Soroban-compatible contract dependency mocking (`src/mock.rs` currently provides only a minimal `MockContract` identifier struct)
- [ ] Snapshot testing and property-based testing helpers
- [ ] Fuzzing and coverage reporting integration

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines and [SECURITY.md](SECURITY.md) for vulnerability reporting.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

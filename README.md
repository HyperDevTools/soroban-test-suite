# Soroban Test Suite

Small Rust helpers for testing Soroban contracts. The project is experimental
and its API is still developing.

## What is implemented

- `TestContext` creates and tracks test addresses.
- `BasicFixture` creates an admin address and a set of user addresses.
- Assertion macros compare values with `==`, `>`, and `>=`.
- `GasTracker` measures CPU instructions and memory reported by the Soroban
  test budget. Native Rust execution can underestimate the equivalent WASM
  cost.
- `TimeController` reads and changes the test ledger timestamp and sequence.

This crate does not currently register contracts or provide token, balance,
event, storage, snapshot, or general contract-mocking helpers. `MockContract`
is currently only an ID wrapper.

## Use from a Soroban project

The crate is not currently published on crates.io. Add it from GitHub as a
development dependency:

```toml
[dev-dependencies]
soroban-test-suite = { git = "https://github.com/HyperDevTools/soroban-test-suite.git" }
```

For example, use the context and comparison macros in a test:

```rust
use soroban_sdk::Env;
use soroban_test_suite::prelude::*;

#[test]
fn test_context_tracks_generated_accounts() {
    let env = Env::default();
    let mut test = TestContext::new(&env);

    let alice = test.create_account();
    let bob = test.create_account();

    assert_eq!(test.accounts().len(), 2);
    assert_contract_gt!(test.accounts().len(), 1);
    assert_ne!(alice, bob);
}
```

To track budget use or change ledger time, use `GasTracker` or
`TimeController` from the prelude:

```rust
let mut tracker = GasTracker::new(&env);
tracker.start().expect("gas tracking should start");
// Invoke the contract operation under test.
let usage = tracker.stop().expect("gas tracking should stop");

let time = TimeController::new(&env);
time.advance(60);
```

## Development

Run the checks from a checkout:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

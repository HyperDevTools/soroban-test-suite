# Soroban Test Suite 🧪

A comprehensive testing framework for Soroban smart contracts on Stellar. Write better tests with powerful assertions, mock utilities, and helpful test patterns.

## 🎯 Overview

Soroban Test Suite simplifies testing smart contracts by providing:
- **Fluent Assertions**: Expressive, chainable assertions for contract behavior
- **Mock Utilities**: Easy mocking of contract calls, storage, and events
- **Test Fixtures**: Reusable test setups and common patterns
- **Gas Tracking**: Monitor and assert on computational costs
- **Event Testing**: Comprehensive event emission testing
- **Time Manipulation**: Control time in tests for time-dependent contracts

## 🚀 Features

- ✅ **Rich Assertions** - Expressive assertions for accounts, balances, events, and storage
- 🎭 **Mocking Support** - Mock external contract calls and dependencies
- 📊 **Gas Analysis** - Track and optimize gas usage in tests
- ⏰ **Time Control** - Fast-forward time for testing vesting, locks, etc.
- 🔍 **Error Testing** - Test error conditions and failure paths easily
- 📝 **Test Utilities** - Helpers for common testing patterns
- 🎨 **Snapshot Testing** - Compare contract state across test runs

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
soroban-test-suite = "0.1"
soroban-sdk = "21.0"
```

## 🔧 Quick Start

```rust
use soroban_sdk::{Env, testutils::Address as _};
use soroban_test_suite::prelude::*;

#[test]
fn test_token_transfer() {
    let env = Env::default();
    let test = TestContext::new(&env);
    
    // Setup accounts
    let alice = test.create_account();
    let bob = test.create_account();
    
    // Initialize contract
    let contract = test.register_contract(token::WASM);
    let client = token::Client::new(&env, &contract);
    
    // Mint tokens to Alice
    client.mint(&alice, &1000);
    
    // Assert balance
    test.assert_balance(&alice, &contract, 1000);
    
    // Transfer tokens
    client.transfer(&alice, &bob, &500);
    
    // Assert final balances
    test.assert_balance(&alice, &contract, 500);
    test.assert_balance(&bob, &contract, 500);
}
```

## 📚 Documentation

### Test Context

The `TestContext` provides a convenient testing environment:

```rust
use soroban_test_suite::prelude::*;

#[test]
fn example_test() {
    let env = Env::default();
    let test = TestContext::new(&env);
    
    // Create test accounts
    let admin = test.create_account();
    let user1 = test.create_account();
    let user2 = test.create_account();
    
    // Register contracts
    let contract_id = test.register_contract(CONTRACT_WASM);
}
```

### Assertions

```rust
// Balance assertions
test.assert_balance(&address, &token, 1000);
test.assert_balance_gt(&address, &token, 500); // greater than
test.assert_balance_gte(&address, &token, 500); // greater than or equal

// Event assertions
test.assert_event_emitted(&contract, "Transfer");
test.assert_events_in_order(&contract, vec!["Mint", "Transfer", "Burn"]);

// Storage assertions
test.assert_storage_value(&contract, "counter", 42);
test.assert_storage_exists(&contract, "initialized");

// Error assertions
test.assert_error(&|| {
    client.unauthorized_operation(&user);
}, ErrorCode::Unauthorized);
```

### Mocking

```rust
use soroban_test_suite::mock::*;

#[test]
fn test_with_mocks() {
    let env = Env::default();
    let test = TestContext::new(&env);
    
    // Mock external contract call
    let oracle = test.mock_contract();
    mock_call!(oracle, "get_price", || 12345);
    
    // Use in your contract tests
    let price = oracle.get_price();
    assert_eq!(price, 12345);
}
```

### Gas Tracking

```rust
use soroban_test_suite::gas::*;

#[test]
fn test_gas_usage() {
    let env = Env::default();
    let test = TestContext::new(&env);
    let tracker = GasTracker::new(&env);
    
    tracker.start();
    
    // Execute contract operation
    client.expensive_operation(&params);
    
    let gas_used = tracker.stop();
    
    // Assert gas usage is within acceptable range
    assert!(gas_used < 1_000_000, "Gas usage too high: {}", gas_used);
}
```

### Time Manipulation

```rust
use soroban_test_suite::time::*;

#[test]
fn test_time_lock() {
    let env = Env::default();
    let test = TestContext::new(&env);
    
    // Initialize time-locked contract
    let unlock_time = env.ledger().timestamp() + 86400; // 24 hours
    client.create_lock(&user, &1000, &unlock_time);
    
    // Try to unlock early (should fail)
    test.assert_error(&|| {
        client.unlock(&user);
    }, ErrorCode::Locked);
    
    // Fast forward 24 hours
    test.advance_time(86400);
    
    // Now unlock should succeed
    client.unlock(&user);
    test.assert_balance(&user, &contract, 1000);
}
```

### Test Fixtures

```rust
use soroban_test_suite::fixtures::*;

// Reusable test setup
fn setup_defi_protocol() -> DeFiFixture {
    let env = Env::default();
    let test = TestContext::new(&env);
    
    let admin = test.create_account();
    let pool = test.register_contract(pool::WASM);
    let token_a = test.register_token();
    let token_b = test.register_token();
    
    DeFiFixture {
        env,
        test,
        admin,
        pool,
        token_a,
        token_b,
    }
}

#[test]
fn test_liquidity_pool() {
    let fixture = setup_defi_protocol();
    
    // Test liquidity operations
    fixture.pool.add_liquidity(&fixture.admin, &1000, &1000);
    
    fixture.test.assert_balance(&fixture.admin, &fixture.token_a, 0);
}
```

## 🧪 Testing Patterns

### Unit Tests

```rust
#[test]
fn test_initialization() {
    let env = Env::default();
    let test = TestContext::new(&env);
    let contract = test.register_contract(WASM);
    
    let admin = test.create_account();
    contract.initialize(&admin);
    
    test.assert_storage_value(&contract, "admin", admin);
}
```

### Integration Tests

```rust
#[test]
fn test_multi_contract_interaction() {
    let env = Env::default();
    let test = TestContext::new(&env);
    
    let token = test.register_contract(token::WASM);
    let dex = test.register_contract(dex::WASM);
    
    // Test interaction between contracts
    token.mint(&user, &1000);
    token.approve(&user, &dex, &500);
    dex.swap(&user, &token, &500);
}
```

### Property-Based Tests

```rust
#[test]
fn test_invariants() {
    let env = Env::default();
    let test = TestContext::new(&env);
    
    // Test that total supply is always conserved
    for _ in 0..100 {
        let amount = test.random_amount(1, 1000);
        contract.transfer(&alice, &bob, &amount);
        
        let total = contract.balance(&alice) + contract.balance(&bob);
        assert_eq!(total, INITIAL_SUPPLY);
    }
}
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📋 Roadmap

- [ ] Snapshot testing support
- [ ] Property-based testing helpers
- [ ] Performance benchmarking utilities
- [ ] Contract upgrade testing
- [ ] Multi-chain testing support
- [ ] Visual test reports
- [ ] Fuzzing integration
- [ ] Coverage reporting

## 📄 License

MIT License - see [LICENSE](LICENSE) for details

## 🔗 Links

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Developers](https://developers.stellar.org/)
- [Issue Tracker](https://github.com/HyperDevTools/soroban-test-suite/issues)

## 💬 Support

- Open an issue for bug reports or feature requests
- Join our community discussions
- Contribute to the codebase

---

Made with ❤️ by HyperDevTools

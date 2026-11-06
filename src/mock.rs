//! Mock utilities for contract testing

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

use crate::error::{TestError, TestResult};

/// Internal Soroban contract backing each registered [`MockContract`] instance.
#[contract]
pub struct OnChainMockContract;

#[contractimpl]
impl OnChainMockContract {
    /// Invoke a mocked function by symbol inside Soroban contract execution.
    ///
    /// Panics if the function was not configured on this mock contract instance.
    pub fn invoke(env: Env, func: Symbol) -> i128 {
        let storage = env.storage().instance();
        let value: Option<i128> = storage.get(&func);
        match value {
            Some(v) => v,
            None => panic!("MockError: unexpected call to unconfigured mock function"),
        }
    }
}

#[derive(Clone, Debug)]
struct MockExpectation {
    return_value: i128,
    expected_calls: Option<u32>,
    actual_calls: u32,
}

/// Deterministic mock contract utility backed by a registered Soroban contract [`Address`].
///
/// # Limitations
/// - Designed for deterministic unit and integration testing inside `soroban_sdk::Env`.
/// - Does not execute arbitrary untrusted bytecode; responses must be explicitly configured via
///   [`MockContract::expect_call`] or [`MockContract::expect_call_times`].
#[derive(Clone, Debug)]
pub struct MockContract {
    id: String,
    address: Option<Address>,
    expectations: Arc<Mutex<HashMap<String, MockExpectation>>>,
}

impl MockContract {
    /// Create a standalone named mock identifier (preserves backward compatibility).
    pub fn new(id: String) -> Self {
        Self {
            id,
            address: None,
            expectations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register an isolated mock contract instance inside the given Soroban [`Env`].
    pub fn register(env: &Env, id: impl Into<String>) -> Self {
        let id_str = id.into();
        let address = env.register_contract(None, OnChainMockContract);
        Self {
            id: id_str,
            address: Some(address),
            expectations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Return the mock identifier string.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the registered Soroban contract [`Address`], if registered in an [`Env`].
    pub fn address(&self) -> TestResult<&Address> {
        self.address.as_ref().ok_or_else(|| {
            TestError::MockError(format!(
                "mock '{}' is not registered in a Soroban Env; use MockContract::register(&env, id)",
                self.id
            ))
        })
    }

    /// Configure a deterministic return value for `fn_name`.
    ///
    /// Returns [`TestError::MockError`] if `fn_name` is empty or exceeds Soroban `Symbol` limits.
    pub fn expect_call(&self, env: &Env, fn_name: &str, return_value: i128) -> TestResult<()> {
        self.configure_expectation(env, fn_name, return_value, None)
    }

    /// Configure a deterministic return value for `fn_name` with an exact expected call count.
    pub fn expect_call_times(
        &self,
        env: &Env,
        fn_name: &str,
        return_value: i128,
        expected_calls: u32,
    ) -> TestResult<()> {
        if expected_calls == 0 {
            return Err(TestError::MockError(
                "expected_calls must be at least 1; omit expectation to forbid calls".to_string(),
            ));
        }
        self.configure_expectation(env, fn_name, return_value, Some(expected_calls))
    }

    fn configure_expectation(
        &self,
        env: &Env,
        fn_name: &str,
        return_value: i128,
        expected_calls: Option<u32>,
    ) -> TestResult<()> {
        let trimmed = fn_name.trim();
        if trimmed.is_empty() {
            return Err(TestError::MockError(
                "mock function name must not be empty".to_string(),
            ));
        }
        if trimmed.len() > 32 {
            return Err(TestError::MockError(format!(
                "mock function name '{}' exceeds 32-character Soroban Symbol limit",
                trimmed
            )));
        }

        if let Some(addr) = &self.address {
            let sym = Symbol::new(env, trimmed);
            env.as_contract(addr, || {
                env.storage().instance().set(&sym, &return_value);
            });
        }

        let mut map = self
            .expectations
            .lock()
            .map_err(|_| TestError::MockError("mock state lock poisoned".to_string()))?;
        map.insert(
            trimmed.to_string(),
            MockExpectation {
                return_value,
                expected_calls,
                actual_calls: 0,
            },
        );
        Ok(())
    }

    /// Invoke a configured mock function and record the call.
    ///
    /// Fails with [`TestError::MockError`] if the function was never configured or if its
    /// configured call count has already been exhausted.
    pub fn call(&self, env: &Env, fn_name: &str) -> TestResult<i128> {
        let mut map = self
            .expectations
            .lock()
            .map_err(|_| TestError::MockError("mock state lock poisoned".to_string()))?;
        let exp = map.get_mut(fn_name).ok_or_else(|| {
            TestError::MockError(format!(
                "unexpected call to unconfigured function '{}' on mock '{}'",
                fn_name, self.id
            ))
        })?;

        if let Some(limit) = exp.expected_calls {
            if exp.actual_calls >= limit {
                return Err(TestError::MockError(format!(
                    "function '{}' on mock '{}' exceeded expected call count ({})",
                    fn_name, self.id, limit
                )));
            }
        }

        exp.actual_calls += 1;
        let expected_return = exp.return_value;
        drop(map);

        if let Some(addr) = &self.address {
            let client = OnChainMockContractClient::new(env, addr);
            let sym = Symbol::new(env, fn_name);
            Ok(client.invoke(&sym))
        } else {
            Ok(expected_return)
        }
    }

    /// Return how many times `fn_name` was called on this mock instance.
    pub fn call_count(&self, fn_name: &str) -> u32 {
        self.expectations
            .lock()
            .ok()
            .and_then(|m| m.get(fn_name).map(|e| e.actual_calls))
            .unwrap_or(0)
    }

    /// Verify that all configured expectations with explicit call counts were satisfied.
    pub fn verify(&self) -> TestResult<()> {
        let map = self
            .expectations
            .lock()
            .map_err(|_| TestError::MockError("mock state lock poisoned".to_string()))?;
        for (name, exp) in map.iter() {
            if let Some(expected) = exp.expected_calls {
                if exp.actual_calls != expected {
                    return Err(TestError::MockError(format!(
                        "mock '{}' function '{}' expected {} call(s), but got {}",
                        self.id, name, expected, exp.actual_calls
                    )));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[contract]
    pub struct ConsumerContract;

    #[contractimpl]
    impl ConsumerContract {
        pub fn fetch_doubled_price(env: Env, oracle: Address, feed: Symbol) -> i128 {
            let client = OnChainMockContractClient::new(&env, &oracle);
            client.invoke(&feed) * 2
        }
    }

    #[test]
    fn test_mock_contract_creation() {
        let mock = MockContract::new("test_contract".to_string());
        assert_eq!(mock.id(), "test_contract");
        assert!(mock.address().is_err());
    }

    #[test]
    fn test_registered_mock_expected_call_and_cross_contract_invocation() {
        let env = Env::default();
        let oracle = MockContract::register(&env, "price_oracle");
        oracle.expect_call(&env, "get_price", 12_345).unwrap();

        assert_eq!(oracle.call(&env, "get_price").unwrap(), 12_345);
        assert_eq!(oracle.call_count("get_price"), 1);

        // Verify a real Soroban consumer contract can call the mock dependency on-chain
        let consumer_id = env.register_contract(None, ConsumerContract);
        let consumer = ConsumerContractClient::new(&env, &consumer_id);
        let doubled = consumer.fetch_doubled_price(
            oracle.address().unwrap(),
            &Symbol::new(&env, "get_price"),
        );
        assert_eq!(doubled, 24_690);
    }

    #[test]
    fn test_unexpected_call_fails_clearly() {
        let env = Env::default();
        let mock = MockContract::register(&env, "strict_oracle");

        let err = mock.call(&env, "unconfigured_fn").unwrap_err();
        assert!(matches!(err, TestError::MockError(_)));
        assert!(err.to_string().contains("unconfigured_fn"));
    }

    #[test]
    fn test_invalid_mock_setup_rejected() {
        let env = Env::default();
        let mock = MockContract::register(&env, "oracle");

        assert!(matches!(
            mock.expect_call(&env, "   ", 100),
            Err(TestError::MockError(_))
        ));
        assert!(matches!(
            mock.expect_call_times(&env, "get_price", 100, 0),
            Err(TestError::MockError(_))
        ));
    }

    #[test]
    fn test_expected_call_count_enforcement_and_verify() {
        let env = Env::default();
        let mock = MockContract::register(&env, "bounded_oracle");
        mock.expect_call_times(&env, "quote", 500, 2).unwrap();

        assert!(mock.verify().is_err());
        assert_eq!(mock.call(&env, "quote").unwrap(), 500);
        assert!(mock.verify().is_err());
        assert_eq!(mock.call(&env, "quote").unwrap(), 500);
        assert!(mock.verify().is_ok());

        // Third call exceeds configured limit of 2
        assert!(matches!(
            mock.call(&env, "quote"),
            Err(TestError::MockError(_))
        ));
    }

    #[test]
    fn test_independent_mock_instances_do_not_collide() {
        let env = Env::default();
        let oracle_a = MockContract::register(&env, "oracle_a");
        let oracle_b = MockContract::register(&env, "oracle_b");

        oracle_a.expect_call(&env, "get_price", 100).unwrap();
        oracle_b.expect_call(&env, "get_price", 999).unwrap();

        assert_ne!(oracle_a.address().unwrap(), oracle_b.address().unwrap());
        assert_eq!(oracle_a.call(&env, "get_price").unwrap(), 100);
        assert_eq!(oracle_b.call(&env, "get_price").unwrap(), 999);
        assert!(oracle_a.call(&env, "other_fn").is_err());
    }
}

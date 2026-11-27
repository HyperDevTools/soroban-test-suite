//! Assertions for events published by Soroban contracts in a test environment.

use soroban_sdk::{testutils::Events as _, Address, Env, Val, Vec};

use crate::error::{TestError, TestResult};

/// An expected contract event. Construct its values in the environment under test.
#[derive(Clone, Debug)]
pub struct ContractEvent {
    pub contract_id: Address,
    pub topics: Vec<Val>,
    pub data: Val,
}

impl ContractEvent {
    fn matches(&self, env: &Env, actual: &(Address, Vec<Val>, Val)) -> bool {
        self.contract_id == actual.0
            && self.topics == actual.1
            && Vec::from_array(env, [self.data]) == Vec::from_array(env, [actual.2])
    }
}

/// Require at least one matching event in the environment's event history.
pub fn assert_event(env: &Env, expected: &ContractEvent) -> TestResult<()> {
    if env
        .events()
        .all()
        .iter()
        .any(|event| expected.matches(env, &event))
    {
        return Ok(());
    }
    Err(TestError::AssertionFailed(format!(
        "Missing contract event: {expected:?}"
    )))
}

/// Require that no event in the environment's history matches the expectation.
pub fn assert_no_event(env: &Env, expected: &ContractEvent) -> TestResult<()> {
    if env
        .events()
        .all()
        .iter()
        .any(|event| expected.matches(env, &event))
    {
        return Err(TestError::AssertionFailed(format!(
            "Unexpected contract event: {expected:?}"
        )));
    }
    Ok(())
}

/// Match the entire event history and report the first mismatch by zero-based index.
pub fn assert_events_in_order(env: &Env, expected: &[ContractEvent]) -> TestResult<()> {
    let events = env.events().all();
    let mut actual = events.iter();
    for (index, event) in expected.iter().enumerate() {
        match actual.next() {
            Some(actual) if event.matches(env, &actual) => {}
            Some(actual) => {
                return Err(TestError::AssertionFailed(format!(
                    "Event at index {index} differs: expected {event:?}, observed {actual:?}"
                )));
            }
            None => {
                return Err(TestError::AssertionFailed(format!(
                    "Missing event at index {index}: {event:?}"
                )));
            }
        }
    }
    if let Some(event) = actual.next() {
        return Err(TestError::AssertionFailed(format!(
            "Unexpected event at index {}: {event:?}",
            expected.len()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use soroban_sdk::{
        contract, contractimpl, symbol_short, testutils::EnvTestConfig, Bytes, IntoVal, Symbol,
    };

    use super::*;

    #[contract]
    struct Emitter;

    #[contractimpl]
    impl Emitter {
        pub fn emit(env: Env, topic: Symbol, data: Bytes) {
            env.events().publish((topic,), data);
        }
    }

    fn expected(env: &Env, contract_id: &Address, topic: Symbol, data: &[u8]) -> ContractEvent {
        ContractEvent {
            contract_id: contract_id.clone(),
            topics: Vec::from_array(env, [topic.into_val(env)]),
            data: Bytes::from_slice(env, data).into_val(env),
        }
    }

    #[test]
    fn matches_contract_topics_and_structural_payload() {
        let env = Env::new_with_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
        let first = env.register_contract(None, Emitter);
        let second = env.register_contract(None, Emitter);
        let topic = symbol_short!("deposit");
        let other = symbol_short!("withdraw");
        EmitterClient::new(&env, &first).emit(&topic, &Bytes::from_slice(&env, b"10"));

        let observed = expected(&env, &first, topic.clone(), b"10");
        assert_event(&env, &observed).unwrap();
        assert!(assert_no_event(&env, &observed).is_err());
        for absent in [
            expected(&env, &second, topic.clone(), b"10"),
            expected(&env, &first, other, b"10"),
            expected(&env, &first, topic, b"99"),
        ] {
            assert!(assert_event(&env, &absent).is_err());
            assert_no_event(&env, &absent).unwrap();
        }
    }

    #[test]
    fn checks_order_and_reports_missing_and_extra_events() {
        let env = Env::new_with_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
        let first = env.register_contract(None, Emitter);
        let second = env.register_contract(None, Emitter);
        let topic = symbol_short!("deposit");
        let first_event = expected(&env, &first, topic.clone(), b"10");
        let second_event = expected(&env, &second, topic.clone(), b"20");
        EmitterClient::new(&env, &first).emit(&topic, &Bytes::from_slice(&env, b"10"));
        assert!(
            assert_events_in_order(&env, &[first_event.clone(), second_event.clone()])
                .unwrap_err()
                .to_string()
                .contains("Missing event at index 1")
        );

        EmitterClient::new(&env, &second).emit(&topic, &Bytes::from_slice(&env, b"20"));
        assert_events_in_order(&env, &[first_event.clone(), second_event.clone()]).unwrap();
        assert!(
            assert_events_in_order(&env, &[second_event, first_event.clone()])
                .unwrap_err()
                .to_string()
                .contains("Event at index 0 differs")
        );
        assert!(assert_events_in_order(&env, &[first_event])
            .unwrap_err()
            .to_string()
            .contains("Unexpected event at index 1"));
    }
}

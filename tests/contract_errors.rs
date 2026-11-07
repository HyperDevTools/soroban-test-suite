//! The error arm of a contract call, asserted through the pinned SDK shape.
//!
//! `Env::try_invoke_contract` answers `Ok(Ok(v))` on success, `Ok(Err(e))` when the
//! contract itself failed, and `Err(_)` when the host could not run or convert the
//! call. The helpers have to tell those apart, so the fixtures below produce every
//! arm rather than only the one that is convenient.

use soroban_sdk::{
    contract, contracterror, contractimpl, symbol_short, Address, ConversionError, Env, IntoVal,
};
use soroban_test_suite::{assert_contract_err, assert_contract_ok};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DemoError {
    Unauthorized = 1,
    NotFound = 2,
}

#[contract]
pub struct Demo;

#[contractimpl]
impl Demo {
    pub fn ok(_env: Env) -> u32 {
        7
    }

    pub fn fail_with(which: u32) -> Result<(), DemoError> {
        match which {
            1 => Err(DemoError::Unauthorized),
            2 => Err(DemoError::NotFound),
            _ => Ok(()),
        }
    }
}

fn setup() -> (Env, Address) {
    let env = Env::default();
    let id = env.register_contract(None, Demo);
    (env, id)
}

type Outcome = Result<Result<(), ConversionError>, Result<DemoError, soroban_sdk::InvokeError>>;

fn invoke(env: &Env, id: &Address, which: u32) -> Outcome {
    env.try_invoke_contract::<(), DemoError>(
        id,
        &symbol_short!("fail_with"),
        (which,).into_val(env),
    )
}

#[test]
fn the_expected_error_passes() {
    let (env, id) = setup();
    assert_contract_err!(invoke(&env, &id, 1), DemoError::Unauthorized);
}

#[test]
fn a_pattern_with_a_binding_still_matches_by_type() {
    let (env, id) = setup();
    assert_contract_err!(
        invoke(&env, &id, 2),
        DemoError::NotFound,
        "the record is missing"
    );
}

#[test]
fn a_successful_call_is_reported_as_an_unexpected_success() {
    let (env, id) = setup();
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_contract_err!(invoke(&env, &id, 3), DemoError::Unauthorized);
    }))
    .unwrap_err();
    let text = panic_text(panic);
    assert!(text.contains("but the call succeeded"), "{text}");
}

#[test]
fn a_different_error_is_reported_with_the_expected_versus_actual() {
    let (env, id) = setup();
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_contract_err!(invoke(&env, &id, 1), DemoError::NotFound, "wrong one");
    }))
    .unwrap_err();
    let text = panic_text(panic);
    assert!(text.contains("Expected DemoError::NotFound"), "{text}");
    assert!(text.contains("Unauthorized"), "{text}");
}

#[test]
fn the_expression_is_evaluated_exactly_once() {
    use std::cell::Cell;
    let (env, id) = setup();
    let calls = Cell::new(0);
    let once = || {
        calls.set(calls.get() + 1);
        invoke(&env, &id, 1)
    };
    assert_contract_err!(once(), DemoError::Unauthorized);
    assert_eq!(calls.get(), 1);
}

#[test]
fn the_success_helper_returns_the_value() {
    let (env, id) = setup();
    let value = assert_contract_ok!(env.try_invoke_contract::<u32, DemoError>(
        &id,
        &symbol_short!("ok"),
        ().into_val(&env)
    ));
    assert_eq!(value, 7);
}

#[test]
fn the_success_helper_fails_on_a_contract_error() {
    let (env, id) = setup();
    let outcome = invoke(&env, &id, 1);
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_contract_ok!(outcome, "should have succeeded");
    }))
    .unwrap_err();
    assert!(panic_text(panic).contains("Expected success, got contract error"));
}

fn panic_text(payload: Box<dyn std::any::Any + Send>) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()))
        .unwrap_or_default()
}

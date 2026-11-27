//! Explicit authorization helpers.
//!
//! `Env::mock_all_auths` accepts every `require_auth` in the invocation, so a test
//! written on it passes even after a contract stops checking one of them. The types
//! here declare *which* address may authorize *which* invocation instead, and
//! [`assert_auth_failure`] proves the call actually fails without it.
//!
//! Both halves matter. A declaration nobody tests against is documentation, and a
//! blanket mock is a test that cannot fail for the reason auth exists.

use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};
use soroban_sdk::xdr::{ContractEventBody, ScError, ScVal};
use soroban_sdk::{Address, Env, InvokeError, Val};

use crate::error::{TestError, TestResult};

/// One authorization a test expects a call to require.
///
/// Build it, apply it with [`ExpectedAuth::apply`] or [`expect_auths`], then call
/// the contract. Authorization requests must match one of the entries you register.
pub struct ExpectedAuth<'a> {
    address: &'a Address,
    contract: &'a Address,
    fn_name: &'a str,
    args: soroban_sdk::Vec<Val>,
    sub_invokes: Vec<MockAuthInvoke<'a>>,
}

impl<'a> ExpectedAuth<'a> {
    /// Declare that `address` is expected to authorize `fn_name` on `contract`.
    pub fn new(address: &'a Address, contract: &'a Address, fn_name: &'a str) -> Self {
        Self {
            address,
            contract,
            fn_name,
            args: soroban_sdk::Vec::new(address.env()),
            sub_invokes: Vec::new(),
        }
    }

    /// The arguments the authorized call is made with, when the contract checks them.
    pub fn with_args(mut self, args: soroban_sdk::Vec<Val>) -> Self {
        self.args = args;
        self
    }

    /// A nested invocation the authorized address is also allowed to make.
    pub fn with_sub_invoke(mut self, sub_invoke: MockAuthInvoke<'a>) -> Self {
        self.sub_invokes.push(sub_invoke);
        self
    }

    /// Register this authorization on `env` as the only one available.
    pub fn apply(&self, env: &Env) {
        env.mock_auths(&[MockAuth {
            address: self.address,
            invoke: &MockAuthInvoke {
                contract: self.contract,
                fn_name: self.fn_name,
                args: self.args.clone(),
                sub_invokes: &self.sub_invokes,
            },
        }]);
    }

    /// The declaration in a form a failure message can carry.
    pub fn describe(&self) -> String {
        format!(
            "{:?} may authorize {} on {:?}",
            self.address, self.fn_name, self.contract
        )
    }
}

/// Apply every declared authorization, refusing an empty list.
///
/// An empty list would leave the environment with no credentials at all, which
/// reads as "authorization is broken" rather than "nothing was declared", so it is
/// an error rather than a no-op.
pub fn expect_auths(env: &Env, auths: &[ExpectedAuth<'_>]) -> TestResult<()> {
    if auths.is_empty() {
        return Err(TestError::SetupError(
            "expect_auths was given no expected authorization".to_string(),
        ));
    }
    let invokes: Vec<_> = auths
        .iter()
        .map(|auth| MockAuthInvoke {
            contract: auth.contract,
            fn_name: auth.fn_name,
            args: auth.args.clone(),
            sub_invokes: &auth.sub_invokes,
        })
        .collect();
    let mocks: Vec<_> = auths
        .iter()
        .zip(&invokes)
        .map(|(auth, invoke)| MockAuth {
            address: auth.address,
            invoke,
        })
        .collect();
    env.mock_auths(&mocks);
    Ok(())
}

/// Assert that a call failed for lack of the authorization it should have needed.
///
/// Reads the outcome of `Env::try_invoke_contract` after [`expect_auths`] has been
/// given credentials deliberately short of what the call requires. The assertion uses
/// new host diagnostics to distinguish authorization failures from unrelated aborts.
pub fn assert_auth_failure<T: core::fmt::Debug, C: core::fmt::Debug, E: core::fmt::Debug>(
    env: &Env,
    invoke: impl FnOnce() -> Result<Result<T, C>, Result<E, InvokeError>>,
    declared: &[ExpectedAuth<'_>],
) -> TestResult<()> {
    let expectation = declared
        .iter()
        .map(ExpectedAuth::describe)
        .collect::<Vec<_>>()
        .join("; ");
    let expectation = if expectation.is_empty() {
        "no authorization declared".to_string()
    } else {
        expectation
    };

    let before = env
        .host()
        .get_diagnostic_events()
        .map_err(|error| {
            TestError::SetupError(format!("could not read diagnostic events: {error:?}"))
        })?
        .0
        .len();
    match invoke() {
        Err(Err(InvokeError::Abort)) => {
            let events = env.host().get_diagnostic_events().map_err(|error| {
                TestError::SetupError(format!("could not read diagnostic events: {error:?}"))
            })?;
            let last_error = events.0.iter().skip(before).rev().find_map(|event| {
                let ContractEventBody::V0(body) = &event.event.body;
                match body.topics.as_slice() {
                    [ScVal::Symbol(name), ScVal::Error(error)] if name.0.as_slice() == b"error" => Some(error),
                    _ => None,
                }
            });
            if matches!(last_error, Some(ScError::Auth(_))) {
                return Ok(());
            }
            Err(TestError::AssertionFailed(format!(
                "expected authorization: {expectation} - the call aborted without an authorization error: {last_error:?}"
            )))
        }
        Err(Err(other)) => Err(TestError::AssertionFailed(format!(
            "expected authorization: {expectation} - the host failed with {other:?} rather than an auth abort"
        ))),
        Err(Ok(contract_error)) => Err(TestError::AssertionFailed(format!(
            "expected authorization: {expectation} - the call returned a contract error: {contract_error:?}"
        ))),
        Ok(Err(conversion_error)) => Err(TestError::AssertionFailed(format!(
            "expected authorization: {expectation} - the success value could not be converted: {conversion_error:?}"
        ))),
        Ok(Ok(value)) => Err(TestError::AssertionFailed(format!(
            "expected authorization: {expectation} - the call succeeded, so the contract never required it: {value:?}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn describe_names_address_function_and_contract() {
        let env = Env::default();
        let address = Address::generate(&env);
        let contract = Address::generate(&env);
        let auth = ExpectedAuth::new(&address, &contract, "withdraw");
        let described = auth.describe();
        assert!(described.starts_with(&format!("{address:?}")));
        assert!(described.contains("withdraw"));
        assert!(described.ends_with(&format!("{contract:?}")));
    }

    #[test]
    fn an_empty_declaration_is_an_error() {
        let env = Env::default();
        let error = expect_auths(&env, &[]).unwrap_err();
        assert!(
            matches!(error, TestError::SetupError(message) if message.contains("no expected authorization"))
        );
    }

    #[test]
    fn a_succeeded_call_is_reported_as_never_requiring_auth() {
        let env = Env::default();
        let address = Address::generate(&env);
        let contract = Address::generate(&env);
        let declared = [ExpectedAuth::new(&address, &contract, "ping")];
        let outcome: Result<Result<(), ()>, Result<(), InvokeError>> = Ok(Ok(()));
        let error = assert_auth_failure(&env, || outcome, &declared).unwrap_err();
        assert!(
            matches!(error, TestError::AssertionFailed(message) if message.contains("never required it"))
        );
    }
}

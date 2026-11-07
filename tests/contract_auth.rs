//! Authorization declared explicitly, and the failure when it is missing.
//!
//! Each test states the credential the contract should demand. Nothing is blanket
//! mocked, so a contract that stops calling `require_auth` fails these tests.

use soroban_sdk::testutils::Address as _;
use soroban_sdk::{
    contract, contracterror, contractimpl, symbol_short, Address, ConversionError, Env, IntoVal,
    InvokeError,
};
use soroban_test_suite::prelude::*;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    Insufficient = 1,
}

#[contract]
pub struct Vault;

#[contractimpl]
impl Vault {
    pub fn withdraw(env: Env, from: Address, amount: i128) -> Result<(), VaultError> {
        from.require_auth();
        let held: i128 = env.storage().instance().get(&from).unwrap_or(0);
        if held < amount {
            return Err(VaultError::Insufficient);
        }
        env.storage().instance().set(&from, &(held - amount));
        Ok(())
    }

    pub fn deposit(env: Env, to: Address, amount: i128) {
        let held: i128 = env.storage().instance().get(&to).unwrap_or(0);
        env.storage().instance().set(&to, &(held + amount));
    }
}

type Outcome = Result<Result<(), ConversionError>, Result<VaultError, InvokeError>>;

fn withdraw(env: &Env, id: &Address, from: &Address, amount: i128) -> Outcome {
    env.try_invoke_contract::<(), VaultError>(
        id,
        &symbol_short!("withdraw"),
        (from.clone(), amount).into_val(env),
    )
}

#[test]
fn the_declared_authorization_lets_the_call_through() {
    let env = Env::default();
    let id = env.register_contract(None, Vault);
    let holder = Address::generate(&env);
    env.mock_all_auths();
    VaultClient::new(&env, &id).deposit(&holder, &10);

    let declared = [ExpectedAuth::new(&holder, &id, "withdraw")
        .with_args((holder.clone(), 4_i128).into_val(&env))];
    expect_auths(&env, &declared).expect("declaration applies");

    let outcome = withdraw(&env, &id, &holder, 4);
    assert!(matches!(outcome, Ok(Ok(()))), "{outcome:?}");
}

#[test]
fn no_authorization_fails_and_names_what_was_declared() {
    let env = Env::default();
    let id = env.register_contract(None, Vault);
    let holder = Address::generate(&env);
    env.mock_all_auths();
    VaultClient::new(&env, &id).deposit(&holder, &10);

    let declared = [ExpectedAuth::new(&holder, &id, "withdraw")];
    env.mock_auths(&[]);

    let outcome = withdraw(&env, &id, &holder, 4);
    assert_auth_failure(&outcome, &declared).expect("the call should be an auth failure");
}

#[test]
fn a_different_address_fails_the_same_way_and_the_declaration_names_it() {
    let env = Env::default();
    let id = env.register_contract(None, Vault);
    let holder = Address::generate(&env);
    let stranger = Address::generate(&env);
    env.mock_all_auths();
    VaultClient::new(&env, &id).deposit(&holder, &10);

    // The holder holds the funds, but the call is made by the stranger: the declared
    // authorization does not cover it.
    let declared = [ExpectedAuth::new(&holder, &id, "withdraw")
        .with_args((stranger.clone(), 4_i128).into_val(&env))];
    expect_auths(&env, &declared).expect("declaration applies");

    let outcome = withdraw(&env, &id, &stranger, 4);
    assert!(
        matches!(outcome, Err(Err(InvokeError::Abort))),
        "{outcome:?}"
    );
    assert_auth_failure(&outcome, &declared).expect("an auth failure is still an auth failure");

    let described = declared[0].describe();
    assert!(described.contains(&format!("{holder:?}")), "{described}");
    assert!(described.contains("withdraw"), "{described}");
}

#[test]
fn a_sub_invocation_can_be_declared() {
    use soroban_sdk::testutils::MockAuthInvoke;

    let env = Env::default();
    let id = env.register_contract(None, Vault);
    let holder = Address::generate(&env);
    env.mock_all_auths();
    VaultClient::new(&env, &id).deposit(&holder, &10);

    let nested = MockAuthInvoke {
        contract: &id,
        fn_name: "deposit",
        args: (&holder, 1_i128).into_val(&env),
        sub_invokes: &[],
    };
    let declared = [ExpectedAuth::new(&holder, &id, "withdraw")
        .with_args((holder.clone(), 4_i128).into_val(&env))
        .with_sub_invoke(nested)];
    expect_auths(&env, &declared).expect("declaration applies");

    let outcome = withdraw(&env, &id, &holder, 4);
    assert!(matches!(outcome, Ok(Ok(()))), "{outcome:?}");
}

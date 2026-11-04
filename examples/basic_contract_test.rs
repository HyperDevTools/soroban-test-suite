use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};
use soroban_test_suite::prelude::*;
use soroban_test_suite::{assert_contract_eq, assert_contract_gt, assert_contract_gte};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct BasicCounterContract;

#[contractimpl]
impl BasicCounterContract {
    pub fn increment(env: Env, _caller: Address, delta: u32) -> u32 {
        let current: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let updated = current + delta;
        env.storage().instance().set(&COUNTER, &updated);
        updated
    }
}

fn main() {
    let env = Env::default();
    let mut test = TestContext::new(&env);
    let account = test.create_account();
    let extra_accounts = test.create_accounts(2);
    assert_contract_eq!(extra_accounts.len(), 2);

    let contract_id = env.register_contract(None, BasicCounterContract);
    let client = BasicCounterContractClient::new(&env, &contract_id);

    let mut tracker = GasTracker::new(&env);
    tracker.start().expect("gas tracking should start");
    let count = client.increment(&account, &5);
    let usage = tracker.stop().expect("gas tracking should stop");

    let time = TimeController::new(&env);
    time.set(1_000);
    time.advance(60);

    let mock_oracle = MockContract::new(&env);

    assert!(!account.to_string().is_empty());
    assert!(!mock_oracle.address().to_string().is_empty());
    assert_contract_eq!(count, 5);
    assert_contract_gt!(usage.cpu_instructions, 0);
    assert_contract_gte!(time.now(), 1_060);
}

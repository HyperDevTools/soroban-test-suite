use soroban_sdk::{testutils::Address as _, Env};
use soroban_test_suite::prelude::*;

fn main() {
    let env = Env::default();
    let mut test = TestContext::new(&env);
    let account = test.create_account();
    let batch = test.create_accounts(2);
    assert_contract_eq!(batch.len(), 2, "batch should contain 2 accounts");
    assert_contract_eq!(test.accounts().len(), 3, "context should track 3 accounts");

    // Measure budget usage across an interval
    let mut tracker = GasTracker::new(&env);
    tracker.start().expect("gas tracking should start");
    let _another_account = soroban_sdk::Address::generate(&env);
    let usage = tracker.stop().expect("gas tracking should stop");

    // Exercise TimeController and BasicFixture
    let time = TimeController::new(&env);
    time.set(1_700_000_000);
    time.advance(60);
    assert_contract_eq!(time.now(), 1_700_000_060, "timestamp should advance by 60s");

    let fixture = BasicFixture::new(&env, 2);
    assert_contract_eq!(fixture.users.len(), 2, "fixture should create 2 users");
    assert_ne!(&fixture.admin, fixture.user(0));

    assert!(!account.to_string().is_empty());
    assert_contract_gt!(usage.cpu_instructions, 0, "cpu instructions should be > 0");
    assert_contract_gte!(usage.memory_bytes, 1, "memory bytes should be >= 1");
}

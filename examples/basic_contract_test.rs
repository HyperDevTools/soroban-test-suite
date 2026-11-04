use soroban_sdk::{testutils::Address as _, Env};
use soroban_test_suite::prelude::*;

fn main() {
    let env = Env::default();
    let mut test = TestContext::new(&env);
    let account = test.create_account();

    let mut tracker = GasTracker::new(&env);
    tracker.start().expect("gas tracking should start");
    let _another_account = soroban_sdk::Address::generate(&env);
    let usage = tracker.stop().expect("gas tracking should stop");

    assert!(!account.to_string().is_empty());
    assert!(usage.cpu_instructions > 0);
}

use soroban_sdk::{contract, contractimpl, testutils::EnvTestConfig, Env};
use soroban_test_suite::prelude::*;

#[contract]
struct Deadline;

#[contractimpl]
impl Deadline {
    pub fn has_elapsed(env: Env, deadline: u64) -> bool {
        env.ledger().timestamp() >= deadline
    }
}

fn main() {
    let env = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    let mut test = TestContext::new(&env);
    let account = test.create_account();
    let batch = test.create_accounts(2);
    assert_contract_eq!(batch.len(), 2, "batch should contain 2 accounts");
    assert_contract_eq!(test.accounts().len(), 3, "context should track 3 accounts");

    let contract_id = env.register_contract(None, Deadline);
    let client = DeadlineClient::new(&env, &contract_id);
    let time = TimeController::new(&env);
    let deadline = 1_700_000_060;
    // Set deterministic ledger time before invoking the registered test contract.
    time.set(deadline - 60);
    assert!(!client.has_elapsed(&deadline));
    time.advance(60).expect("ledger timestamp should advance");

    let mut tracker = GasTracker::new(&env);
    tracker.start().expect("gas tracking should start");
    assert!(client.has_elapsed(&deadline));
    let usage = tracker.stop().expect("gas tracking should stop");
    usage
        .assert_within(BudgetLimits {
            max_cpu_instructions: Some(1_000_000),
            max_memory_bytes: Some(100_000),
        })
        .expect("deadline check should stay within its test budget");

    let fixture = BasicFixture::new(&env, 2);
    assert_contract_eq!(fixture.users.len(), 2, "fixture should create 2 users");
    assert_ne!(&fixture.admin, fixture.user(0));

    assert!(!account.to_string().is_empty());
    assert_contract_gt!(usage.cpu_instructions, 0, "cpu instructions should be > 0");
    assert_contract_gte!(usage.memory_bytes, 1, "memory bytes should be >= 1");
}

#[test]
fn deadline_and_budget_example_runs() {
    main();
}

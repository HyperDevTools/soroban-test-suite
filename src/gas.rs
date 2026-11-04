use soroban_sdk::Env;

use crate::error::{TestError, TestResult};

/// Soroban budget usage measured during a tracked interval.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GasUsage {
    /// CPU instructions consumed.
    pub cpu_instructions: u64,
    /// Memory bytes consumed.
    pub memory_bytes: u64,
}

#[derive(Clone, Copy)]
struct BudgetSnapshot {
    cpu_instructions: u64,
    memory_bytes: u64,
}

/// Track Soroban CPU and memory budget usage during test execution.
///
/// The measurements come from Soroban's test budget. As with the SDK's budget
/// counters, native Rust execution can underestimate the equivalent WASM cost.
pub struct GasTracker<'a> {
    env: &'a Env,
    start: Option<BudgetSnapshot>,
}

impl<'a> GasTracker<'a> {
    /// Create a new gas tracker
    pub fn new(env: &'a Env) -> Self {
        Self { env, start: None }
    }

    /// Start tracking gas usage
    pub fn start(&mut self) -> TestResult<()> {
        if self.start.is_some() {
            return Err(TestError::InvalidState(
                "gas tracking has already started".to_string(),
            ));
        }

        self.start = Some(self.snapshot());
        Ok(())
    }

    /// Stop tracking and return CPU instruction and memory usage for the interval.
    pub fn stop(&mut self) -> TestResult<GasUsage> {
        let start = self
            .start
            .ok_or_else(|| TestError::InvalidState("gas tracking has not started".to_string()))?;
        let current = self.snapshot();
        let usage = GasUsage {
            cpu_instructions: current
                .cpu_instructions
                .checked_sub(start.cpu_instructions)
                .ok_or_else(|| {
                    TestError::InvalidState(
                        "Soroban CPU budget was reset during gas tracking".to_string(),
                    )
                })?,
            memory_bytes: current
                .memory_bytes
                .checked_sub(start.memory_bytes)
                .ok_or_else(|| {
                    TestError::InvalidState(
                        "Soroban memory budget was reset during gas tracking".to_string(),
                    )
                })?,
        };
        self.start = None;
        Ok(usage)
    }

    /// Discard any active measurement and return the tracker to its initial state.
    pub fn reset(&mut self) {
        self.start = None;
    }

    fn snapshot(&self) -> BudgetSnapshot {
        let budget = self.env.budget();
        BudgetSnapshot {
            cpu_instructions: budget.cpu_instruction_cost(),
            memory_bytes: budget.memory_bytes_cost(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn measures_budget_consumed_during_interval() {
        let env = Env::default();
        let mut tracker = GasTracker::new(&env);

        tracker.start().unwrap();
        let _account = soroban_sdk::Address::generate(&env);
        let usage = tracker.stop().unwrap();

        assert!(usage.cpu_instructions > 0);
        assert!(usage.memory_bytes > 0);
    }

    #[test]
    fn stop_without_start_returns_an_error() {
        let env = Env::default();
        let mut tracker = GasTracker::new(&env);

        assert!(matches!(tracker.stop(), Err(TestError::InvalidState(_))));
    }

    #[test]
    fn starting_twice_returns_an_error() {
        let env = Env::default();
        let mut tracker = GasTracker::new(&env);

        tracker.start().unwrap();
        assert!(matches!(tracker.start(), Err(TestError::InvalidState(_))));
    }

    #[test]
    fn reset_discards_the_active_measurement() {
        let env = Env::default();
        let mut tracker = GasTracker::new(&env);

        tracker.start().unwrap();
        tracker.reset();

        assert!(matches!(tracker.stop(), Err(TestError::InvalidState(_))));
        tracker.start().unwrap();
        assert!(tracker.stop().is_ok());
    }
}

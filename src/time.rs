use soroban_sdk::{testutils::Ledger as _, Env};

use crate::error::{TestError, TestResult};

/// Control time in contract tests
pub struct TimeController<'a> {
    env: &'a Env,
}

impl<'a> TimeController<'a> {
    /// Create a new time controller
    pub fn new(env: &'a Env) -> Self {
        Self { env }
    }

    /// Get current ledger timestamp
    pub fn now(&self) -> u64 {
        self.env.ledger().timestamp()
    }

    /// Advance the timestamp, or return an error and leave the ledger unchanged on overflow.
    pub fn advance(&self, seconds: u64) -> TestResult<()> {
        let timestamp = self.now().checked_add(seconds).ok_or_else(|| {
            TestError::InvalidState("ledger timestamp would overflow".to_string())
        })?;
        self.env.ledger().set_timestamp(timestamp);
        Ok(())
    }

    /// Set time to a specific timestamp
    pub fn set(&self, timestamp: u64) {
        self.env.ledger().set_timestamp(timestamp);
    }

    /// Advance the sequence by one and the timestamp by five seconds.
    /// Both values stay unchanged if either would overflow.
    pub fn next_ledger(&self) -> TestResult<()> {
        let ledger = self.env.ledger();
        let sequence = ledger
            .sequence()
            .checked_add(1)
            .ok_or_else(|| TestError::InvalidState("ledger sequence would overflow".to_string()))?;
        let timestamp = self.now().checked_add(5).ok_or_else(|| {
            TestError::InvalidState("ledger timestamp would overflow".to_string())
        })?;
        ledger.with_mut(|info| {
            info.sequence_number = sequence;
            info.timestamp = timestamp;
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_controller_advance() {
        let env = Env::default();
        let controller = TimeController::new(&env);

        let start_time = controller.now();
        controller.advance(100).unwrap();
        let end_time = controller.now();

        assert_eq!(end_time, start_time + 100);
    }

    #[test]
    fn test_time_controller_set() {
        let env = Env::default();
        let controller = TimeController::new(&env);

        controller.set(1000);
        assert_eq!(controller.now(), 1000);
    }

    #[test]
    fn test_next_ledger() {
        let env = Env::default();
        let controller = TimeController::new(&env);

        let start_time = controller.now();
        let start_ledger = env.ledger().sequence();

        controller.next_ledger().unwrap();

        assert_eq!(env.ledger().sequence(), start_ledger + 1);
        assert_eq!(controller.now(), start_time + 5);
    }

    #[test]
    fn advance_rejects_timestamp_overflow_without_changing_time() {
        let env = Env::default();
        let controller = TimeController::new(&env);
        controller.set(u64::MAX - 1);

        controller.advance(1).unwrap();
        assert!(matches!(
            controller.advance(1),
            Err(TestError::InvalidState(_))
        ));
        assert_eq!(controller.now(), u64::MAX);
    }

    #[test]
    fn next_ledger_is_atomic_if_timestamp_or_sequence_would_overflow() {
        let env = Env::default();
        let controller = TimeController::new(&env);
        for (sequence, timestamp) in [(u32::MAX, 100), (10, u64::MAX - 4)] {
            env.ledger().set_sequence_number(sequence);
            controller.set(timestamp);

            assert!(matches!(
                controller.next_ledger(),
                Err(TestError::InvalidState(_))
            ));
            assert_eq!(env.ledger().sequence(), sequence);
            assert_eq!(controller.now(), timestamp);
        }
    }
}

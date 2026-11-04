use soroban_sdk::{testutils::Ledger as _, Env};

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

    /// Advance time by the specified number of seconds
    pub fn advance(&self, seconds: u64) {
        let timestamp = self.now().saturating_add(seconds);
        self.env.ledger().set_timestamp(timestamp);
    }

    /// Set time to a specific timestamp
    pub fn set(&self, timestamp: u64) {
        self.env.ledger().set_timestamp(timestamp);
    }

    /// Advance to the next ledger
    pub fn next_ledger(&self) {
        let ledger = self.env.ledger();
        ledger.set_sequence_number(ledger.sequence().saturating_add(1));
        ledger.set_timestamp(self.now().saturating_add(5));
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
        controller.advance(100);
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

        controller.next_ledger();

        assert_eq!(env.ledger().sequence(), start_ledger + 1);
        assert_eq!(controller.now(), start_time + 5);
    }
}

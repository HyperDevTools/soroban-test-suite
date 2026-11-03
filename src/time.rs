use soroban_sdk::Env;

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
        self.env.ledger().with_mut(|li| {
            li.timestamp = li.timestamp.saturating_add(seconds);
        });
    }

    /// Set time to a specific timestamp
    pub fn set(&self, timestamp: u64) {
        self.env.ledger().with_mut(|li| {
            li.timestamp = timestamp;
        });
    }

    /// Advance to the next ledger
    pub fn next_ledger(&self) {
        self.env.ledger().with_mut(|li| {
            li.sequence_number = li.sequence_number.saturating_add(1);
            li.timestamp = li.timestamp.saturating_add(5); // Assume 5 second ledger time
        });
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

use soroban_sdk::{testutils::Address as _, Address, Env};

/// TestContext provides a convenient environment for testing Soroban contracts
pub struct TestContext<'a> {
    env: &'a Env,
    accounts: Vec<Address>,
}

impl<'a> TestContext<'a> {
    /// Create a new test context
    pub fn new(env: &'a Env) -> Self {
        Self {
            env,
            accounts: Vec::new(),
        }
    }

    /// Create a test account
    pub fn create_account(&mut self) -> Address {
        let account = Address::generate(self.env);
        self.accounts.push(account.clone());
        account
    }

    /// Get the environment
    pub fn env(&self) -> &Env {
        self.env
    }

    /// Get all created accounts
    pub fn accounts(&self) -> &[Address] {
        &self.accounts
    }

    /// Create multiple accounts at once
    pub fn create_accounts(&mut self, count: usize) -> Vec<Address> {
        (0..count).map(|_| self.create_account()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let env = Env::default();
        let mut context = TestContext::new(&env);

        let account = context.create_account();
        assert_eq!(context.accounts().len(), 1);
        assert_eq!(context.accounts()[0], account);
    }

    #[test]
    fn test_create_multiple_accounts() {
        let env = Env::default();
        let mut context = TestContext::new(&env);

        let accounts = context.create_accounts(3);
        assert_eq!(accounts.len(), 3);
        assert_eq!(context.accounts().len(), 3);
    }
}

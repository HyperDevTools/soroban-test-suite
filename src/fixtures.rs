/// Test fixtures for common testing patterns

use soroban_sdk::{Address, Env};

/// A basic test fixture with common setup
pub struct BasicFixture<'a> {
    pub env: &'a Env,
    pub admin: Address,
    pub users: Vec<Address>,
}

impl<'a> BasicFixture<'a> {
    /// Create a new basic fixture with an admin and specified number of users
    pub fn new(env: &'a Env, num_users: usize) -> Self {
        let admin = Address::generate(env);
        let users: Vec<Address> = (0..num_users).map(|_| Address::generate(env)).collect();

        Self { env, admin, users }
    }

    /// Get a user by index
    pub fn user(&self, index: usize) -> &Address {
        &self.users[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_fixture() {
        let env = Env::default();
        let fixture = BasicFixture::new(&env, 3);

        assert_eq!(fixture.users.len(), 3);
        assert_ne!(fixture.admin, fixture.users[0]);
    }

    #[test]
    fn test_fixture_user_access() {
        let env = Env::default();
        let fixture = BasicFixture::new(&env, 2);

        let user0 = fixture.user(0);
        let user1 = fixture.user(1);

        assert_ne!(user0, user1);
    }
}

//! Mock contract placeholder.

/// Placeholder for mock contract functionality
pub struct MockContract {
    id: String,
}

impl MockContract {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_contract_creation() {
        let mock = MockContract::new("test_contract".to_string());
        assert_eq!(mock.id(), "test_contract");
    }
}

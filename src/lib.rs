//! Soroban Test Suite - A comprehensive testing framework for Soroban smart contracts
//!
//! This library provides utilities and helpers for testing Soroban smart contracts,
//! including assertions, mocking, gas tracking, and time manipulation.

pub mod assertions;
pub mod context;
pub mod error;
pub mod fixtures;
pub mod gas;
pub mod mock;
pub mod time;

/// Re-export commonly used types and traits
pub mod prelude {
    pub use crate::context::TestContext;
    pub use crate::error::TestError;
    pub use crate::fixtures::*;
    pub use crate::gas::{GasTracker, GasUsage};
    pub use crate::time::TimeController;
    pub use crate::{
        assert_contract_eq, assert_contract_err, assert_contract_gt, assert_contract_gte,
        assert_contract_ok,
    };
}

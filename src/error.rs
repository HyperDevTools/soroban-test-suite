use thiserror::Error;

#[derive(Error, Debug)]
pub enum TestError {
    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

    #[error("Contract error: {0}")]
    ContractError(String),

    #[error("Setup error: {0}")]
    SetupError(String),

    #[error("Mock error: {0}")]
    MockError(String),

    #[error("Timeout: operation took too long")]
    Timeout,

    #[error("Invalid test state: {0}")]
    InvalidState(String),
}

pub type TestResult<T> = Result<T, TestError>;

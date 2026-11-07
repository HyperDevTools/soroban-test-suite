//! Assertion utilities for Soroban contract testing

/// Assert that two values are equal with a custom message
#[macro_export]
macro_rules! assert_contract_eq {
    ($left:expr, $right:expr, $msg:expr) => {
        if $left != $right {
            panic!(
                "Assertion failed: {} - Expected: {:?}, Got: {:?}",
                $msg, $right, $left
            );
        }
    };
    ($left:expr, $right:expr) => {
        assert_contract_eq!($left, $right, "Values should be equal")
    };
}

/// Assert that a value is greater than another
#[macro_export]
macro_rules! assert_contract_gt {
    ($left:expr, $right:expr, $msg:expr) => {
        if $left <= $right {
            panic!(
                "Assertion failed: {} - Expected {} > {}, but {} <= {}",
                $msg,
                stringify!($left),
                stringify!($right),
                $left,
                $right
            );
        }
    };
    ($left:expr, $right:expr) => {
        assert_contract_gt!($left, $right, "Value should be greater")
    };
}

/// Assert that a value is greater than or equal to another
#[macro_export]
macro_rules! assert_contract_gte {
    ($left:expr, $right:expr, $msg:expr) => {
        if $left < $right {
            panic!(
                "Assertion failed: {} - Expected {} >= {}, but {} < {}",
                $msg,
                stringify!($left),
                stringify!($right),
                $left,
                $right
            );
        }
    };
    ($left:expr, $right:expr) => {
        assert_contract_gte!($left, $right, "Value should be greater or equal")
    };
}

/// Assert that a contract call fails with a particular typed error.
///
/// The expression is evaluated exactly once, so a call with side effects is safe
/// to assert on. An unexpected success and a *different* error are separate
/// failures with their own diagnostics, because "it failed" is not the same
/// answer as "it failed the way the test is about".
///
/// The expectation is a pattern, not a string, so it is compared by type: pass
/// `MyError::Unauthorized`, or the same with a `$msg` for the panic text.
///
/// `Env::try_invoke_contract` answers `Err(Ok(e))` when the contract failed and
/// `Err(Err(..))` when the host could not convert that error, so both arms are
/// covered and the second is reported as a host failure rather than being folded
/// into the contract-error arm.
///
/// ```ignore
/// let outcome = env.try_invoke_contract::<(), MyError>(
///     &contract_id,
///     &symbol_short!("withdraw"),
///     vec![&env, amount.into_val(&env)],
/// );
/// assert_contract_err!(outcome, MyError::Unauthorized, "unfunded caller is rejected");
/// ```
#[macro_export]
macro_rules! assert_contract_err {
    ($call:expr, $expected:pat, $msg:expr) => {{
        let message = $msg;
        match $call {
            Ok(Ok(_)) => panic!(
                "Assertion failed: {} - Expected {}, but the call succeeded",
                message,
                stringify!($expected)
            ),
            Err(Ok(actual)) => {
                if !matches!(actual, $expected) {
                    panic!(
                        "Assertion failed: {} - Expected {}, got {:?}",
                        message,
                        stringify!($expected),
                        actual
                    );
                }
            }
            Err(Err(host_error)) => panic!(
                "Assertion failed: {} - Expected {}, the host error could not be \
                 converted into that error type: {:?}",
                message,
                stringify!($expected),
                host_error
            ),
            Ok(Err(conversion_error)) => panic!(
                "Assertion failed: {} - Expected {}, the success value could not be \
                 converted: {:?}",
                message,
                stringify!($expected),
                conversion_error
            ),
        }
    }};
    ($call:expr, $expected:pat) => {
        assert_contract_err!($call, $expected, "Contract call should fail")
    };
}

/// Assert that a contract call succeeds and return what it produced.
///
/// The counterpart to [`assert_contract_err!`]: it fails on a contract error or a
/// host error and hands back the value, so a test reads as the shape it expects
/// rather than unwrapping two nested `Result`s by hand.
#[macro_export]
macro_rules! assert_contract_ok {
    ($call:expr) => {{
        match $call {
            Ok(Ok(value)) => value,
            Err(Ok(contract_error)) => {
                panic!("Assertion failed: Expected success, got contract error {:?}", contract_error)
            }
            Err(Err(host_error)) => {
                panic!("Assertion failed: Expected success, host error could not be converted: {:?}", host_error)
            }
            Ok(Err(conversion_error)) => {
                panic!("Assertion failed: Expected success, value could not be converted: {:?}", conversion_error)
            }
        }
    }};
    ($call:expr, $msg:expr) => {{
        let message = $msg;
        match $call {
            Ok(Ok(value)) => value,
            Err(Ok(contract_error)) => {
                panic!("Assertion failed: {} - Expected success, got contract error {:?}", message, contract_error)
            }
            Err(Err(host_error)) => {
                panic!("Assertion failed: {} - Expected success, host error could not be converted: {:?}", message, host_error)
            }
            Ok(Err(conversion_error)) => {
                panic!("Assertion failed: {} - Expected success, value could not be converted: {:?}", message, conversion_error)
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assert_contract_eq() {
        assert_contract_eq!(5, 5);
        assert_contract_eq!(5, 5, "Numbers should match");
    }

    #[test]
    fn test_assert_contract_gt() {
        assert_contract_gt!(10, 5);
        assert_contract_gt!(10, 5, "10 should be greater than 5");
    }

    #[test]
    fn test_assert_contract_gte() {
        assert_contract_gte!(10, 5);
        assert_contract_gte!(5, 5);
        assert_contract_gte!(10, 5, "10 should be >= 5");
    }

    #[test]
    #[should_panic]
    fn test_assert_contract_eq_fails() {
        assert_contract_eq!(5, 10);
    }

    #[test]
    #[should_panic]
    fn test_assert_contract_gt_fails() {
        assert_contract_gt!(5, 10);
    }

    #[test]
    #[should_panic]
    fn test_assert_contract_gte_fails() {
        assert_contract_gte!(5, 10);
    }
}

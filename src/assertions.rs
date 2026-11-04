/// Assertion utilities for Soroban contract testing

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

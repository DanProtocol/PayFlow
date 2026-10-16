use soroban_sdk::{token, Address, Env};

use crate::errors::ContractError;
use crate::Subscription;

/// Verifies that `user` has granted the contract an allowance of at least
/// `min_amount` for `token`. Panics with `ContractError::InsufficientAllowance`
/// if the check fails.
pub fn check_allowance(env: &Env, user: &Address, token: &Address, min_amount: i128) {
    let client = token::Client::new(env, token);
    let allowance = client.allowance(user, &env.current_contract_address());
    if allowance < min_amount {
        env.panic_with_error(ContractError::InsufficientAllowance);
    }
}

/// Composable helper that asserts a subscription is ready to be used:
/// the subscription must be active and the user must have sufficient
/// allowance for the subscription's token and amount.
pub fn validate_subscription_readiness(env: &Env, user: &Address, sub: &Subscription) {
    if !sub.active {
        env.panic_with_error(ContractError::SubscriptionNotActive);
    }
    check_allowance(env, user, &sub.token, sub.amount);
}

/// Validates that `new_amount` is a legal subscription amount: must be positive
/// and must not exceed `MAX_SUBSCRIPTION_AMOUNT`. Panics with the appropriate
/// `ContractError` variant on failure.
pub fn require_valid_amount(env: &Env, new_amount: i128) {
    if new_amount <= 0 {
        env.panic_with_error(ContractError::AmountMustBePositive);
    }
    if new_amount > crate::MAX_SUBSCRIPTION_AMOUNT {
        env.panic_with_error(ContractError::AmountExceedsMaximum);
    }
}

/// Validates that `new_interval` is a legal subscription interval: must be
/// strictly greater than zero. Panics with `ContractError::IntervalTooShort`
/// if the floor is not met.
pub fn require_valid_interval(env: &Env, new_interval: u64) {
    if new_interval == 0 {
        env.panic_with_error(ContractError::IntervalTooShort);
    }
}

pub fn require_positive_amount(env: &Env, amount: i128) {
    if amount <= 0 {
        env.panic_with_error(ContractError::AmountMustBePositive);
    }
}

pub fn require_positive_interval(env: &Env, interval: u64) {
    if interval == 0 {
        env.panic_with_error(ContractError::IntervalMustBePositive);
    }
}

pub fn require_active_subscription(env: &Env, active: bool) {
    if !active {
        env.panic_with_error(ContractError::SubscriptionNotActive);
    }
}

pub fn require_charge_interval_elapsed(env: &Env, now: u64, last_charged: u64, interval: u64) {
    if now < last_charged + interval {
        env.panic_with_error(ContractError::IntervalNotElapsed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_positive_amount_accepts_positive() {
        let env = soroban_sdk::Env::default();
        require_positive_amount(&env, 1);
        require_positive_amount(&env, 100);
    }

    #[test]
    #[should_panic]
    fn test_require_positive_amount_panics_on_zero() {
        let env = soroban_sdk::Env::default();
        require_positive_amount(&env, 0);
    }

    #[test]
    #[should_panic]
    fn test_require_positive_amount_negative_signed() {
        let env = soroban_sdk::Env::default();
        require_positive_amount(&env, -5);
    }

    #[test]
    fn test_require_positive_interval_accepts_positive() {
        let env = soroban_sdk::Env::default();
        require_positive_interval(&env, 1);
        require_positive_interval(&env, 60);
    }

    #[test]
    #[should_panic]
    fn test_require_positive_interval_panics_on_zero() {
        let env = soroban_sdk::Env::default();
        require_positive_interval(&env, 0);
    }

    #[test]
    fn test_require_active_subscription_accepts_true() {
        let env = soroban_sdk::Env::default();
        require_active_subscription(&env, true);
    }

    #[test]
    #[should_panic]
    fn test_require_active_subscription_panics_on_false() {
        let env = soroban_sdk::Env::default();
        require_active_subscription(&env, false);
    }

    #[test]
    fn test_require_charge_interval_elapsed_accepts_elapsed_interval() {
        let env = soroban_sdk::Env::default();
        require_charge_interval_elapsed(&env, 100, 40, 60);
        require_charge_interval_elapsed(&env, 150, 40, 60);
    }

    #[test]
    #[should_panic]
    fn test_require_charge_interval_elapsed_panics_if_too_early() {
        let env = soroban_sdk::Env::default();
        require_charge_interval_elapsed(&env, 99, 40, 60);
    }
}

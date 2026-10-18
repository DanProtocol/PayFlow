// Additional tests for CancelResult expansion (Issue 3)
// These tests should be integrated into contract/src/test.rs after the PRs are merged

#[cfg(test)]
mod batch_cancel_tests {
    use super::*;

    #[test]
    fn test_batch_cancel_distinguishes_paused_cancellation() {
        let env = Env::default();
        env.mock_all_auths();

        let token = create_token(&env, &Address::generate(&env));
        let contract = create_contract(&env, &token.address);
        
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let merchant = Address::generate(&env);

        contract.initialize(&token.address);
        contract.propose_admin(&admin);
        contract.accept_admin();
        
        // Whitelist merchant
        contract.add_merchant(&merchant);
        
        // Subscribe
        contract.subscribe(
            &user,
            &merchant,
            &100,
            &86400, // 1 day interval
            &None,
            &None,
            &None,
        );
        
        // Pause subscription
        contract.pause(&user);
        
        // Batch cancel - should return PausedCancelled
        let results = contract.batch_cancel(&vec![&env, user.clone()]);
        
        assert_eq!(results.len(), 1);
        assert_eq!(results.get(0).unwrap(), CancelResult::PausedCancelled);
        
        // Verify subscription is now cancelled
        let sub = contract.get_subscription(&user);
        assert_eq!(sub.active, false);
    }

    #[test]
    fn test_batch_cancel_distinguishes_frozen_merchant() {
        let env = Env::default();
        env.mock_all_auths();

        let token = create_token(&env, &Address::generate(&env));
        let contract = create_contract(&env, &token.address);
        
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let merchant = Address::generate(&env);

        contract.initialize(&token.address);
        contract.propose_admin(&admin);
        contract.accept_admin();
        
        // Whitelist merchant
        contract.add_merchant(&merchant);
        
        // Subscribe
        contract.subscribe(
            &user,
            &merchant,
            &100,
            &86400,
            &None,
            &None,
            &None,
        );
        
        // Freeze merchant
        contract.freeze_merchant(&merchant);
        
        // Batch cancel - should return MerchantFrozen
        let results = contract.batch_cancel(&vec![&env, user.clone()]);
        
        assert_eq!(results.len(), 1);
        assert_eq!(results.get(0).unwrap(), CancelResult::MerchantFrozen);
        
        // Verify subscription is now cancelled
        let sub = contract.get_subscription(&user);
        assert_eq!(sub.active, false);
    }

    #[test]
    fn test_batch_cancel_frozen_merchant_takes_precedence_over_paused() {
        let env = Env::default();
        env.mock_all_auths();

        let token = create_token(&env, &Address::generate(&env));
        let contract = create_contract(&env, &token.address);
        
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let merchant = Address::generate(&env);

        contract.initialize(&token.address);
        contract.propose_admin(&admin);
        contract.accept_admin();
        
        // Whitelist merchant
        contract.add_merchant(&merchant);
        
        // Subscribe
        contract.subscribe(
            &user,
            &merchant,
            &100,
            &86400,
            &None,
            &None,
            &None,
        );
        
        // Pause subscription
        contract.pause(&user);
        
        // Freeze merchant
        contract.freeze_merchant(&merchant);
        
        // Batch cancel - should return MerchantFrozen (takes precedence)
        let results = contract.batch_cancel(&vec![&env, user.clone()]);
        
        assert_eq!(results.len(), 1);
        assert_eq!(results.get(0).unwrap(), CancelResult::MerchantFrozen);
        
        // Verify subscription is now cancelled
        let sub = contract.get_subscription(&user);
        assert_eq!(sub.active, false);
    }
}

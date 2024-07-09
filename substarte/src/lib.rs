#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod my_contract {
    #[ink(storage)]
    pub struct MyContract {
        owner: AccountId,
        locked_balances: ink::storage::Mapping<AccountId, Balance>,
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            Self {
                owner,
                locked_balances: Default::default(),
            }
        }

        #[ink(message)]
        pub fn lock_tokens(&mut self, amount: Balance) -> bool {
            let caller = self.env().caller();
            if amount == 0 {
                return false;
            }
            let caller_balance = self.locked_balances.get(&caller).unwrap_or(0);
            self.locked_balances.insert(caller, &(caller_balance + amount));
            true
        }

        #[ink(message)]
        pub fn get_locked_balance(&self, user: AccountId) -> Balance {
            self.locked_balances.get(&user).unwrap_or(0)
        }

        #[ink(message)]
        pub fn unlock_tokens(&mut self, amount: Balance) -> bool {
            let caller = self.env().caller();
            let caller_balance = self.locked_balances.get(&caller).unwrap_or(0);
            if amount == 0 || caller_balance < amount {
                return false;
            }
            self.locked_balances.insert(caller, &(caller_balance - amount));
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ink::env::test;

    #[ink::test]
    fn test_lock_tokens() {
        let mut contract = MyContract::new();
        let caller = test::default_accounts().alice;

        test::set_caller::<ink::env::DefaultEnvironment>(caller);
        assert_eq!(contract.lock_tokens(100), true);
        assert_eq!(contract.get_locked_balance(caller), 100);
    }

    #[ink::test]
    fn test_unlock_tokens() {
        let mut contract = MyContract::new();
        let caller = test::default_accounts().alice;

        test::set_caller::<ink::env::DefaultEnvironment>(caller);
        assert_eq!(contract.lock_tokens(100), true);
        assert_eq!(contract.unlock_tokens(50), true);
        assert_eq!(contract.get_locked_balance(caller), 50);
    }
}

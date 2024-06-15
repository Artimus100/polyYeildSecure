#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod mint_contract {
    #[ink(storage)]
    pub struct MintContract {
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
    }

    impl MintContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Default::default(),
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, amount: Balance) {
            let balance = self.balances.entry(to).or_insert(0);
            *balance += amount;
        }

        #[ink(message)]
        pub fn get_balance(&self, account: AccountId) -> Balance {
            self.balances.get(&account).copied().unwrap_or(0)
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod cross_chain_transfer {
    #[ink(storage)]
    pub struct CrossChainTransfer {
        locked_assets: ink_storage::collections::HashMap<AccountId, Balance>,
    }

    impl CrossChainTransfer {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                locked_assets: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn lock_asset(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let current_balance = self.locked_assets.get(&caller).copied().unwrap_or(0);
            self.locked_assets.insert(caller, current_balance + amount);
            self.env().emit_event(AssetLocked {
                account: caller,
                amount,
            });
        }

        #[ink(message)]
        pub fn unlock_asset(&mut self, account: AccountId, amount: Balance) -> bool {
            let current_balance = self.locked_assets.get(&account).copied().unwrap_or(0);
            if current_balance < amount {
                return false;
            }
            self.locked_assets.insert(account, current_balance - amount);
            self.env().transfer(account, amount).unwrap_or_else(|err| {
                ink_env::debug_println!("Failed to transfer: {:?}", err);
                panic!("Failed to transfer")
            });
            true
        }
    }

    #[ink(event)]
    pub struct AssetLocked {
        #[ink(topic)]
        account: AccountId,
        amount: Balance,
    }
}

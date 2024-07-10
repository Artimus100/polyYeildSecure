#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TokensLocked(T::AccountId, BalanceOf<T>),
        TokensUnlocked(T::AccountId, BalanceOf<T>),
    }

    #[pallet::storage]
    #[pallet::getter(fn locked_balances)]
    pub type LockedBalances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn lock_tokens(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            <LockedBalances<T>>::insert(&who, amount);
            Self::deposit_event(Event::TokensLocked(who, amount));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn unlock_tokens(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let balance = <LockedBalances<T>>::get(&who);
            ensure!(balance >= amount, "Insufficient balance");
            <LockedBalances<T>>::insert(&who, balance - amount);
            Self::deposit_event(Event::TokensUnlocked(who, amount));
            Ok(())
        }
    }
}

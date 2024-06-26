#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn locked_assets)]
    pub type LockedAssets<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AssetLocked(T::AccountId, u64),
        AssetUnlocked(T::AccountId, u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        AssetNotLocked,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn lock_asset(origin: OriginFor<T>, amount: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;

            T::Currency::reserve(&who, amount.into())
                .map_err(|_| Error::<T>::InsufficientBalance)?;

            <LockedAssets<T>>::insert(&who, amount);

            Self::deposit_event(Event::AssetLocked(who, amount));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn unlock_asset(origin: OriginFor<T>, recipient: T::AccountId, amount: u64) -> DispatchResult {
            let _ = ensure_root(origin)?;

            ensure!(<LockedAssets<T>>::contains_key(&recipient), Error::<T>::AssetNotLocked);

            T::Currency::unreserve(&recipient, amount.into());

            <LockedAssets<T>>::remove(&recipient);

            Self::deposit_event(Event::AssetUnlocked(recipient, amount));
            Ok(())
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        YieldUpdated(T::AccountId, u128),
    }

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn update_yield(origin: OriginFor<T>, new_yield: u128) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::deposit_event(Event::YieldUpdated(who, new_yield));
            Ok(())
        }
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);
}

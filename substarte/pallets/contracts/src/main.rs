#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An event emitted when a cross-chain message is sent.
        CrossChainMessageSent(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error emitted when the message length exceeds the limit.
        MessageTooLong,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn send_message(
            origin: OriginFor<T>,
            to_chain: Vec<u8>,
            message: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Ensure the message is not too long.
            ensure!(message.len() <= 256, Error::<T>::MessageTooLong);

            // Emit an event for cross-chain message sending.
            Self::deposit_event(Event::CrossChainMessageSent(sender, message));

            Ok(())
        }
    }
}

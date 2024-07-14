// Ensure this file has the `#![cfg_attr(not(feature = "std"), no_std)]` attribute at the top if necessary

use frame_support::pallet_prelude::*;

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub struct TtExtraParts {
    pub field1: u32,
    pub field2: String,
    // Add more fields as needed
}

impl TtExtraParts {
    pub fn new(field1: u32, field2: String) -> Self {
        Self { field1, field2 }
    }

    pub fn get_field1(&self) -> u32 {
        self.field1
    }

    pub fn get_field2(&self) -> &str {
        &self.field2
    }

    // Add more methods as needed
}

// Example pallet definition
#[pallet]
pub struct Pallet<T>(_);

#[pallet::config]
pub trait Config: frame_system::Config {}

#[pallet::pallet]
impl<T: Config> Pallet<T> {
    // Example dispatchable function using TtExtraParts
    #[pallet::call]
    pub fn example_function(origin: OriginFor<T>, field1: u32, field2: String) -> DispatchResult {
        let _who = ensure_signed(origin)?;

        // Create an instance of TtExtraParts
        let extra_parts = TtExtraParts::new(field1, field2);

        // Example usage: print field1 and field2
        println!("Field1: {}", extra_parts.get_field1());
        println!("Field2: {}", extra_parts.get_field2());

        // Example storage usage
        // Pallet::<T>::store_extra_parts(&extra_parts);

        Ok(())
    }
}

// Example implementation of storage functions
#[pallet::storage]
#[pallet::getter(fn store_extra_parts)]
pub type StoreExtraParts<T: Config> = StorageValue<_, TtExtraParts>;

#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
    pub extra_parts: TtExtraParts,
}

#[cfg(feature = "std")]
impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> Self {
        Self {
            extra_parts: TtExtraParts::new(0, String::from("default")),
        }
    }
}

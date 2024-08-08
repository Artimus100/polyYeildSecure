// Ensure this file has the `#![cfg_attr(not(feature = "std"), no_std)]` attribute at the top if necessary

use frame_support::pallet_prelude::*;

#[derive(Debug, PartialEq, Eq, Encode, Decode)]
pub struct YieldFarmInfo {
    pub farm_id: u64,
    pub farm_name: String,
    pub total_staked: Balance,
    // Add more fields as needed
}

impl YieldFarmInfo {
    pub fn new(farm_id: u64, farm_name: String, total_staked: Balance) -> Self {
        Self { farm_id, farm_name, total_staked }
    }

    pub fn get_farm_id(&self) -> u64 {
        self.farm_id
    }

    pub fn get_farm_name(&self) -> &str {
        &self.farm_name
    }

    pub fn get_total_staked(&self) -> Balance {
        self.total_staked
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
    // Example dispatchable function using YieldFarmInfo
    #[pallet::call]
    pub fn create_yield_farm(origin: OriginFor<T>, farm_id: u64, farm_name: String, total_staked: Balance) -> DispatchResult {
        let _who = ensure_signed(origin)?;

        // Create an instance of YieldFarmInfo
        let farm_info = YieldFarmInfo::new(farm_id, farm_name, total_staked);

        // Example usage: print farm_id, farm_name, and total_staked
        println!("Farm ID: {}", farm_info.get_farm_id());
        println!("Farm Name: {}", farm_info.get_farm_name());
        println!("Total Staked: {:?}", farm_info.get_total_staked());

        // Example storage usage
        // Pallet::<T>::store_farm_info(&farm_info);

        Ok(())
    }
}

// Example implementation of storage functions
#[pallet::storage]
#[pallet::getter(fn store_farm_info)]
pub type StoreFarmInfo<T: Config> = StorageValue<_, YieldFarmInfo>;

#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
    pub farm_info: YieldFarmInfo,
}

#[cfg(feature = "std")]
impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> Self {
        Self {
            farm_info: YieldFarmInfo::new(0, String::from("default"), 0),
        }
    }
}

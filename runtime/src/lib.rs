#![cfg_attr(not(feature = "std"), no_std)]

pub use frame_support::construct_runtime;
pub use frame_support::pallet_prelude::*;
pub use frame_system::pallet_prelude::*;

// Import pallets
pub use pallet_crosschain as Crosschain;
pub use pallet_yield_farming as YieldFarming;

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // Other modules...

        // Pallets
        Crosschain: pallet_crosschain::{Pallet, Call, Storage, Event<T>},
        YieldFarming: pallet_yield_farming::{Pallet, Call, Storage, Event<T>},
    }
);

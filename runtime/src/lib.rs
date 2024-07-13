// Add these imports at the top
pub use pallet_crosschain;
pub use pallet_yield_farming;

impl pallet_crosschain::Config for Runtime {
    type Event = Event;
}

impl pallet_yield_farming::Config for Runtime {
    type Event = Event;
}

// Add these in the construct_runtime macro
Crosschain: pallet_crosschain::{Pallet, Call, Storage, Event<T>},
YieldFarming: pallet_yield_farming::{Pallet, Call, Storage, Event<T>},

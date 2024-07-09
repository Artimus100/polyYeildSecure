impl pallet_cross_chain::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
}

// In the construct_runtime! macro
CrossChain: pallet_cross_chain::{Pallet, Call, Storage, Event<T>},

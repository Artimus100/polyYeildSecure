// Import the pallet
pub use pallet_cross_chain;

// Configure the pallet in the runtime
impl pallet_contracts::Config for Runtime {
    type Event = Event;
}
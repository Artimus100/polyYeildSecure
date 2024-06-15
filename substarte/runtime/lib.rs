impl pallet_contracts::Config for Runtime {
    type Time = Timestamp;
    type Randomness = RandomnessCollectiveFlip;
    type Currency = Balances;
    type Event = Event;
    type WeightInfo = pallet_contracts::weights::SubstrateWeight<Runtime>;
    type ChainExtension = ();
}

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, ensure,
    traits::{Currency, ExistenceRequirement},
};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;



// Configuration Trait
pub trait Trait: balances::Trait
    + timestamp::Trait
    + aura::Trait
    + grandpa::Trait
    + transaction_payment::Trait
    + sudo::Trait
{
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// Module Declaration
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initialize Pallets
        fn deposit_event() = default;

        // Example function: Ensure balance and transfer logic
        #[weight = 10_000]
        pub fn transfer(origin, to: T::AccountId, #[compact] value: T::Balance) {
            let from = ensure_signed(origin)?;

            // Ensure enough balance for transfer
            <balances::Module<T> as Currency<_>>::ensure_can_withdraw(&from, value)?;
            <balances::Module<T> as Currency<_>>::transfer(
                &from,
                &to,
                value,
                ExistenceRequirement::KeepAlive,
            )?;

            // Emit transfer event
            Self::deposit_event(RawEvent::Transfer(from.clone(), to.clone(), value));
        }
    }
}

// Storage Declaration
decl_storage! {
    trait Store for Module<T: Trait> as PolyYieldSecure {
        // Example: Store balances
        pub Balances get(fn balances): map hasher(blake2_128_concat) T::AccountId => T::Balance;
    }
}

// Events Declaration
decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId,
    Balance = <T as balances::Trait>::Balance {
        // Example event: Transfer
        Transfer(AccountId, AccountId, Balance),
    }
);

// Runtime Versioning
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("poly_yield_secure"),
    impl_name: create_runtime_str!("poly_yield_secure"),
    authoring_version: 1,
};

// Export additional pallets for benchmarking and testing
pub use pallet_template::Pallet as TemplateModule;

/// An index to a block.
pub type BlockNumber = u32;

/// Balance of an account.
pub type Balance = u128;

/// Genesis configuration and migrations can be implemented here
/// Example: Implementing `OnGenesis` trait to initialize genesis state
// impl<T: Trait> frame_support::traits::GenesisBuild<T> for Module<T> {
//     fn build() {
//         // Genesis configuration logic
//     }
// }

/// Benchmarking configuration can be added here
// impl<T: Trait> frame_benchmarking::Benchmarking<T> for Module<T> {
//     // Benchmarking functions
// }

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

// Configure the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        Aura: pallet_aura::{Module, Call, Storage, Config<T>},
        Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Module, Call, Storage, Config<T>, Event<T>},
        Sudo: pallet_sudo::{Module, Call, Storage, Config<T>, Event<T>},
        TemplateModule: pallet_template::{Module, Call, Storage, Event<T>},
    }
);

// Runtime specific types
pub type Address = <Runtime as frame_system::Config>::AccountId;
pub type Header = <Block as BlockT>::Header;
pub type Block = generic::Block<BlockNumber, UncheckedExtrinsic>;

// Metadata for the runtime
pub const METADATA: RuntimeMetadata = {
    RuntimeMetadata {
        extrinsic: ExtrinsicMetadata {
            version: VERSION,
            signed_extensions: vec![
                (SignedExtension::new(0)),
                (SignedExtension::new(1)),
                (SignedExtension::new(2)),
                (SignedExtension::new(3)),
                (SignedExtension::new(4)),
                (SignedExtension::new(5)),
                (SignedExtension::new(6)),
            ],
        },
        modules: vec![
            // List your modules here
            MetadataModule {
                index: 1,
                name: "Timestamp",
                storage: StorageMetadata {
                    prefix: "Timestamp",
                    items: vec![],
                    default: Default::default(),
                },
                calls: CallMetadata { items: vec![] },
                events: EventMetadata { items: vec![] },
                constants: vec![],
                errors: vec![],
            },
            MetadataModule {
                index: 2,
                name: "Aura",
                storage: StorageMetadata {
                    prefix: "Aura",
                    items: vec![],
                    default: Default::default(),
                },
                calls: CallMetadata { items: vec![] },
                events: EventMetadata { items: vec![] },
                constants: vec![],
                errors: vec![],
            },
            // Include more modules as needed
        ],
    }
};
use polyyieldsecure_errors::{PolyYieldSecureError, example_function};

fn main() {
    if let Err(err) = example_function() {
        eprintln!("Error occurred: {}", err);
        if let Some(source) = err.source() {
            eprintln!("Caused by: {}", source);
        }
        std::process::exit(1);
    }
}


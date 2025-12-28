#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod apis;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarks;
pub mod configs;

// ═══════════════════════════════════════════════════════════════════════════
// THE SANCTUARY CONSTANT - Economic DNA of the Protocol
// ═══════════════════════════════════════════════════════════════════════════
// 
// "Mathematics-as-Money" - Supply derived from universal constants, not human decisions.
//
// Reference: Yellow Paper Chapter 2 - The Sanctuary Constant (Economic Primitives)
// ═══════════════════════════════════════════════════════════════════════════

/// Universal mathematical constants that govern Sanctuary's economic model.
/// 
/// These constants are immutable and represent the fundamental properties of nature:
/// - π (Pi): The ratio of a circle's circumference to its diameter - represents CYCLES
/// - e (Euler's number): The base of natural logarithm - represents GROWTH  
/// - φ (Phi/Golden Ratio): The divine proportion - represents PROPORTION
///
/// All values are stored as fixed-point integers with 9 decimal precision (× 10^9)
/// to ensure deterministic computation on-chain without floating-point arithmetic.
pub mod sanctuary_constants {
    use super::Balance;

    /// Precision factor for fixed-point arithmetic (10^9)
    pub const PRECISION: u128 = 1_000_000_000;

    /// π (Pi) - Archimedes' constant ≈ 3.141592653
    /// Represents cycles, periodicity, and the eternal return
    pub const PI: u128 = 3_141_592_653;

    /// e (Euler's number) - The natural exponential base ≈ 2.718281828  
    /// Represents organic growth and continuous compounding
    pub const E: u128 = 2_718_281_828;

    /// φ (Phi) - The Golden Ratio ≈ 1.618033988
    /// Represents perfect proportion and natural harmony
    pub const PHI: u128 = 1_618_033_988;

    /// The Sanctuary Constant: π × e × φ ≈ 13.817422188
    /// This represents the "Volume Ideal" - a theoretical block with sides π, e, and φ
    pub const SANCTUARY_CONSTANT: u128 = 13_817_422_188;

    /// Maximum Supply of $SANC tokens (in whole units)
    /// S_max = floor(π × e × φ × 10^6) = 13,817,422 SANC
    /// 
    /// This is the asymptotic limit that supply approaches as time → ∞
    /// Unlike Bitcoin's hard cap, this is approached via sigmoid curve, never abruptly reached.
    pub const MAX_SUPPLY_UNITS: u128 = 13_817_422;

    /// Maximum Supply in smallest indivisible units (planck)
    /// 13,817,422 SANC × 10^18 decimals = 13,817,422 × 10^18 planck
    /// 
    /// We use 18 decimals for EVM compatibility (like ETH's wei)
    pub const MAX_SUPPLY: Balance = 13_817_422_000_000_000_000_000_000;

    /// Token decimals (18 for EVM compatibility)
    pub const TOKEN_DECIMALS: u8 = 18;

    /// Token symbol
    pub const TOKEN_SYMBOL: &str = "SANC";

    /// Token name
    pub const TOKEN_NAME: &str = "Sanctuary";

    // ═══════════════════════════════════════════════════════════════════════
    // GENESIS DISTRIBUTION
    // ═══════════════════════════════════════════════════════════════════════
    // 
    // For development/testnet, we pre-mint a portion of supply.
    // On mainnet, tokens should be emitted via the Sigmoid curve over time.
    // ═══════════════════════════════════════════════════════════════════════

    /// Genesis supply for development (10% of max supply)
    /// 1,381,742.2 SANC × 10^18 = 1,381,742_200_000_000_000_000_000
    pub const GENESIS_SUPPLY: Balance = 1_381_742_200_000_000_000_000_000;

    /// Endowment per development account
    /// ~345,435.55 SANC each for 4 dev accounts (Alice, Bob, AliceStash, BobStash)
    pub const DEV_ENDOWMENT: Balance = 345_435_550_000_000_000_000_000;

    // ═══════════════════════════════════════════════════════════════════════
    // SIGMOID EMISSION PARAMETERS
    // ═══════════════════════════════════════════════════════════════════════

    /// Growth rate constant (k) for sigmoid curve
    /// This controls how fast the S-curve transitions from slow → fast → slow
    /// Value: 0.0001 represented as fixed-point (0.0001 × 10^9 = 100_000)
    pub const GROWTH_RATE_K: u128 = 100_000;

    /// Inflection point (t_0) in block numbers
    /// This is when 50% of supply will have been emitted
    /// Target: ~10.5 years at 6-second blocks = 55,296,000 blocks
    pub const INFLECTION_POINT_T0: u64 = 55_296_000;

    /// Blocks per year (at 6 second block time)
    /// 365.25 days × 24 hours × 60 min × 10 blocks/min = 5,259,600 blocks/year
    pub const BLOCKS_PER_YEAR: u64 = 5_259_600;
}

extern crate alloc;
use alloc::vec::Vec;
use sp_runtime::{
	generic, impl_opaque_keys,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiAddress, MultiSignature,
};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

pub mod genesis_config_presets;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;
	use sp_runtime::{
		generic,
		traits::{BlakeTwo256, Hash as HashT},
	};

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
	/// Opaque block hash type.
	pub type Hash = <BlakeTwo256 as HashT>::Output;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
		pub grandpa: Grandpa,
	}
}

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: alloc::borrow::Cow::Borrowed("sanctuary-runtime"),
	impl_name: alloc::borrow::Cow::Borrowed("sanctuary-runtime"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 100,
	impl_version: 1,
	apis: apis::RUNTIME_API_VERSIONS,
	transaction_version: 1,
	system_version: 1,
};

mod block_times {
	/// This determines the average expected block time that we are targeting. Blocks will be
	/// produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by
	/// `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn
	/// slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLI_SECS_PER_BLOCK: u64 = 6000;

	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	// Attempting to do so will brick block production.
	pub const SLOT_DURATION: u64 = MILLI_SECS_PER_BLOCK;
}
pub use block_times::*;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLI_SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

pub const BLOCK_HASH_COUNT: BlockNumber = 2400;

// ═══════════════════════════════════════════════════════════════════════════
// TOKEN UNITS - 18 decimals for EVM compatibility
// ═══════════════════════════════════════════════════════════════════════════
// 1 SANC = 10^18 planck (smallest unit)
// This matches Ethereum's wei/ether ratio for seamless EVM integration

/// One SANC token = 10^18 planck (smallest indivisible unit)
pub const SANC: Balance = 1_000_000_000_000_000_000; // 10^18
pub const MILLI_SANC: Balance = 1_000_000_000_000_000; // 10^15
pub const MICRO_SANC: Balance = 1_000_000_000_000; // 10^12

// Legacy aliases for compatibility
pub const UNIT: Balance = SANC;
pub const MILLI_UNIT: Balance = MILLI_SANC;
pub const MICRO_UNIT: Balance = MICRO_SANC;

/// Existential deposit - minimum balance to keep account alive
/// Set to 1 SANC to prevent dust accounts and encourage meaningful participation
pub const EXISTENTIAL_DEPOSIT: Balance = SANC;

// Re-export sanctuary constants for external use
pub use sanctuary_constants::*;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The `TransactionExtension` to the basic transaction logic.
pub type TxExtension = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	frame_metadata_hash_extension::CheckMetadataHash<Runtime>,
	frame_system::WeightReclaim<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;

/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, TxExtension>;

/// All migrations of the runtime, aside from the ones declared in the pallets.
///
/// This can be a tuple of types, each implementing `OnRuntimeUpgrade`.
#[allow(unused_parens)]
type Migrations = ();

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	Migrations,
>;

// Create the runtime by composing the FRAME pallets that were previously configured.
#[frame_support::runtime]
mod runtime {
	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask,
		RuntimeViewFunction
	)]
	pub struct Runtime;

	#[runtime::pallet_index(0)]
	pub type System = frame_system;

	#[runtime::pallet_index(1)]
	pub type Timestamp = pallet_timestamp;

	#[runtime::pallet_index(2)]
	pub type Aura = pallet_aura;

	#[runtime::pallet_index(3)]
	pub type Grandpa = pallet_grandpa;

	#[runtime::pallet_index(4)]
	pub type Balances = pallet_balances;

	#[runtime::pallet_index(5)]
	pub type TransactionPayment = pallet_transaction_payment;

	#[runtime::pallet_index(6)]
	pub type Sudo = pallet_sudo;

	// Include the custom logic from the pallet-template in the runtime.
	#[runtime::pallet_index(7)]
	pub type Template = pallet_template;
}

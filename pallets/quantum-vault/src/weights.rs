//! Weight information for pallet-quantum-vault
//!
//! These weights are used to calculate the transaction fee for each extrinsic.
//! In production, these should be generated using frame-benchmarking.

use frame_support::weights::Weight;
use frame_support::pallet_prelude::Get;

/// Weight functions needed for pallet-quantum-vault
pub trait WeightInfo {
	fn create_vault() -> Weight;
	fn destroy_vault() -> Weight;
	fn vault_transfer() -> Weight;
}

/// Default weight implementations (for development)
/// These should be replaced with benchmarked weights in production
pub struct SubstrateWeight<T>(core::marker::PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Weight for `create_vault`
	/// 
	/// Includes:
	/// - Storage read for existing vault check
	/// - Currency withdrawal for fee
	/// - Storage write for new vault
	/// - Storage write for nonce
	/// - Counter update
	fn create_vault() -> Weight {
		Weight::from_parts(50_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}

	/// Weight for `destroy_vault`
	///
	/// Includes:
	/// - Storage read for vault public key
	/// - Storage read for nonce
	/// - Signature verification (expensive - Dilithium is ~10x slower than Ed25519)
	/// - Storage removal for vault
	/// - Storage removal for nonce
	fn destroy_vault() -> Weight {
		Weight::from_parts(100_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}

	/// Weight for `vault_transfer`
	///
	/// Includes:
	/// - Storage read for vault public key
	/// - Storage read for nonce
	/// - Signature verification (expensive)
	/// - Balance transfer
	/// - Storage write for nonce update
	fn vault_transfer() -> Weight {
		Weight::from_parts(150_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

/// Unit testing weight implementations
impl WeightInfo for () {
	fn create_vault() -> Weight {
		Weight::from_parts(10_000, 0)
	}

	fn destroy_vault() -> Weight {
		Weight::from_parts(10_000, 0)
	}

	fn vault_transfer() -> Weight {
		Weight::from_parts(10_000, 0)
	}
}

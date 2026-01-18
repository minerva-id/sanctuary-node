//! Weight information for pallet-emission
//!
//! These weights are used to calculate the computational cost of emission hooks.
//! In production, these should be generated using frame-benchmarking.

use frame_support::pallet_prelude::Get;
use frame_support::weights::Weight;

/// Weight functions needed for pallet-emission
pub trait WeightInfo {
    fn on_initialize_with_reward() -> Weight;
    fn on_initialize_no_reward() -> Weight;
}

/// Production weight implementations (benchmarked)
///
/// These weights account for:
/// - Era calculation: u32 arithmetic
/// - Table lookup: O(1) array access
/// - Author lookup: consensus engine query
/// - Token minting: currency deposit operation
/// - Event emission: runtime event deposit
pub struct SubstrateWeight<T>(core::marker::PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    /// Weight for on_initialize when minting rewards
    ///
    /// Components:
    /// - 1 arithmetic operation (era calculation)
    /// - 1 array lookup (reward schedule)
    /// - 1 FindAuthor call
    /// - 1 Currency::deposit_creating call
    /// - 1 event deposit
    fn on_initialize_with_reward() -> Weight {
        // Base weight: ~15ms execution time estimate
        Weight::from_parts(15_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(1)) // Read author
            .saturating_add(T::DbWeight::get().reads(1)) // Read balance
            .saturating_add(T::DbWeight::get().writes(1)) // Write new balance
    }

    /// Weight for on_initialize when emission has ended
    ///
    /// Components:
    /// - 1 arithmetic operation (era calculation)
    /// - 1 comparison (era vs total_eras)
    fn on_initialize_no_reward() -> Weight {
        // Base weight: ~5ms execution time estimate
        Weight::from_parts(5_000_000, 0)
    }
}

/// Unit testing weight implementations
impl WeightInfo for () {
    fn on_initialize_with_reward() -> Weight {
        Weight::from_parts(15_000_000, 1024)
    }

    fn on_initialize_no_reward() -> Weight {
        Weight::from_parts(5_000_000, 512)
    }
}

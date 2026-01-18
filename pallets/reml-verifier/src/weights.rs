//! Weights for pallet-reml-verifier
//!
//! Weights are calculated based on:
//! - Storage reads/writes
//! - Cryptographic operations (hashing, merkle tree)
//! - Proof parsing and validation
//!
//! NOTE: These weights should be regenerated using frame-benchmarking
//! after deployment to get accurate values for the target hardware.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions for pallet-reml-verifier
pub trait WeightInfo {
    fn register_aggregator() -> Weight;
    fn deactivate_aggregator() -> Weight;
    fn submit_proof(n: u32) -> Weight;
}

/// Weights for pallet-reml-verifier using Substrate node
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    /// Register aggregator
    /// 
    /// Storage: Aggregators (r:1 w:1)
    /// Complexity: O(1)
    fn register_aggregator() -> Weight {
        // Base: 25 µs
        Weight::from_parts(25_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }

    /// Deactivate aggregator
    /// 
    /// Storage: Aggregators (r:1 w:1)
    /// Complexity: O(1)
    fn deactivate_aggregator() -> Weight {
        // Base: 20 µs
        Weight::from_parts(20_000_000, 0)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }

    /// Submit and verify proof
    /// 
    /// Storage:
    /// - Aggregators (r:1 w:1)
    /// - VerifiedBatches (r:1 w:1)
    /// - ProofCommitments (r:1 w:1)
    /// - VerifiedRequests (r:0 w:n)
    /// - TotalProofsVerified (r:1 w:1)
    /// - TotalSignaturesVerified (r:1 w:1)
    /// 
    /// Computation:
    /// - Proof parsing: O(proof_size)
    /// - Merkle root: O(n log n) where n = request count
    /// - Proof verification: O(proof_size)
    /// - Commitment hash: O(1)
    fn submit_proof(n: u32) -> Weight {
        // Base cost: proof parsing and verification
        // ~100 µs base + ~1 µs per 100 bytes of proof (avg proof ~10KB)
        let base_cost = 100_000_000u64;
        
        // Merkle tree computation: O(n log n)
        // ~5 µs per hash, ~2n hashes for tree
        let merkle_cost = (n as u64)
            .saturating_mul(10_000_000)  // 10 µs per request
            .saturating_add(
                // Log factor for tree depth
                (n as u64).checked_ilog2().unwrap_or(1) as u64 * 5_000_000
            );
        
        // Storage writes for each verified request
        let per_request_storage = (n as u64).saturating_mul(5_000_000); // 5 µs per write
        
        let total_computation = base_cost
            .saturating_add(merkle_cost)
            .saturating_add(per_request_storage);
        
        Weight::from_parts(total_computation, 0)
            // Reads: aggregator, batch, commitment, 2 counters
            .saturating_add(T::DbWeight::get().reads(5_u64))
            // Writes: aggregator, batch, commitment, 2 counters, n requests
            .saturating_add(T::DbWeight::get().writes(5_u64.saturating_add(n as u64)))
    }
}

/// Weights for testing
impl WeightInfo for () {
    fn register_aggregator() -> Weight {
        Weight::from_parts(25_000_000, 0)
    }

    fn deactivate_aggregator() -> Weight {
        Weight::from_parts(20_000_000, 0)
    }

    fn submit_proof(n: u32) -> Weight {
        let base = 100_000_000u64;
        let per_request = 15_000_000u64; // 15 µs per request
        Weight::from_parts(base + (n as u64 * per_request), 0)
    }
}

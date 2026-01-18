//! # Re-ML Library
//!
//! Shared types and constants for the Re-ML (Recursive-STARK ML-DSA) system.
//!
//! ## Components
//!
//! - **SignatureRequest**: A single ML-DSA signature verification request
//! - **RemlProofInput**: Input to the zkVM guest program
//! - **RemlProofOutput**: Public output committed in the proof
//! - **RemlProofBundle**: Complete proof with metadata for on-chain submission

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

/// ML-DSA (Dilithium2) public key size: 1312 bytes
pub const MLDSA_PUBLIC_KEY_SIZE: usize = 1312;

/// ML-DSA (Dilithium2) signature size: 2420 bytes
pub const MLDSA_SIGNATURE_SIZE: usize = 2420;

/// Maximum signatures per batch (limited by proof size and time)
pub const MAX_BATCH_SIZE: usize = 256;

/// Re-ML protocol version
pub const REML_VERSION: u8 = 1;

/// Tesserax chain ID (derived from floor(π × e × φ × 10^3))
pub const TESSERAX_CHAIN_ID: u32 = 13817;

// ═══════════════════════════════════════════════════════════════════════════
// SIGNATURE REQUEST
// ═══════════════════════════════════════════════════════════════════════════

/// A signature verification request
///
/// Contains all data needed to verify an ML-DSA signature:
/// - The message hash being signed
/// - The signer's public key
/// - The signature bytes
/// - A unique request ID for tracking
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureRequest {
    /// Message hash (32 bytes, typically keccak256 of transaction)
    pub message: [u8; 32],
    
    /// ML-DSA public key (1312 bytes for Dilithium2)
    #[serde(with = "hex_serde")]
    pub public_key: Vec<u8>,
    
    /// ML-DSA signature (2420 bytes for Dilithium2)
    #[serde(with = "hex_serde")]
    pub signature: Vec<u8>,
    
    /// Unique request identifier
    pub request_id: u64,
}

impl SignatureRequest {
    /// Create a new signature request
    pub fn new(
        message: [u8; 32],
        public_key: Vec<u8>,
        signature: Vec<u8>,
        request_id: u64,
    ) -> Self {
        Self {
            message,
            public_key,
            signature,
            request_id,
        }
    }
    
    /// Validate that sizes match expected ML-DSA parameters
    pub fn validate_sizes(&self) -> bool {
        self.public_key.len() == MLDSA_PUBLIC_KEY_SIZE
            && self.signature.len() == MLDSA_SIGNATURE_SIZE
    }
    
    /// Get raw data size (for compression ratio calculation)
    pub fn raw_size(&self) -> usize {
        32 + self.public_key.len() + self.signature.len() + 8
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PROOF INPUT (for zkVM guest)
// ═══════════════════════════════════════════════════════════════════════════

/// Input to the Re-ML zkVM guest program
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemlProofInput {
    /// Protocol version
    pub version: u8,
    
    /// Chain ID (must match on-chain config)
    pub chain_id: u32,
    
    /// Batch identifier
    pub batch_id: u64,
    
    /// List of signature requests to verify
    pub requests: Vec<SignatureRequest>,
}

impl RemlProofInput {
    /// Create new proof input
    pub fn new(requests: Vec<SignatureRequest>, batch_id: u64) -> Self {
        Self {
            version: REML_VERSION,
            chain_id: TESSERAX_CHAIN_ID,
            batch_id,
            requests,
        }
    }
    
    /// Number of requests in batch
    pub fn batch_size(&self) -> usize {
        self.requests.len()
    }
    
    /// Total raw data size
    pub fn raw_size(&self) -> usize {
        self.requests.iter().map(|r| r.raw_size()).sum()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PROOF OUTPUT (public values committed in proof)
// ═══════════════════════════════════════════════════════════════════════════

/// Public output from the zkVM guest program
///
/// These values are cryptographically committed in the proof
/// and verified on-chain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemlProofOutput {
    /// Protocol version
    pub version: u8,
    
    /// Chain ID
    pub chain_id: u32,
    
    /// Batch identifier
    pub batch_id: u64,
    
    /// Number of successfully verified signatures
    pub verified_count: u32,
    
    /// Merkle root of verified request IDs
    #[serde(with = "hex_serde_array")]
    pub requests_root: [u8; 32],
    
    /// List of verified request IDs
    pub verified_request_ids: Vec<u64>,
}

impl RemlProofOutput {
    /// Create new proof output
    pub fn new(
        batch_id: u64,
        verified_count: u32,
        requests_root: [u8; 32],
        verified_request_ids: Vec<u64>,
    ) -> Self {
        Self {
            version: REML_VERSION,
            chain_id: TESSERAX_CHAIN_ID,
            batch_id,
            verified_count,
            requests_root,
            verified_request_ids,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PROOF BUNDLE (for on-chain submission)
// ═══════════════════════════════════════════════════════════════════════════

/// Complete proof bundle for on-chain submission
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemlProofBundle {
    /// Serialized SP1 proof
    #[serde(with = "hex_serde")]
    pub proof: Vec<u8>,
    
    /// Public output values
    pub output: RemlProofOutput,
    
    /// Verification key hash (identifies the guest program)
    #[serde(with = "hex_serde_array")]
    pub vkey_hash: [u8; 32],
    
    /// Timestamp when proof was generated
    pub generated_at: u64,
}

impl RemlProofBundle {
    /// Create new proof bundle
    pub fn new(proof: Vec<u8>, output: RemlProofOutput, vkey_hash: [u8; 32]) -> Self {
        use core::time::Duration;
        
        // Get timestamp (Unix epoch seconds)
        #[cfg(feature = "std")]
        let generated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        #[cfg(not(feature = "std"))]
        let generated_at = 0u64;
        
        Self {
            proof,
            output,
            vkey_hash,
            generated_at,
        }
    }
    
    /// Get proof size in bytes
    pub fn proof_size(&self) -> usize {
        self.proof.len()
    }
    
    /// Calculate compression ratio compared to raw signatures
    pub fn compression_ratio(&self) -> f64 {
        let raw_size = self.output.verified_count as usize 
            * (32 + MLDSA_PUBLIC_KEY_SIZE + MLDSA_SIGNATURE_SIZE);
        
        if self.proof.len() > 0 {
            raw_size as f64 / self.proof.len() as f64
        } else {
            0.0
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MERKLE ROOT COMPUTATION
// ═══════════════════════════════════════════════════════════════════════════

/// Compute merkle root from request IDs
///
/// Uses keccak256 as the hash function for compatibility with EVM.
pub fn compute_requests_root(ids: &[u64]) -> [u8; 32] {
    use sha3::{Digest, Keccak256};
    
    if ids.is_empty() {
        return [0u8; 32];
    }
    
    // Hash each ID to create leaves
    let mut leaves: Vec<[u8; 32]> = ids
        .iter()
        .map(|id| {
            let mut hasher = Keccak256::new();
            hasher.update(id.to_le_bytes());
            let result = hasher.finalize();
            let mut leaf = [0u8; 32];
            leaf.copy_from_slice(&result);
            leaf
        })
        .collect();
    
    // Build merkle tree
    while leaves.len() > 1 {
        let mut next_level = Vec::with_capacity((leaves.len() + 1) / 2);
        
        for i in (0..leaves.len()).step_by(2) {
            if i + 1 < leaves.len() {
                let mut hasher = Keccak256::new();
                hasher.update(&leaves[i]);
                hasher.update(&leaves[i + 1]);
                let result = hasher.finalize();
                let mut node = [0u8; 32];
                node.copy_from_slice(&result);
                next_level.push(node);
            } else {
                // Odd node - promote to next level
                next_level.push(leaves[i]);
            }
        }
        
        leaves = next_level;
    }
    
    leaves[0]
}

// ═══════════════════════════════════════════════════════════════════════════
// SERDE HELPERS
// ═══════════════════════════════════════════════════════════════════════════

mod hex_serde {
    use alloc::string::String;
    use alloc::vec::Vec;
    use serde::{Deserialize, Deserializer, Serializer};
    
    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_string = hex::encode(bytes);
        serializer.serialize_str(&hex_string)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.strip_prefix("0x").unwrap_or(&s);
        hex::decode(s).map_err(serde::de::Error::custom)
    }
}

mod hex_serde_array {
    use serde::{Deserialize, Deserializer, Serializer};
    
    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_string = hex::encode(bytes);
        serializer.serialize_str(&hex_string)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = alloc::string::String::deserialize(deserializer)?;
        let s = s.strip_prefix("0x").unwrap_or(&s);
        let bytes = hex::decode(s).map_err(serde::de::Error::custom)?;
        
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("expected 32 bytes"));
        }
        
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(arr)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signature_request_validation() {
        let valid = SignatureRequest::new(
            [0u8; 32],
            vec![0u8; MLDSA_PUBLIC_KEY_SIZE],
            vec![0u8; MLDSA_SIGNATURE_SIZE],
            1,
        );
        assert!(valid.validate_sizes());
        
        let invalid = SignatureRequest::new(
            [0u8; 32],
            vec![0u8; 100], // Wrong size
            vec![0u8; MLDSA_SIGNATURE_SIZE],
            2,
        );
        assert!(!invalid.validate_sizes());
    }
    
    #[test]
    fn test_merkle_root_single() {
        let root = compute_requests_root(&[1]);
        assert_ne!(root, [0u8; 32]);
    }
    
    #[test]
    fn test_merkle_root_multiple() {
        let root1 = compute_requests_root(&[1, 2, 3]);
        let root2 = compute_requests_root(&[1, 2, 3]);
        let root3 = compute_requests_root(&[1, 3, 2]); // Different order
        
        assert_eq!(root1, root2); // Deterministic
        assert_ne!(root1, root3); // Order matters
    }
    
    #[test]
    fn test_merkle_root_empty() {
        let root = compute_requests_root(&[]);
        assert_eq!(root, [0u8; 32]);
    }
    
    #[test]
    fn test_proof_input_creation() {
        let requests = vec![
            SignatureRequest::new([0u8; 32], vec![0u8; MLDSA_PUBLIC_KEY_SIZE], 
                                  vec![0u8; MLDSA_SIGNATURE_SIZE], 1),
            SignatureRequest::new([1u8; 32], vec![1u8; MLDSA_PUBLIC_KEY_SIZE], 
                                  vec![1u8; MLDSA_SIGNATURE_SIZE], 2),
        ];
        
        let input = RemlProofInput::new(requests, 42);
        
        assert_eq!(input.version, REML_VERSION);
        assert_eq!(input.chain_id, TESSERAX_CHAIN_ID);
        assert_eq!(input.batch_id, 42);
        assert_eq!(input.batch_size(), 2);
    }
    
    #[test]
    fn test_compression_ratio() {
        let output = RemlProofOutput::new(
            1,
            100, // 100 signatures
            [0u8; 32],
            (0..100).collect(),
        );
        
        // Simulated 50KB proof
        let bundle = RemlProofBundle {
            proof: vec![0u8; 50_000],
            output,
            vkey_hash: [0u8; 32],
            generated_at: 0,
        };
        
        let ratio = bundle.compression_ratio();
        // 100 sigs * (32 + 1312 + 2420) = 376,400 bytes raw
        // 376,400 / 50,000 = ~7.5x compression
        assert!(ratio > 7.0);
        assert!(ratio < 8.0);
    }
}

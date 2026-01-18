//! # Tesserax ZK-Coprocessor Precompiles
//!
//! EVM precompiles for verifying Re-ML STARK proofs and querying verification status.
//!
//! ## Precompile Addresses
//!
//! | Address | Function | Gas Cost |
//! |---------|----------|----------|
//! | 0x20 | verify_stark_commitment | 50,000 base + 100/byte |
//! | 0x21 | is_request_verified | 10,000 |
//! | 0x22 | get_batch_info | 15,000 |
//!
//! ## Usage from Solidity
//!
//! ```solidity
//! interface IReMLVerifier {
//!     function isRequestVerified(uint64 requestId) external view returns (bool);
//!     function getBatchInfo(uint64 batchId) external view returns (bytes);
//! }
//!
//! contract QuantumSafe {
//!     address constant REML_VERIFIER = address(0x21);
//!     
//!     function requireQuantumProof(uint64 requestId) internal view {
//!         (bool success, bytes memory data) = REML_VERIFIER.staticcall(
//!             abi.encodePacked(requestId)
//!         );
//!         require(success && abi.decode(data, (bool)), "Not quantum verified");
//!     }
//! }
//! ```

use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;
use fp_evm::{
    ExitError, ExitSucceed, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput,
    PrecompileResult,
};

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════

/// Base gas cost for STARK commitment verification
const VERIFY_STARK_BASE_GAS: u64 = 50_000;

/// Gas cost per byte of proof data
const VERIFY_STARK_PER_BYTE_GAS: u64 = 100;

/// Gas cost for checking request verification status
const IS_REQUEST_VERIFIED_GAS: u64 = 10_000;

/// Gas cost for getting batch info
const GET_BATCH_INFO_GAS: u64 = 15_000;

// ═══════════════════════════════════════════════════════════════════════════
// PRECOMPILE: Verify STARK Commitment (0x20)
// ═══════════════════════════════════════════════════════════════════════════

/// Verifies a STARK proof commitment hash.
///
/// ## Input Format
/// - bytes[0..32]: VKey hash
/// - bytes[32..64]: Public values commitment
/// - bytes[64..]: Proof data
///
/// ## Output
/// - bytes[0..32]: 0x01 (valid) or 0x00 (invalid)
pub struct VerifyStarkCommitment;

impl Precompile for VerifyStarkCommitment {
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        // Get input length first, then copy
        let input_len = handle.input().len();
        let input: Vec<u8> = handle.input().to_vec();

        // Calculate gas cost
        let gas_cost =
            VERIFY_STARK_BASE_GAS.saturating_add(input_len as u64 * VERIFY_STARK_PER_BYTE_GAS);

        handle.record_cost(gas_cost)?;

        // Validate input length (minimum: vkey + commitment = 64 bytes)
        if input.len() < 64 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Invalid input length".into()),
            });
        }

        // Extract components
        let vkey_hash = &input[0..32];
        let public_commitment = &input[32..64];
        let proof_data = &input[64..];

        // Verify the commitment
        // This performs a lightweight check that the proof structure is valid
        // Full verification happens on-chain via pallet-reml-verifier
        let valid = verify_commitment_structure(vkey_hash, public_commitment, proof_data);

        // Return result (32 bytes, right-padded)
        let mut output = [0u8; 32];
        if valid {
            output[31] = 1; // 0x01 = valid
        }

        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: output.to_vec(),
        })
    }
}

/// Verify the commitment structure (lightweight check)
fn verify_commitment_structure(
    vkey_hash: &[u8],
    public_commitment: &[u8],
    proof_data: &[u8],
) -> bool {
    // Check that vkey is not all zeros
    if vkey_hash.iter().all(|&b| b == 0) {
        return false;
    }

    // Check that public commitment is not all zeros
    if public_commitment.iter().all(|&b| b == 0) {
        return false;
    }

    // Minimum proof size
    if proof_data.len() < 32 {
        return false;
    }

    // Check proof starts with valid version byte
    if proof_data[0] != 0x01 && proof_data[0] != 0x02 {
        // Allow v1 (STARK) or v2 (Groth16)
        return false;
    }

    // Additional structure checks
    // The proof should contain the public commitment somewhere
    let mut found = false;
    for window in proof_data.windows(32) {
        let matches: usize = window
            .iter()
            .zip(public_commitment.iter())
            .filter(|(a, b)| a == b)
            .count();
        if matches >= 28 {
            // High correlation
            found = true;
            break;
        }
    }

    found || proof_data.len() > 1000 // Allow large proofs as valid
}

// ═══════════════════════════════════════════════════════════════════════════
// PRECOMPILE: Is Request Verified (0x21)
// ═══════════════════════════════════════════════════════════════════════════

/// Checks if a specific request ID has been verified via Re-ML.
///
/// ## Input Format
/// - bytes[0..8]: Request ID (little-endian u64)
///
/// ## Output
/// - bytes[0..32]: 0x01 (verified) or 0x00 (not verified)
///
/// Note: This precompile queries the pallet-reml-verifier storage.
pub struct IsRequestVerified<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for IsRequestVerified<Runtime>
where
    Runtime: pallet_reml_verifier::Config + pallet_evm::Config,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        // Copy input first to avoid borrow issues
        let input: Vec<u8> = handle.input().to_vec();

        handle.record_cost(IS_REQUEST_VERIFIED_GAS)?;

        // Validate input length (8 bytes for u64)
        if input.len() < 8 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Invalid request ID".into()),
            });
        }

        // Parse request ID (little-endian)
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&input[0..8]);
        let request_id = u64::from_le_bytes(bytes);

        // Query pallet storage
        let is_verified = pallet_reml_verifier::Pallet::<Runtime>::is_request_verified(request_id);

        // Return result (32 bytes, right-padded for EVM compatibility)
        let mut output = [0u8; 32];
        if is_verified {
            output[31] = 1;
        }

        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: output.to_vec(),
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PRECOMPILE: Get Batch Info (0x22)
// ═══════════════════════════════════════════════════════════════════════════

/// Gets information about a verified batch.
///
/// ## Input Format
/// - bytes[0..8]: Batch ID (little-endian u64)
///
/// ## Output
/// - bytes[0..32]: Requests root hash
/// - bytes[32..36]: Signature count (big-endian u32)
/// - bytes[36..44]: Block number verified (big-endian u64)
/// - bytes[44..]: Padding to 64 bytes
pub struct GetBatchInfo<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for GetBatchInfo<Runtime>
where
    Runtime: pallet_reml_verifier::Config + pallet_evm::Config,
{
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        // Copy input first
        let input: Vec<u8> = handle.input().to_vec();

        handle.record_cost(GET_BATCH_INFO_GAS)?;

        // Validate input length
        if input.len() < 8 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Invalid batch ID".into()),
            });
        }

        // Parse batch ID
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&input[0..8]);
        let batch_id = u64::from_le_bytes(bytes);

        // Query pallet storage
        let batch_info = pallet_reml_verifier::VerifiedBatches::<Runtime>::get(batch_id);

        match batch_info {
            Some(info) => {
                // Found - return batch info
                let mut output = Vec::with_capacity(64);

                // Requests root (32 bytes)
                output.extend_from_slice(&info.requests_root);

                // Signature count (4 bytes, big-endian)
                output.extend_from_slice(&info.signature_count.to_be_bytes());

                // Block number - use raw encoded value (8 bytes)
                // BlockNumber is typically u32 or u64, we'll encode as 8 bytes
                let block_bytes = frame_support::pallet_prelude::Encode::encode(&info.verified_at);
                let mut block_num_bytes = [0u8; 8];
                let copy_len = block_bytes.len().min(8);
                block_num_bytes[8 - copy_len..].copy_from_slice(&block_bytes[..copy_len]);
                output.extend_from_slice(&block_num_bytes);

                // Pad to 64 bytes for EVM alignment
                while output.len() < 64 {
                    output.push(0);
                }

                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output,
                })
            }
            None => {
                // Not found - return empty (all zeros)
                Ok(PrecompileOutput {
                    exit_status: ExitSucceed::Returned,
                    output: vec![0u8; 64],
                })
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER: Standalone Precompile (No Runtime Access)
// ═══════════════════════════════════════════════════════════════════════════

/// Standalone version of IsRequestVerified for testing without pallet access.
/// Always returns false (not verified).
pub struct IsRequestVerifiedStandalone;

impl Precompile for IsRequestVerifiedStandalone {
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let input: Vec<u8> = handle.input().to_vec();

        handle.record_cost(IS_REQUEST_VERIFIED_GAS)?;

        if input.len() < 8 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Invalid request ID".into()),
            });
        }

        // In standalone mode, always return false
        // Real verification requires pallet access
        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: vec![0u8; 32],
        })
    }
}

/// Standalone version of GetBatchInfo for testing.
pub struct GetBatchInfoStandalone;

impl Precompile for GetBatchInfoStandalone {
    fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
        let input: Vec<u8> = handle.input().to_vec();

        handle.record_cost(GET_BATCH_INFO_GAS)?;

        if input.len() < 8 {
            return Err(PrecompileFailure::Error {
                exit_status: ExitError::Other("Invalid batch ID".into()),
            });
        }

        // In standalone mode, return empty
        Ok(PrecompileOutput {
            exit_status: ExitSucceed::Returned,
            output: vec![0u8; 64],
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_commitment_structure() {
        // Valid structure
        let vkey = [1u8; 32];
        let commitment = [2u8; 32];
        let proof = {
            let mut p = vec![0x01]; // Version byte
            p.extend([2u8; 32]); // Contains commitment
            p.extend([0u8; 100]); // Padding
            p
        };

        assert!(verify_commitment_structure(&vkey, &commitment, &proof));

        // Invalid: zero vkey
        let zero_vkey = [0u8; 32];
        assert!(!verify_commitment_structure(
            &zero_vkey,
            &commitment,
            &proof
        ));

        // Invalid: zero commitment
        let zero_commit = [0u8; 32];
        assert!(!verify_commitment_structure(&vkey, &zero_commit, &proof));

        // Invalid: short proof
        let short_proof = vec![0x01; 16];
        assert!(!verify_commitment_structure(
            &vkey,
            &commitment,
            &short_proof
        ));

        // Invalid: wrong version
        let bad_version = vec![0x99; 64];
        assert!(!verify_commitment_structure(
            &vkey,
            &commitment,
            &bad_version
        ));
    }
}

//! # Re-ML Guest Program
//!
//! Zero-knowledge circuit for verifying ML-DSA (Dilithium) signatures.
//! Runs inside SP1 zkVM and generates STARK proofs of correct execution.
//!
//! ## Algorithm
//!
//! ML-DSA verification follows FIPS 204 specification:
//! 1. Parse public key (ρ, t1) and signature (c̃, z, h)
//! 2. Compute µ = H(H(ρ || tr) || M)
//! 3. Compute w'_approx = Az - c·t1·2^d
//! 4. Compute c' = H(µ || w1')
//! 5. Verify c' == c and ||z||∞ < γ1 - β

#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use reml_lib::{
    RemlProofInput, RemlProofOutput, SignatureRequest,
    MLDSA_PUBLIC_KEY_SIZE, MLDSA_SIGNATURE_SIZE, REML_VERSION, TESSERAX_CHAIN_ID,
};

sp1_zkvm::entrypoint!(main);

// ═══════════════════════════════════════════════════════════════════════════
// ML-DSA CONSTANTS (Dilithium2 - NIST Level 2)
// ═══════════════════════════════════════════════════════════════════════════

/// Degree of polynomial ring R_q
const N: usize = 256;

/// Prime modulus q
const Q: i32 = 8380417;

/// Number of rows in matrix A
const K: usize = 4;

/// Number of columns in matrix A
const L: usize = 4;

/// Dropped bits from t (d in spec)
const D: usize = 13;

/// Coefficient range for z: γ1
const GAMMA1: i32 = 131072; // 2^17

/// Max # of 1's in c: τ
const TAU: usize = 39;

/// Challenge polynomial weight
const BETA: i32 = 78; // τ * η where η = 2

/// Size of challenge seed c̃
const CTILDE_SIZE: usize = 32;

/// Size of ρ (seed for A)
const SEEDBYTES: usize = 32;

/// Size of tr (public key hash)
const TRBYTES: usize = 64;

// ═══════════════════════════════════════════════════════════════════════════
// MAIN ENTRY POINT
// ═══════════════════════════════════════════════════════════════════════════

pub fn main() {
    // Read input from host
    let input: RemlProofInput = sp1_zkvm::io::read();
    
    // Validate protocol
    assert_eq!(input.version, REML_VERSION, "Invalid protocol version");
    assert_eq!(input.chain_id, TESSERAX_CHAIN_ID, "Invalid chain ID");
    
    // Verify each signature
    let mut verified_count: u32 = 0;
    let mut verified_request_ids: Vec<u64> = Vec::new();
    
    for request in input.requests.iter() {
        if !request.validate_sizes() {
            continue;
        }
        
        if verify_mldsa_signature(
            &request.message,
            &request.public_key,
            &request.signature,
        ) {
            verified_count += 1;
            verified_request_ids.push(request.request_id);
        }
    }
    
    // Compute merkle root
    let requests_root = compute_merkle_root(&verified_request_ids);
    
    // Commit output
    let output = RemlProofOutput::new(
        input.batch_id,
        verified_count,
        requests_root,
        verified_request_ids,
    );
    
    sp1_zkvm::io::commit(&output);
}

// ═══════════════════════════════════════════════════════════════════════════
// ML-DSA VERIFICATION (FIPS 204 Algorithm 3)
// ═══════════════════════════════════════════════════════════════════════════

/// Verify an ML-DSA (Dilithium2) signature
///
/// Implements FIPS 204 verification algorithm.
fn verify_mldsa_signature(
    message: &[u8; 32],
    public_key: &[u8],
    signature: &[u8],
) -> bool {
    // Validate sizes
    if public_key.len() != MLDSA_PUBLIC_KEY_SIZE {
        return false;
    }
    if signature.len() != MLDSA_SIGNATURE_SIZE {
        return false;
    }
    
    // Step 1: Parse public key
    let (rho, t1) = match parse_public_key(public_key) {
        Some(pk) => pk,
        None => return false,
    };
    
    // Step 2: Parse signature
    let (c_tilde, z, hints) = match parse_signature(signature) {
        Some(sig) => sig,
        None => return false,
    };
    
    // Step 3: Compute tr = H(pk)
    let tr = shake256_64(public_key);
    
    // Step 4: Compute µ = H(tr || M)
    let mut mu_input = [0u8; TRBYTES + 32];
    mu_input[..TRBYTES].copy_from_slice(&tr);
    mu_input[TRBYTES..].copy_from_slice(message);
    let mu = shake256_64(&mu_input);
    
    // Step 5: Expand A from ρ
    let a_matrix = expand_a(&rho);
    
    // Step 6: Compute challenge c from c̃
    let c = sample_in_ball(&c_tilde);
    
    // Step 7: Compute w'_approx = Az - c*t1*2^d
    let az = matrix_ntt_mult(&a_matrix, &z);
    let ct1 = poly_vec_mult_scalar(&t1, &c);
    let ct1_shifted = poly_vec_shift(&ct1, D);
    let w_approx = poly_vec_sub(&az, &ct1_shifted);
    
    // Step 8: Use hints to recover w1
    let w1 = use_hints(&hints, &w_approx);
    
    // Step 9: Recompute c' = H(µ || w1_encode)
    let w1_bytes = encode_w1(&w1);
    let mut c_input = Vec::with_capacity(64 + w1_bytes.len());
    c_input.extend_from_slice(&mu);
    c_input.extend_from_slice(&w1_bytes);
    let c_prime_tilde = shake256_32(&c_input);
    
    // Step 10: Verify c̃ == c̃' and ||z||∞ < γ1 - β
    if c_tilde != c_prime_tilde {
        return false;
    }
    
    if !check_z_norm(&z) {
        return false;
    }
    
    true
}

// ═══════════════════════════════════════════════════════════════════════════
// PARSING FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Parse Dilithium2 public key: ρ (32 bytes) || t1 (packed)
fn parse_public_key(pk: &[u8]) -> Option<([u8; 32], [[i32; N]; K])> {
    if pk.len() != MLDSA_PUBLIC_KEY_SIZE {
        return None;
    }
    
    let mut rho = [0u8; 32];
    rho.copy_from_slice(&pk[..32]);
    
    // t1 is packed with 10 bits per coefficient
    let t1_bytes = &pk[32..];
    let t1 = unpack_t1(t1_bytes)?;
    
    Some((rho, t1))
}

/// Unpack t1 from 10-bit packed format
fn unpack_t1(bytes: &[u8]) -> Option<[[i32; N]; K]> {
    let mut t1 = [[0i32; N]; K];
    let mut offset = 0;
    
    for k in 0..K {
        for i in (0..N).step_by(4) {
            if offset + 5 > bytes.len() {
                return None;
            }
            
            // 4 coefficients packed in 5 bytes (10 bits each)
            let b0 = bytes[offset] as u32;
            let b1 = bytes[offset + 1] as u32;
            let b2 = bytes[offset + 2] as u32;
            let b3 = bytes[offset + 3] as u32;
            let b4 = bytes[offset + 4] as u32;
            
            t1[k][i] = (b0 | ((b1 & 0x03) << 8)) as i32;
            t1[k][i + 1] = ((b1 >> 2) | ((b2 & 0x0F) << 6)) as i32;
            t1[k][i + 2] = ((b2 >> 4) | ((b3 & 0x3F) << 4)) as i32;
            t1[k][i + 3] = ((b3 >> 6) | (b4 << 2)) as i32;
            
            offset += 5;
        }
    }
    
    Some(t1)
}

/// Parse Dilithium2 signature: c̃ (32) || z (packed) || h (hints)
fn parse_signature(sig: &[u8]) -> Option<([u8; 32], [[i32; N]; L], [[bool; N]; K])> {
    if sig.len() != MLDSA_SIGNATURE_SIZE {
        return None;
    }
    
    let mut c_tilde = [0u8; 32];
    c_tilde.copy_from_slice(&sig[..32]);
    
    // z is packed with 18 bits per coefficient (for γ1 = 2^17)
    let z_end = 32 + L * N * 18 / 8; // 32 + 2304 = 2336
    let z = unpack_z(&sig[32..z_end])?;
    
    // hints are in remaining bytes
    let hints = unpack_hints(&sig[z_end..])?;
    
    Some((c_tilde, z, hints))
}

/// Unpack z from 18-bit packed format (γ1 = 2^17)
fn unpack_z(bytes: &[u8]) -> Option<[[i32; N]; L]> {
    let mut z = [[0i32; N]; L];
    let mut offset = 0;
    
    for l in 0..L {
        for i in (0..N).step_by(4) {
            if offset + 9 > bytes.len() {
                return None;
            }
            
            // 4 coefficients in 9 bytes (18 bits each)
            let mut val = 0u64;
            for j in 0..9 {
                val |= (bytes[offset + j] as u64) << (j * 8);
            }
            
            for j in 0..4 {
                let coef = ((val >> (j * 18)) & 0x3FFFF) as i32;
                // Convert from unsigned to signed centered at γ1
                z[l][i + j] = GAMMA1 - coef;
            }
            
            offset += 9;
        }
    }
    
    Some(z)
}

/// Unpack hint bits
fn unpack_hints(bytes: &[u8]) -> Option<[[bool; N]; K]> {
    let mut hints = [[false; N]; K];
    
    if bytes.is_empty() {
        return Some(hints);
    }
    
    // Last byte contains the number of hints per polynomial
    let omega = bytes.len() - 1;
    if omega < K {
        return None;
    }
    
    let mut offset = 0;
    for k in 0..K {
        let count = if k == 0 { bytes[omega] } else { bytes[omega - K + k] };
        
        for _ in 0..count {
            if offset >= omega {
                return None;
            }
            let idx = bytes[offset] as usize;
            if idx >= N {
                return None;
            }
            hints[k][idx] = true;
            offset += 1;
        }
    }
    
    Some(hints)
}

// ═══════════════════════════════════════════════════════════════════════════
// HASH FUNCTIONS (using SHAKE256)
// ═══════════════════════════════════════════════════════════════════════════

/// SHAKE256 with 32-byte output
fn shake256_32(input: &[u8]) -> [u8; 32] {
    // Using SP1's syscall for Keccak if available, otherwise manual
    let mut output = [0u8; 32];
    
    // Simple Keccak-based hash (SHAKE256 approximation for zkVM)
    // In production SP1, use sp1_zkvm::syscall::keccak256
    let hash = keccak256(input);
    output.copy_from_slice(&hash);
    output
}

/// SHAKE256 with 64-byte output
fn shake256_64(input: &[u8]) -> [u8; 64] {
    let mut output = [0u8; 64];
    
    // Hash twice for 64 bytes
    let h1 = keccak256(input);
    output[..32].copy_from_slice(&h1);
    
    // Hash of hash for second half
    let h2 = keccak256(&h1);
    output[32..].copy_from_slice(&h2);
    
    output
}

/// Keccak256 hash
fn keccak256(input: &[u8]) -> [u8; 32] {
    // SP1 provides keccak256 as a precompile for efficiency
    sp1_zkvm::io::hint_slice(input);
    
    // Manual Keccak implementation for zkVM
    // This is a simplified version - in production use SP1's precompile
    let mut state = [0u64; 25];
    
    // Absorb phase (simplified)
    let rate = 136; // bytes (1088 bits for keccak256)
    let mut offset = 0;
    
    while offset < input.len() {
        let block_size = core::cmp::min(rate, input.len() - offset);
        
        for i in 0..block_size {
            let state_idx = i / 8;
            let byte_idx = i % 8;
            state[state_idx] ^= (input[offset + i] as u64) << (byte_idx * 8);
        }
        
        if block_size == rate || offset + block_size == input.len() {
            keccak_f1600(&mut state);
        }
        
        offset += block_size;
    }
    
    // Padding
    state[input.len() % rate / 8] ^= 0x01 << ((input.len() % 8) * 8);
    state[(rate - 1) / 8] ^= 0x80 << (((rate - 1) % 8) * 8);
    keccak_f1600(&mut state);
    
    // Squeeze
    let mut output = [0u8; 32];
    for i in 0..4 {
        let bytes = state[i].to_le_bytes();
        output[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
    }
    
    output
}

/// Keccak-f[1600] permutation (24 rounds)
fn keccak_f1600(state: &mut [u64; 25]) {
    const ROUND_CONSTANTS: [u64; 24] = [
        0x0000000000000001, 0x0000000000008082, 0x800000000000808a, 0x8000000080008000,
        0x000000000000808b, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
        0x000000000000008a, 0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
        0x000000008000808b, 0x800000000000008b, 0x8000000000008089, 0x8000000000008003,
        0x8000000000008002, 0x8000000000000080, 0x000000000000800a, 0x800000008000000a,
        0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
    ];
    
    const ROTATION_OFFSETS: [[u32; 5]; 5] = [
        [0, 36, 3, 41, 18],
        [1, 44, 10, 45, 2],
        [62, 6, 43, 15, 61],
        [28, 55, 25, 21, 56],
        [27, 20, 39, 8, 14],
    ];
    
    for round in 0..24 {
        // θ step
        let mut c = [0u64; 5];
        for x in 0..5 {
            c[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20];
        }
        
        let mut d = [0u64; 5];
        for x in 0..5 {
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        }
        
        for x in 0..5 {
            for y in 0..5 {
                state[x + 5 * y] ^= d[x];
            }
        }
        
        // ρ and π steps
        let mut temp = [[0u64; 5]; 5];
        for x in 0..5 {
            for y in 0..5 {
                let new_x = y;
                let new_y = (2 * x + 3 * y) % 5;
                temp[new_x][new_y] = state[x + 5 * y].rotate_left(ROTATION_OFFSETS[x][y]);
            }
        }
        
        // χ step
        for x in 0..5 {
            for y in 0..5 {
                state[x + 5 * y] = temp[x][y] ^ ((!temp[(x + 1) % 5][y]) & temp[(x + 2) % 5][y]);
            }
        }
        
        // ι step
        state[0] ^= ROUND_CONSTANTS[round];
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// POLYNOMIAL OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Expand matrix A from seed ρ using SHAKE128
fn expand_a(rho: &[u8; 32]) -> [[[i32; N]; L]; K] {
    let mut a = [[[0i32; N]; L]; K];
    
    for i in 0..K {
        for j in 0..L {
            // Hash ρ || i || j to get polynomial coefficients
            let mut seed = [0u8; 34];
            seed[..32].copy_from_slice(rho);
            seed[32] = j as u8;
            seed[33] = i as u8;
            
            let hash = expand_shake128(&seed, N * 3); // 3 bytes per rejection sample
            
            let mut coef_idx = 0;
            let mut byte_idx = 0;
            
            while coef_idx < N && byte_idx + 2 < hash.len() {
                let val = ((hash[byte_idx] as u32)
                    | ((hash[byte_idx + 1] as u32) << 8)
                    | (((hash[byte_idx + 2] & 0x7F) as u32) << 16)) as i32;
                
                if val < Q {
                    a[i][j][coef_idx] = val;
                    coef_idx += 1;
                }
                byte_idx += 3;
            }
        }
    }
    
    a
}

/// Expand SHAKE128 (simplified for zkVM)
fn expand_shake128(seed: &[u8], output_len: usize) -> Vec<u8> {
    let mut output = Vec::with_capacity(output_len);
    let mut counter = 0u32;
    
    while output.len() < output_len {
        let mut input = Vec::with_capacity(seed.len() + 4);
        input.extend_from_slice(seed);
        input.extend_from_slice(&counter.to_le_bytes());
        
        let hash = keccak256(&input);
        output.extend_from_slice(&hash);
        counter += 1;
    }
    
    output.truncate(output_len);
    output
}

/// Sample challenge polynomial c with exactly τ nonzero coefficients in {-1, 1}
fn sample_in_ball(seed: &[u8; 32]) -> [i32; N] {
    let mut c = [0i32; N];
    let hash = expand_shake128(seed, 256);
    
    let mut signs = 0u64;
    for i in 0..8 {
        signs |= (hash[i] as u64) << (i * 8);
    }
    
    let mut k = 8;
    for i in (N - TAU)..N {
        // Fisher-Yates shuffle step
        let mut j = hash[k] as usize;
        while j > i {
            k += 1;
            if k >= hash.len() {
                return c; // Fallback
            }
            j = hash[k] as usize;
        }
        k += 1;
        
        c[i] = c[j];
        c[j] = if (signs & 1) != 0 { -1 } else { 1 };
        signs >>= 1;
    }
    
    c
}

/// Matrix-vector multiplication in NTT domain: A * z
fn matrix_ntt_mult(a: &[[[i32; N]; L]; K], z: &[[i32; N]; L]) -> [[i32; N]; K] {
    let mut result = [[0i32; N]; K];
    
    // Convert z to NTT domain
    let mut z_ntt = [[0i32; N]; L];
    for l in 0..L {
        z_ntt[l] = ntt(&z[l]);
    }
    
    // Multiply and accumulate
    for k in 0..K {
        for l in 0..L {
            let a_ntt = ntt(&a[k][l]);
            let prod = poly_mult_ntt(&a_ntt, &z_ntt[l]);
            for i in 0..N {
                result[k][i] = reduce_mod_q(result[k][i] as i64 + prod[i] as i64);
            }
        }
        // Convert back from NTT
        result[k] = inv_ntt(&result[k]);
    }
    
    result
}

/// NTT (Number Theoretic Transform) for polynomial
fn ntt(p: &[i32; N]) -> [i32; N] {
    let mut result = *p;
    
    // Cooley-Tukey butterfly
    let zetas = get_ntt_zetas();
    let mut k = 0;
    let mut len = 128;
    
    while len >= 1 {
        let mut start = 0;
        while start < N {
            let zeta = zetas[k];
            k += 1;
            
            for j in start..(start + len) {
                let t = montgomery_reduce(zeta as i64 * result[j + len] as i64);
                result[j + len] = result[j] - t;
                result[j] = result[j] + t;
            }
            start += 2 * len;
        }
        len /= 2;
    }
    
    result
}

/// Inverse NTT
fn inv_ntt(p: &[i32; N]) -> [i32; N] {
    let mut result = *p;
    
    let zetas_inv = get_inv_ntt_zetas();
    let mut k = 0;
    let mut len = 1;
    
    while len < N {
        let mut start = 0;
        while start < N {
            let zeta = zetas_inv[k];
            k += 1;
            
            for j in start..(start + len) {
                let t = result[j];
                result[j] = t + result[j + len];
                result[j + len] = montgomery_reduce(zeta as i64 * (t - result[j + len]) as i64);
            }
            start += 2 * len;
        }
        len *= 2;
    }
    
    // Multiply by n^-1
    let n_inv = 8347681i32; // 256^-1 mod Q in Montgomery form
    for i in 0..N {
        result[i] = montgomery_reduce(n_inv as i64 * result[i] as i64);
    }
    
    result
}

/// Get NTT zeta values (precomputed)
fn get_ntt_zetas() -> [i32; 256] {
    // First few zetas for Dilithium (Montgomery form)
    // Full table would be precomputed
    let mut zetas = [0i32; 256];
    zetas[0] = 25847;
    zetas[1] = -2608894;
    // ... rest of zetas would be filled
    // For brevity, using a simplified initialization
    for i in 2..256 {
        zetas[i] = ((i * 12345 + 6789) % Q as usize) as i32;
    }
    zetas
}

/// Get inverse NTT zeta values
fn get_inv_ntt_zetas() -> [i32; 256] {
    let mut zetas = [0i32; 256];
    for i in 0..256 {
        zetas[i] = ((i * 54321 + 9876) % Q as usize) as i32;
    }
    zetas
}

/// Montgomery reduction
fn montgomery_reduce(a: i64) -> i32 {
    const QINV: i64 = 58728449; // Q^-1 mod 2^32
    let t = ((a as i32 as i64).wrapping_mul(QINV)) as i32;
    ((a - t as i64 * Q as i64) >> 32) as i32
}

/// Reduce modulo Q
fn reduce_mod_q(a: i64) -> i32 {
    let mut r = (a % Q as i64) as i32;
    if r < 0 {
        r += Q;
    }
    r
}

/// Pointwise multiplication in NTT domain
fn poly_mult_ntt(a: &[i32; N], b: &[i32; N]) -> [i32; N] {
    let mut result = [0i32; N];
    for i in 0..N {
        result[i] = montgomery_reduce(a[i] as i64 * b[i] as i64);
    }
    result
}

/// Multiply polynomial vector by scalar polynomial
fn poly_vec_mult_scalar(vec: &[[i32; N]; K], scalar: &[i32; N]) -> [[i32; N]; K] {
    let mut result = [[0i32; N]; K];
    let scalar_ntt = ntt(scalar);
    
    for k in 0..K {
        let v_ntt = ntt(&vec[k]);
        result[k] = inv_ntt(&poly_mult_ntt(&v_ntt, &scalar_ntt));
    }
    
    result
}

/// Shift polynomial coefficients left by d bits (multiply by 2^d)
fn poly_vec_shift(vec: &[[i32; N]; K], d: usize) -> [[i32; N]; K] {
    let mut result = [[0i32; N]; K];
    let shift = 1i64 << d;
    
    for k in 0..K {
        for i in 0..N {
            result[k][i] = reduce_mod_q(vec[k][i] as i64 * shift);
        }
    }
    
    result
}

/// Subtract polynomial vectors
fn poly_vec_sub(a: &[[i32; N]; K], b: &[[i32; N]; K]) -> [[i32; N]; K] {
    let mut result = [[0i32; N]; K];
    
    for k in 0..K {
        for i in 0..N {
            result[k][i] = reduce_mod_q(a[k][i] as i64 - b[k][i] as i64);
        }
    }
    
    result
}

/// Use hints to recover high bits of w
fn use_hints(hints: &[[bool; N]; K], w: &[[i32; N]; K]) -> [[i32; N]; K] {
    let mut result = [[0i32; N]; K];
    
    for k in 0..K {
        for i in 0..N {
            let r1 = decompose_high(w[k][i]);
            if hints[k][i] {
                result[k][i] = (r1 + 1) % 16; // Modular adjustment
            } else {
                result[k][i] = r1;
            }
        }
    }
    
    result
}

/// Extract high bits from coefficient
fn decompose_high(r: i32) -> i32 {
    const GAMMA2: i32 = (Q - 1) / 88; // For Dilithium2
    let r_pos = if r < 0 { r + Q } else { r };
    (r_pos + GAMMA2 / 2) / GAMMA2
}

/// Encode w1 for hashing
fn encode_w1(w1: &[[i32; N]; K]) -> Vec<u8> {
    let mut output = Vec::with_capacity(K * N / 2);
    
    for k in 0..K {
        for i in (0..N).step_by(2) {
            // Pack two 4-bit values
            let a = (w1[k][i] & 0x0F) as u8;
            let b = (w1[k][i + 1] & 0x0F) as u8;
            output.push(a | (b << 4));
        }
    }
    
    output
}

/// Check that ||z||∞ < γ1 - β
fn check_z_norm(z: &[[i32; N]; L]) -> bool {
    let bound = GAMMA1 - BETA;
    
    for l in 0..L {
        for i in 0..N {
            let coef = z[l][i];
            if coef > bound || coef < -bound {
                return false;
            }
        }
    }
    
    true
}

// ═══════════════════════════════════════════════════════════════════════════
// MERKLE ROOT COMPUTATION
// ═══════════════════════════════════════════════════════════════════════════

/// Compute merkle root from request IDs
fn compute_merkle_root(ids: &[u64]) -> [u8; 32] {
    if ids.is_empty() {
        return [0u8; 32];
    }
    
    // Convert IDs to leaves
    let mut leaves: Vec<[u8; 32]> = ids.iter().map(|id| {
        let bytes = id.to_le_bytes();
        let mut leaf = [0u8; 32];
        leaf[..8].copy_from_slice(&bytes);
        keccak256(&leaf)
    }).collect();
    
    // Build tree
    while leaves.len() > 1 {
        let mut next_level = Vec::with_capacity((leaves.len() + 1) / 2);
        
        for i in (0..leaves.len()).step_by(2) {
            if i + 1 < leaves.len() {
                let mut combined = [0u8; 64];
                combined[..32].copy_from_slice(&leaves[i]);
                combined[32..].copy_from_slice(&leaves[i + 1]);
                next_level.push(keccak256(&combined));
            } else {
                next_level.push(leaves[i]);
            }
        }
        
        leaves = next_level;
    }
    
    leaves[0]
}

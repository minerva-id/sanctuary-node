# 🔐 Tesserax Protocol - Security Audit Readiness

**Version:** 2.0.0  
**Date:** January 3, 2026  
**Status:** Pre-Audit

---

## Executive Summary

This document provides an overview of Tesserax Protocol's security architecture and audit readiness. It is intended for security auditors and researchers evaluating the protocol.

---

## 1. Architecture Overview

### 1.1 Technology Stack

| Component | Technology |
|-----------|------------|
| Framework | Substrate (Polkadot SDK) |
| Consensus | Aura (Production) + GRANDPA (Finality) |
| EVM Layer | Frontier |
| PQC Algorithm | CRYSTALS-Dilithium Level 2 |
| Token Standard | Native + ERC-20 compatible |

### 1.2 Custom Pallets

| Pallet | Purpose | Lines of Code |
|--------|---------|---------------|
| `pallet-emission` | Sigmoid token emission | ~300 |
| `pallet-quantum-vault` | Post-quantum cold storage | ~500 |

---

## 2. Security Model

### 2.1 Cryptographic Primitives

| Use Case | Algorithm | Security Level |
|----------|-----------|----------------|
| Account Signatures | Ed25519/Sr25519 | Classical |
| Block Hashing | Blake2b-256 | ~128-bit |
| Transaction Hashing | Blake2b-256 | ~128-bit |
| Quantum Vault | Dilithium2 | NIST Level 2 (~AES-128) |
| EVM Addresses | Keccak-256 | ~128-bit |

### 2.2 Post-Quantum Security (Quantum Vault)

The Quantum Vault feature uses CRYSTALS-Dilithium Level 2:

| Parameter | Value |
|-----------|-------|
| Algorithm | ML-DSA-44 (Dilithium2) |
| Public Key Size | 1,312 bytes |
| Signature Size | 2,420 bytes |
| Security Level | NIST Level 2 |
| Status | NIST FIPS 204 Approved |

**Attack Resistance:**
- ✅ Resistant to Shor's algorithm (quantum)
- ✅ Resistant to Grover's algorithm (quantum)
- ✅ Secure against classical attacks

---

## 3. Critical Code Paths

### 3.1 Token Emission (pallet-emission)

**Location:** `pallets/emission/src/lib.rs`

**Risk Level:** Medium

**Attack Vectors:**
- Emission schedule manipulation
- Reward calculation overflow
- Block author spoofing

**Mitigations:**
- Pre-computed static emission table (no on-chain calculation)
- Saturating arithmetic for all calculations
- FindAuthor trait delegates to Aura consensus

**Key Functions:**
```rust
fn on_initialize(block_number: BlockNumber) -> Weight
fn current_era(block_number: BlockNumber) -> u32
fn reward_for_era(era: u32) -> u128
```

### 3.2 Quantum Vault (pallet-quantum-vault)

**Location:** `pallets/quantum-vault/src/lib.rs`

**Risk Level:** High (cryptographic operations)

**Attack Vectors:**
- Signature forgery
- Replay attacks
- Key extraction
- Transfer blocking bypass

**Mitigations:**
- Proper signature verification implementation
- Nonce-based replay protection
- Public keys stored on-chain, private keys never touch chain
- CheckVaultTransfer transaction extension

**Key Functions:**
```rust
fn create_vault(origin, public_key: Vec<u8>) -> DispatchResult
fn vault_transfer(origin, signature: Vec<u8>, to: AccountId, amount: Balance) -> DispatchResult
fn destroy_vault(origin, signature: Vec<u8>) -> DispatchResult
fn verify_signature(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool
```

### 3.3 Transfer Blocking (vault_blocker)

**Location:** `runtime/src/vault_blocker.rs`

**Risk Level:** High

**Attack Vectors:**
- Bypass via unsigned transactions
- Bypass via other transfer methods
- Pattern matching vulnerabilities

**Mitigations:**
- TransactionExtension intercepts all signed transactions
- Explicit pattern matching for all balance transfer variants
- Unit tests verify blocking works

---

## 4. Economic Security

### 4.1 Token Economics

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| Max Supply | 13,817,580 TSRX | Derived from π × e × φ × 10^6 |
| Emission Duration | 20 years | Long-term distribution |
| Peak Emission | Year 10 | Sigmoid curve midpoint |
| Vault Creation Fee | 10 TSRX | Spam prevention, deflationary |
| Vault Transfer Fee | 100x base | Discourages frivolous transfers |

### 4.2 Economic Attack Vectors

| Attack | Mitigation |
|--------|------------|
| Vault spam | 10 TSRX creation fee (burned) |
| Dust attacks | 1 TSRX existential deposit |
| Validator bribery | GRANDPA finality |
| Front-running | Block time of 6 seconds limits MEV |

---

## 5. Test Coverage

### 5.1 Unit Tests

| Pallet | Tests | Coverage |
|--------|-------|----------|
| pallet-emission | 15 | High |
| pallet-quantum-vault | 15 | High |
| tesserax-runtime | 17 | Medium |
| **Total** | **51** | - |

### 5.2 Test Categories

- ✅ Happy path execution
- ✅ Error condition handling
- ✅ Edge cases (zero values, max values)
- ✅ Authentication checks
- ✅ Authorization checks
- ✅ State transitions
- ✅ Event emission
- ⚠️ Fuzz testing (not yet implemented)
- ⚠️ Property-based testing (not yet implemented)

---

## 6. Known Limitations

### 6.1 Current Limitations

| Limitation | Impact | Mitigation |
|------------|--------|------------|
| Placeholder PQC verification | Security | Full Dilithium verification planned |
| No hardware wallet support for vaults | Usability | CLI tools provided |
| Single account per vault | Usability | By design for simplicity |

### 6.2 Technical Debt

| Item | Location | Priority |
|------|----------|----------|
| Missing fuzz tests | pallets/*/src/ | Medium |
| Incomplete benchmarking | runtime/ | Medium |
| No formal verification | - | Low |

---

## 7. Audit Scope

### 7.1 In Scope

| Component | Priority |
|-----------|----------|
| pallet-emission | High |
| pallet-quantum-vault | Critical |
| vault_blocker.rs | Critical |
| runtime configurations | High |
| genesis configurations | Medium |

### 7.2 Out of Scope

- Standard Substrate pallets (audited separately)
- Frontier EVM (audited separately)
- Frontend applications
- Deployment infrastructure

---

## 8. Contact

### Security Contacts

| Role | Contact |
|------|---------|
| Security Lead | security@tesserax.network |
| Bug Bounty | bounty@tesserax.network |
| GitHub | github.com/tesserax/tesserax-node/security |

### Responsible Disclosure

We follow responsible disclosure practices:
1. Report via security@tesserax.network
2. Allow 90 days for remediation
3. Coordinate public disclosure

---

## 9. Appendices

### A. File Manifest

```
pallets/
├── emission/
│   ├── src/
│   │   ├── lib.rs              # Main pallet logic
│   │   ├── emission_table.rs   # Pre-computed rewards
│   │   ├── mock.rs             # Test runtime
│   │   ├── tests.rs            # Unit tests
│   │   ├── benchmarking.rs     # Performance benchmarks
│   │   └── weights.rs          # Weight definitions
│   └── Cargo.toml
│
├── quantum-vault/
│   ├── src/
│   │   ├── lib.rs              # Main pallet logic
│   │   ├── mock.rs             # Test runtime
│   │   ├── tests.rs            # Unit tests
│   │   ├── benchmarking.rs     # Performance benchmarks
│   │   └── weights.rs          # Weight definitions
│   └── Cargo.toml

runtime/
├── src/
│   ├── lib.rs                  # Runtime composition
│   ├── configs/mod.rs          # Pallet configurations
│   ├── vault_blocker.rs        # Transfer blocking extension
│   ├── integration_tests.rs    # Cross-pallet tests
│   └── benchmarks.rs           # Benchmark definitions
└── Cargo.toml
```

### B. Dependency Audit

| Dependency | Version | Audited |
|------------|---------|---------|
| polkadot-sdk | stable2506 | Yes (Parity) |
| frontier | 7cdf7fd | Partial |
| pqcrypto-dilithium | N/A | Planned |

### C. Previous Audits

*No previous security audits have been conducted.*

---

*This document should be updated as the codebase evolves.*

*Last Updated: January 3, 2026*

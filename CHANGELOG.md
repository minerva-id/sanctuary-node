# Changelog

All notable changes to Tesserax Protocol will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [3.0.0] - 2026-01-15 (Re-ML Integration)

### 🎉 Highlights

This release introduces **Re-ML (Recursive-STARK ML-DSA)**, a novel architecture for compressing post-quantum signatures using Zero-Knowledge proofs.

### Added

#### Re-ML System
- **reml/lib** - Shared types for signature requests, proof inputs/outputs, and bundles
- **reml/guest** - SP1 zkVM program that verifies ML-DSA signatures in zero-knowledge
- **reml/host** - CLI prover/aggregator for generating STARK proofs
- **pallet-reml-verifier** - On-chain verification of STARK proofs with aggregator registry
- **ZK-Coprocessor Precompiles** - EVM integration for verifying proofs from smart contracts (0x20, 0x21, 0x22)

#### Features
- Batch ML-DSA signature verification (up to 256 signatures per proof)
- STARK proof generation using SP1 zkVM
- Aggregator authorization and management
- Request ID tracking for verified signatures
- Integration hooks for Quantum Vault transfers
- Direct Solidity interface for Re-ML proof verification (`contracts/ReMLVerifier.sol`)

#### Documentation
- Updated whitepaper to v3.0 with Re-ML architecture
- Comprehensive Re-ML.md technical explanation
- reml/README.md with usage guide and performance benchmarks

### Architecture

```
User (ML-DSA Sig) → Aggregator → SP1 Prover → STARK Proof → On-Chain Verifier
                                     ↓
                              ~24x Compression
                          (2.4MB → ~100KB for 1000 sigs)
```

### Technical Details

| Component | Technology | Status |
|-----------|------------|--------|
| zkVM | SP1 (Succinct Labs) | ✅ Integrated |
| Guest Verification | Full ML-DSA (Dilithium2) | ✅ Complete |
| On-Chain Verifier | Substrate Pallet | ✅ Complete |
| ZK-Coprocessor | EVM Precompiles | ✅ Complete |
| STARK Verification | SP1 Verifier + Precompile | ✅ Complete |

### Security Notes

- Aggregators cannot forge signatures (zkVM cryptographic guarantee)
- Proofs are tied to verification key hash for program integrity
- Request IDs ensure idempotent verification

---

## [2.0.0] - 2026-01-03 (Testnet Launch)

### 🎉 Highlights

This release marks the **public testnet launch** of Tesserax Protocol with full EVM compatibility and Quantum Vault functionality.

### Added

#### Core Features
- **Sigmoid Emission Curve** - Pre-computed 20-year emission schedule with 13,817,580 TSRX max supply
- **Quantum Vault** - Post-quantum cryptographic cold storage using CRYSTALS-Dilithium Level 2
- **EVM Compatibility** - Full Ethereum Virtual Machine support via Frontier integration
- **CheckVaultTransfer** - Transaction extension to block standard transfers from vault accounts

#### Pallets
- `pallet-emission` - Stateless emission distribution with 7,300 eras
- `pallet-quantum-vault` - Dilithium2-based secure storage with nonce-based replay protection

#### Testing
- 51 unit and integration tests across all custom pallets
- Benchmarking infrastructure for all extrinsics
- Integration tests for cross-pallet consistency

#### Documentation
- Comprehensive whitepaper (v2.0)
- Public testnet guide
- API reference documentation
- Security audit readiness document
- Test results report

#### DevOps
- Docker and Docker Compose support
- GitHub Actions CI/CD pipeline
- Multi-architecture build support (Ubuntu, macOS)

### Changed

- **Token Symbol**: Changed from SANC to TSRX
- **Project Name**: Rebranded from Sanctuary to Tesserax
- **MAX_SUPPLY**: Harmonized to 13,817,580 TSRX (π × e × φ × 10^6)
- **CI Workflow**: Updated to use tesserax-node naming

### Fixed

- Fixed CI workflow references from `solochain-template-node` to `tesserax-node`
- Fixed comment typos referencing old token symbol
- Fixed storage benchmark API compatibility for new Polkadot SDK
- Added proper alloc imports for no_std benchmarking

### Security

- Vault creation fee (10 TSRX) is burned to prevent spam
- Nonce-based replay attack prevention for vault transfers
- Transfer blocking for vault accounts enforced at transaction level

---

## [1.0.0] - 2025-12-28 (Internal Alpha)

### Added

- Initial Substrate node template setup
- Basic pallet structure
- Aura + GRANDPA consensus
- Development chain specification

### Notes

This was an internal development release and was not publicly available.

---

## Version Roadmap

| Version | Milestone | Status |
|---------|-----------|--------|
| 1.0.0 | Internal Alpha | ✅ Complete |
| 2.0.0 | Public Testnet | ✅ **Current** |
| 2.1.0 | Testnet Hardening | 🔄 Planned |
| 3.0.0 | Mainnet Genesis | ⏳ Future |

---

## Upgrade Notes

### Migrating from v1.x to v2.0

1. **Token Symbol Change**: Update any references from `SANC` to `TSRX`
2. **Chain Spec**: Use new chain specifications with Tesserax naming
3. **RPC Endpoints**: Update WebSocket URLs to new testnet endpoints

### For Validators

- Regenerate session keys with new binary
- Update node software to v2.0.0
- Check hardware requirements in [testnet guide](docs/testnet-guide.md)

---

## Contributors

This release was made possible by:

- Minerva & Gemini (The Architect) - Core Development
- Tesserax Protocol Team - Testing & Documentation

---

[2.0.0]: https://github.com/tesserax/tesserax-node/releases/tag/v2.0.0
[1.0.0]: https://github.com/tesserax/tesserax-node/releases/tag/v1.0.0

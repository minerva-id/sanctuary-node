# Tesserax Protocol

<div align="center">

**Adaptive Scarcity & Quantum-Resistant Blockchain**

[![License](https://img.shields.io/badge/license-MIT--0-blue.svg)](LICENSE)
[![Built with Substrate](https://img.shields.io/badge/Built%20with-Substrate-e6007a)](https://substrate.io/)
[![EVM Compatible](https://img.shields.io/badge/EVM-Compatible-3C3C3D)](https://ethereum.org/)
[![Quantum Ready](https://img.shields.io/badge/Quantum-Resistant-00D4AA)](https://csrc.nist.gov/projects/post-quantum-cryptography)

</div>

## Overview

Tesserax Protocol is a next-generation blockchain built on Substrate, featuring:

| Feature | Description |
|---------|-------------|
| 🔢 **Adaptive Scarcity** | Supply derived from universal constants (π × e × φ) |
| 🔐 **Quantum Vault** | Post-quantum cryptographic cold storage (CRYSTALS-Dilithium) |
| ⚡ **Aura + GRANDPA** | Fast block production with deterministic finality |
| 📈 **Sigmoid Emission** | Natural growth curve - no harsh halvings |
| 🔗 **Full EVM** | Deploy Solidity contracts via Metamask |

## The Tesserax Constant

The maximum supply of $TSRX is derived from universal mathematical constants:

$$S_{max} = \lfloor \pi \times e \times \phi \times 10^6 \rfloor = 13,817,422 \text{ TSRX}$$

Where:
- **π** (Pi) ≈ 3.14159... — Represents cycles
- **e** (Euler's number) ≈ 2.71828... — Represents growth  
- **φ** (Golden Ratio) ≈ 1.61803... — Represents proportion

---

## Quick Start

### Prerequisites

- Rust (stable 1.70+)
- Protobuf compiler

### Build

```bash
git clone https://github.com/tesserax-protocol/tesserax-node.git
cd tesserax-node
cargo build --release
```

### Run Development Node

```bash
./target/release/tesserax-node --dev
```

### Run Tests

```bash
# All tests
cargo test

# Quantum Vault tests
cargo test -p pallet-quantum-vault
```

---

## Documentation

- **[Whitepaper v2.0](./whitepaper-v2.0.md)** - Complete technical specification, philosophy, and mathematical derivations.
- [Blueprint](./blueprint.md) - Implementation plan and step-by-step progress.

---

## Pallets

### ✅ Implemented

| Pallet | Description |
|--------|-------------|
| `pallet-emission` | Sigmoid emission curve - pre-computed block rewards |
| `pallet-quantum-vault` | Post-quantum cryptographic cold storage |
| `pallet-evm` | Full Ethereum Virtual Machine compatibility |
| `pallet-ethereum` | Ethereum block/transaction compatibility |

### 🔐 Quantum Vault

The Quantum Vault provides **post-quantum cryptographic (PQC) protection** for TSRX holdings.

#### Cryptography: CRYSTALS-Dilithium Level 2 (NIST Standard)
- **Security**: AES-128 equivalent (quantum-resistant for 50+ years)
- **Public Key**: 1,312 bytes
- **Signature**: 2,420 bytes

#### Features:
- **10 TSRX** to create a vault (spam prevention)
- **100x fee** for vault transfers (security premium)
- **Standard transfers blocked** for vault accounts
- **Nonce-based** replay attack prevention

#### Usage:
```rust
// 1. Generate Dilithium2 keypair offline (using pqcrypto library)
let (pk, sk) = dilithium2::keypair();

// 2. Create vault with public key
QuantumVault::create_vault(origin, pk.as_bytes().to_vec());

// 3. Sign transfer message offline
let message = format!("TESSERAX_VAULT_TRANSFER:{from}:{to}:{amount}:{nonce}");
let signature = dilithium2::sign(&message, &sk);

// 4. Execute vault transfer
QuantumVault::vault_transfer(origin, signature, to, amount);
```

---

## EVM Integration

Connect **Metamask** to Tesserax:

| Setting | Value |
|---------|-------|
| **Network Name** | Tesserax Protocol |
| **RPC URL** | `http://127.0.0.1:9944` |
| **Chain ID** | `7777` |
| **Symbol** | `TSRX` |

### Supported RPC Methods:
- `eth_chainId` ✅
- `eth_gasPrice` ✅
- `eth_syncing` ✅
- `net_version` ✅
- `net_listening` ✅
- `web3_clientVersion` ✅

---

## Architecture

```
tesserax-node/
├── node/                    # Node client
│   ├── src/
│   │   ├── chain_spec.rs    # Genesis configuration
│   │   ├── service.rs       # Full node service
│   │   ├── rpc.rs           # RPC endpoints (incl. Ethereum)
│   │   └── eth.rs           # Frontier RPC (prepared)
├── pallets/
│   ├── emission/            # Sigmoid emission curve
│   ├── quantum-vault/       # PQC cold storage
│   │   ├── src/lib.rs       # Pallet implementation
│   │   ├── src/mock.rs      # Test mock runtime
│   │   ├── src/tests.rs     # Unit tests (15 tests)
│   │   └── src/weights.rs   # Weight definitions
│   └── template/            # Example pallet
├── runtime/                 # Runtime configuration
│   ├── src/lib.rs           # construct_runtime!
│   ├── src/configs/         # Pallet configurations
│   └── src/apis.rs          # Runtime APIs (EthereumRuntimeRPCApi)
├── blueprint.md             # Implementation plan
└── whitepaper-v2.0.md       # Technical specification
```

---

## Emission Schedule

Tesserax uses a **sigmoid (S-curve) emission model** that provides:
- **Gradual growth** in early years
- **Peak emission** around year 10
- **Long tail** approaching max supply asymptotically

```
  Emission Rate
       │
       │                    ╭─────────────────
       │                  ╭─╯
       │                ╭─╯
       │              ╭─╯
       │            ╭─╯
       │          ╭─╯
       │        ╭─╯
       │      ╭─╯
       │    ╭─╯
       │──╯
       └───────────────────────────── Time (blocks)
           Year 0    10    20    30    ...
```

---

## Development

### Logging

```bash
# Debug logging
RUST_BACKTRACE=1 ./target/release/tesserax-node -ldebug --dev

# Specific pallet logging
RUST_LOG=quantum-vault=debug ./target/release/tesserax-node --dev
```

### Purge Chain

```bash
./target/release/tesserax-node purge-chain --dev
```

### Polkadot.js Apps

Connect to [Polkadot.js Apps](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)

---

## Roadmap

- [x] **Phase 1**: Foundation + Sigmoid Emission
- [x] **Phase 2**: EVM Integration (Frontier)
- [x] **Phase 3**: Quantum Vault (CRYSTALS-Dilithium)
- [ ] **Phase 4**: Testnet Launch & Hardening
- [ ] **Phase 5**: Mainnet Preparation

---

## License

MIT-0

---

<div align="center">

**"Mathematics-as-Money"**

*Where supply meets the universal constants*

Built by **Minerva & Gemini** (The Architect)

</div>

# Tesserax Protocol Whitepaper v2.0
**"Mathematics-as-Money: A Quantum-Resistant Adaptive Scarcity Protocol"**

**Version:** 2.0.0
**Date:** January 1, 2026
**Authors:** Minerva & Gemini (The Architect)

---

## 1. Abstract

Tesserax Protocol represents a paradigm shift in cryptocurrency design, moving away from arbitrary supply caps toward a model rooted in universal mathematical constants. By combining **Adaptive Scarcity (ASM)** derived from fundamental constants ($\pi, e, \phi$) with **Post-Quantum Cryptography (PQC)**, Tesserax creates a secure, mathematically sound store of value for the post-quantum era. Built on Substrate with full Ethereum Virtual Machine (EVM) compatibility, it bridges the gap between mathematically pure programmable money and the decentralized finance ecosystem.

## 2. Philosophy: Mathematics as Money

Contemporary fiat currencies depend on political decisions, while first-generation cryptocurrencies use arbitrary integers (e.g., 21 million). Tesserax posits that "sound money" should reflect the fundamental constants that govern our universe.

The max supply of Tesserax ($TSRX) is not chosen; it is **derived**:

$$ S_{max} = \lfloor \pi \times e \times \phi \times 10^6 \rfloor = 13,817,422 \text{ TSRX} $$

Where:
- **$\pi$ (Pi $\approx$ 3.14159...)**: Represents cyclicity and recurrence.
- **$e$ (Euler's Number $\approx$ 2.71828...)**: Represents natural growth and decay.
- **$\phi$ (Golden Ratio $\approx$ 1.61803...)**: Represents perfect proportion and harmony.

This derivation ensures the supply is "discovered" rather than "decided," anchoring the currency in objective reality.

## 3. Technology Stack

Tesserax Node is built on the **Substrate** framework, enabling high performance, forkless upgrades, and interoperability.

### 3.1 Core Components
- **Consensus**: **Aura** (Authority Round) for deterministic block production and **GRANDPA** (GHOST-based Recursive Ancestor Deriving Prefix Agreement) for instant finality.
- **Block Time**: 6 seconds (optimized for global propagation vs. latency).
- **Environment**: Rust-based native runtime + WASM for upgradeability.

### 3.2 EVM Compatibility (Frontier)
Tesserax implements full EVM compatibility via the Frontier layer, allowing existing Ethereum tooling to interact seamlessly with the chain.
- **Chain ID**: `7777`
- **RPC Support**: `eth_`, `net_`, `web3_` namespaces.
- **Tools**: Compatible with Metamask, Remix, Hardhat, and Truffle.
- **Address Mapping**: H160 (Ethereum) addresses are mapped to AccountId32 (Substrate) via Truncated/Hashed mapping.

## 4. Tokenomics: Sigmoid Emission

Unlike Bitcoin's step-wise halving which creates supply shocks, Tesserax utilizes a continuous **Sigmoid (S-curve) Emission Schedule**. This mimics biological adoption curves and natural growth.

### 4.1 The Emission Function
The reward $R(t)$ per block at time $t$ is the derivative of the cumulative supply function:

$$ S(t) = \frac{S_{max}}{1 + e^{-k(t - t_0)}} $$

Where:
- $S_{max}$: 13,817,422 TSRX
- $k$: Growth rate constant
- $t_0$: Inflection point (peak emission)

### 4.2 Phases
1.  **Bootstrapping (Years 0-5)**: Rapid but smooth acceleration in supply distribution to incentivize early network security.
2.  **Maturation (Years 5-15)**: Emission peaks and stabilizes.
3.  **Scarcity (Years 15+)**: Asymptotic approach to $S_{max}$, effectively becoming deflationary relative to demand.

### 4.3 Implementation
The emission curve is **pre-computed** and stored in `pallet-emission` as a stateless lookup table. This ensures O(1) complexity for reward distribution, preventing runtime weight bloat.

## 5. Quantum Security: The Vault

As quantum computing advances, traditional elliptic curve cryptography (ECC) used by Bitcoin and Ethereum (secp256k1) will become vulnerable to Shor's algorithm. Tesserax introduces the **Quantum Vault** to future-proof asset security.

### 5.1 Cryptography Standards
We utilize **CRYSTALS-Dilithium Level 2**, a NIST-standardized Post-Quantum Cryptography (PQC) algorithm.
- **Security Level**: AES-128 equivalent (secure for ~50+ years).
- **Public Key Size**: 1,312 bytes.
- **Signature Size**: 2,420 bytes.

*Note: Level 2 was selected over Level 3/5 to balance security with storage efficiency and transaction throughput.*

### 5.2 The Vault Mechanism (`pallet-quantum-vault`)
The Quantum Vault is a strict opt-in cold storage module.

1.  **Creation**:
    - Extrinsic: `create_vault(dilithium_public_key)`
    - Fee: **10 TSRX** (Spam prevention burn).
    - Effect: The account is locked. Standard `transfer` calls (ECDSA signed) are **permanently blocked**.

2.  **Withdrawal/Transfer**:
    - Extrinsic: `vault_transfer(dilithium_signature, to, amount)`
    - Fee: **100x Base Fee**.
    - Validation: Requires a valid Dilithium signature matching the stored PQC key. ECDSA signatures are ignored.

3.  **Replay Protection**:
    - Dedicated `VaultNonces` storage ensures PQC signatures cannot be replayed.

### 5.3 Security Model
This creates a hybrid security model. Day-to-day interactions can effectively use fast ECDSA (EVM), while long-term "Whale" holdings remain securely locked behind quantum-resistant walls. An attacker breaking ECC cannot steal funds from a Quantum Vault.

## 6. Architecture & Pallets

The runtime is composed of the following custom and standard modules:

| Pallet | Function |
| :--- | :--- |
| **System** | Core low-level types and storage. |
| **Balances** | Token logic ($TSRX). |
| **Aura/Grandpa** | Consensus mechanism. |
| **EVM/Ethereum** | Solidity smart contract execution. |
| **Emission** | V2.0 Stateless Sigmoid Emission distribution. |
| **QuantumVault** | PQC Cold Storage implementation. |

## 7. Roadmap

- **Phase 1: Foundation (Completed)**
    - Chain specification & Genesis.
    - Sigmoid Emission logic (`pallet-emission`).

- **Phase 2: EVM Integration (Completed)**
    - Frontier integration.
    - RPC Layer compatibility.

- **Phase 3: Quantum Defense (Completed)**
    - `pallet-quantum-vault` implementation.
    - Dilithium integration.

- **Phase 4: Testnet Hardening (Current)**
    - Unit testing & Benchmarking.
    - Transfer blocking hooks.
    - Public testnet launch.

- **Phase 5: Mainnet**
    - Security Audit.
    - Genesis Block Ceremony.

---

*"Numbers are the universal language, and Tesserax is the currency spoken in it."*

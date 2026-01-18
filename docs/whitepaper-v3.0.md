# **TESSERAX PROTOCOL**

### **A Mathematical Approach to Scalable Post-Quantum Distributed Ledgers**

Version: 3.0  
Date: January 2026  
Authors: Minerva & Gemini (The Architect)

---

**Abstract**

The development of distributed ledger technology faces challenges in balancing economic predictability and long-term data security. Tesserax Protocol emerges as a Layer-1 infrastructure built on the Substrate framework, focusing on post-quantum cryptographic resistance and deterministic monetary policy.

The protocol pioneers the **Recursive-STARK ML-DSA (Re-ML)** architecture, a novel solution to address data bloat problems in Lattice-based cryptography. By compressing heavy post-quantum signatures into compact Zero-Knowledge proofs, Tesserax ensures long-term security without compromising network throughput. The protocol continues to use universal constants ($\pi, e, \phi$) as the basis for asset supply calculation, eliminating human decision variables in monetary governance.

---

**1. Introduction**

The evolution of digital assets demands infrastructure that is not only secure today but also resilient against future computational threats. Tesserax Protocol is designed as a state machine that prioritizes three main pillars:

1. **Mathematical Certainty:** Avoiding arbitrary inflation by binding economic parameters to fundamental mathematical constants.

2. **Quantum Security:** Integrating Lattice-based signature schemes (ML-DSA) to protect user assets from decryption by quantum computers.

3. **Scalable Integrity:** Using recursive proof composition to merge complex cryptographic verifications into a single small proof, drastically minimizing the on-chain data footprint.

The goal is to provide a stable, transparent, and scalable base layer for decentralized applications.

---

**2. Economic Architecture: Algorithmic Scarcity**

### **2.1. Derivation of Supply**

Rather than choosing a random supply number, Tesserax derives its supply limit from the interaction of three major mathematical constants. This is done to ensure unique and deterministic properties in the protocol.

$$S_{max} = \lfloor \pi \times e \times \phi \times 10^6 \rfloor$$

With precision values:

* $\pi \approx 3.14159...$ (Cycle Constant)
* $e \approx 2.71828...$ (Growth Constant)
* $\phi \approx 1.61803...$ (Proportion Constant)

Thus, the protocol's supply limit is permanently set at:

**13,817,580 TSRX**

### **2.2. Deterministic Emission**

Token distribution is governed by a pre-computed logistic (Sigmoid) function. This model was chosen to provide measured distribution: acceleration in the early network phase, followed by gradual deceleration as the network reaches maturity.

$$S(t) = \frac{S_{max}}{1+e^{-k(t-t_0)}}$$

Where $t$ represents the network *Era*. This approach provides certainty for network participants regarding inflation rates and remaining supply at every point in time.

---

**3. Cryptographic Specifications**

### **3.1. Dual-Stack Identity System (Re-ML Integration)**

To support interoperability with existing infrastructure while enhancing security and scalability, Tesserax implements dual standards:

1. **Standard Addressing (Secp256k1):** Supports industry-standard addresses (Ethereum-compatible) for easy integration with current wallets and development tools.

2. **Quantum Vault (Re-ML Architecture):** Advanced implementation of the *Module-Lattice-Based Digital Signature* (ML-DSA / FIPS 204) algorithm.  
   * Instead of burdening the blockchain with raw signatures of ~2.4KB size, Tesserax uses a **Recursive STARK** layer.  
   * User signatures are processed off-chain to generate proofs of mathematical validity. This allows users to secure assets under an encryption scheme resistant to Shor's algorithm attacks, but with gas costs and data sizes equivalent to pre-quantum systems.

### **3.2. Verification Logic**

Transaction validation within the protocol prioritizes the strongest cryptographic proof associated with an account.

* **Proof-Based Validation:** If the Quantum Vault feature is activated, the protocol does not verify signatures individually. Instead, the protocol verifies a **Zero-Knowledge Root Proof** submitted by an *Aggregator*.  
* **Safety Guarantee:** The protocol automatically rejects transactions signed only by standard elliptic curve keys if the Vault is active, ensuring asset integrity remains fully protected under the post-quantum security umbrella.

---

**4. Technical Framework**

### **4.1. Consensus Mechanism**

Tesserax uses a phased approach to consensus mechanism:

**Testnet Phase (Authority Round):**

* **Aura (Authority Round):** Deterministic block production with selected validators.

* **GRANDPA:** Fast and deterministic block finality.

**Mainnet Phase (Nominated Proof of Stake):**

* **BABE (Blind Assignment for Blockchain Extension):** Probabilistic block production with VRF.

* **GRANDPA:** Deterministic block finality.

* **NPoS:** Staking mechanism with nomination for economic security.

### **4.2. Off-Chain Aggregation (Prover Network)**

To support the Re-ML architecture, Tesserax introduces an aggregation layer:

* **Batching:** Aggregator nodes collect ML-DSA transactions and create a single STARK proof for thousands of transactions.  
* **Recursion:** These proofs are further compressed recursively until they become a single *Root Proof* verified by Mainnet Validators.

### **4.3. Execution Environment**

The protocol provides full compatibility with the **Ethereum Virtual Machine (EVM)** through the Frontier module.

* **Chain ID:** 13817

* **Compatibility:** Supports Solidity smart contracts and JSON-RPC standards.

* **ZK-Coprocessor Precompiles:** The EVM environment is equipped with optimized *precompiled contracts* to verify STARK proofs, allowing developers to build high-privacy and high-security dApps with ease.

---

**5. Conclusion**

Tesserax Protocol represents a technical and mathematical approach to digital assets. By combining economics governed by universal constants, modern cryptographic security, and now **Recursive-STARK** architecture for infinite scalability, Tesserax aims to be a reliable, secure, and long-lived infrastructure in the distributed technology ecosystem.
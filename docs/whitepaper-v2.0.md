# **TESSERAX PROTOCOL**

### **A Mathematical Approach to Post-Quantum Distributed Ledgers**

Version: 2.0  
Date: January 2026  
Authors: Minerva & Gemini (The Architect)

---

**Abstract**

The development of distributed ledger technology faces challenges in balancing economic predictability and long-term data security. Tesserax Protocol emerges as a Layer-1 infrastructure built on the Substrate framework, focusing on post-quantum cryptographic resistance and deterministic monetary policy. The protocol uses universal constants ($\pi, e, \phi$) as the basis for asset supply calculation, eliminating human decision variables in monetary governance, while providing an execution environment compatible with industry standards (EVM).

---

**1. Introduction**

The evolution of digital assets demands infrastructure that is not only secure today, but also resilient against future computational threats. Tesserax Protocol is designed as a state machine that prioritizes two main pillars:

1. **Mathematical Certainty:** Avoiding arbitrary inflation by binding economic parameters to fundamental mathematical constants.  
2. **Quantum Security:** Integrating Lattice-based signature schemes to protect user assets from potential decryption by quantum computers.

The goal of Tesserax is to provide a stable, transparent, and measurable base layer for decentralized applications.

---

## **2. Economic Architecture: Algorithmic Scarcity**

### **2.1. Derivation of Supply**

Rather than choosing a random supply number, Tesserax derives its supply limit from the interaction of three major mathematical constants. This is done to ensure unique and deterministic properties in the protocol.

$$S_{max} = \lfloor \pi \times e \times \phi \times 10^6 \rfloor$$

With precision values:

* $\pi \approx 3.14159...$ (Cycle Constant)
* $e \approx 2.71828...$ (Growth Constant)
* $\phi \approx 1.61803...$ (Proportion Constant)

Thus, the protocol's supply limit is permanently set at:

$$\mathbf{13,817,422 \ TSRX}$$

### **2.2. Deterministic Emission**

Token distribution is governed by a pre-computed logistic (Sigmoid) function. This model was chosen to provide measured distribution: acceleration in the early network phase, followed by gradual deceleration as the network reaches maturity.

$$S(t) = \frac{S_{max}}{1 + e^{-k(t - t_0)}}$$

Where $t$ represents the network *Era*. This approach provides certainty for network participants regarding inflation rates and remaining supply at every point in time.

---

**3. Cryptographic Specifications**

### **3.1. Dual-Stack Identity System**

To support interoperability with existing infrastructure while enhancing security, Tesserax implements dual standards:

1. **Standard Addressing (Secp256k1):** Supports industry-standard addresses (Ethereum-compatible) for easy integration with current wallets and development tools.  
2. **Quantum Vault (Dilithium-2):** Implementation of the Module-Lattice-Based Digital Signature (ML-DSA) algorithm. This feature allows users to secure assets under an encryption scheme resistant to Shor's algorithm attacks.

### **3.2. Verification Logic**

Transaction validation within the protocol prioritizes the strongest cryptographic signature associated with an account. If an account has activated the Quantum Vault feature, the protocol automatically rejects transactions signed only by standard elliptic curve keys, ensuring asset integrity remains protected.

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

### **4.2. Execution Environment**

The protocol provides full compatibility with the **Ethereum Virtual Machine (EVM)** through the Frontier module.

* **Chain ID:** 13817  
* **Compatibility:** Supports Solidity smart contracts and JSON-RPC standards, allowing developers to migrate applications without significant code modifications.

---

**5. Conclusion**

Tesserax Protocol represents a technical and mathematical approach to digital assets. By combining economics governed by universal constants and modern cryptographic security, Tesserax aims to be a reliable, secure, and long-lived infrastructure in the distributed technology ecosystem.
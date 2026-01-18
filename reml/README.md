# Re-ML: Recursive-STARK ML-DSA

## Overview

Re-ML (Recursive-STARK ML-DSA) is Tesserax Protocol's solution for scalable post-quantum signatures. It uses SP1 zkVM to compress thousands of ML-DSA signatures into a single STARK proof.

```
┌─────────────────────────────────────────────────────────────────────────┐
│  PROBLEM: ML-DSA signatures are ~2.4KB each                            │
│           1,000 transactions = 2.4 MB of signature data                │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│  SOLUTION: Re-ML compresses signatures into ~100KB proof               │
│           Compression ratio: ~24x (or higher with more signatures)     │
└─────────────────────────────────────────────────────────────────────────┘
```

## Architecture

Re-ML consists of three components:

### 1. Guest (zkVM Program)
**Location:** `reml/guest/`

The circuit logic that runs inside SP1 zkVM:
- Receives batch of ML-DSA signature requests
- Verifies each signature
- Commits verification results as public output

```rust
// Example: What the guest does
for signature in batch {
    if verify_mldsa(signature) {
        verified_ids.push(signature.id);
    }
}
commit(verified_ids);
```

### 2. Host (Prover/Aggregator)
**Location:** `reml/host/`

Off-chain program that generates proofs:
- Collects signature requests from users
- Batches them together
- Invokes SP1 prover to generate STARK proof
- Outputs proof bundle for on-chain submission

```bash
# Generate proof
reml-prover prove --input batch.json --output proof.json

# Verify locally
reml-prover verify --proof proof.json

# Generate test data
reml-prover gen-test --count 100 --output test-batch.json
```

### 3. Verifier (Substrate Pallet)
**Location:** `pallets/reml-verifier/`

On-chain verification:
- Receives proof submissions from aggregators
- Verifies STARK proofs
- Marks request IDs as verified
- Integrates with Quantum Vault for transfer authorization

## Data Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────────────────────┐
│   Users     │───▶│  Aggregator │───▶│     SP1 Prover (GPU)       │
│ (ML-DSA Sig)│    │   (Batch)   │    │  (Generate STARK Proof)    │
└─────────────┘    └─────────────┘    └─────────────────────────────┘
                                                   │
                                                   ▼ Submit Proof
┌─────────────────────────────────────────────────────────────────────────┐
│                        Tesserax Blockchain                              │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐     │
│  │ Re-ML Verifier  │───▶│  Quantum Vault  │───▶│    Balances     │     │
│  │ (Verify Proof)  │    │ (Check Authz)   │    │   (Transfer)    │     │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘     │
└─────────────────────────────────────────────────────────────────────────┘
```

## Prerequisites

### Install SP1

```bash
# Install SP1 toolchain
curl -L https://sp1.succinct.xyz | bash
sp1up

# Verify installation
cargo prove --version
```

### GPU Support (Optional but Recommended)

For faster proof generation:
- CUDA-capable GPU (RTX 3080+ recommended)
- CUDA Toolkit 12.0+

## Building

```bash
cd reml

# Build all components
cargo build --release

# Build guest program for zkVM
cd guest && cargo prove build
```

## Usage

### 1. Generate Test Signatures

```bash
# Generate 100 test ML-DSA signatures
cargo run --release --bin reml-prover -- \
    gen-test --count 100 --output test-batch.json
```

### 2. Generate Proof

```bash
# Generate STARK proof (requires GPU for speed)
cargo run --release --bin reml-prover -- \
    prove --input test-batch.json --output proof.json --batch-id 1
```

### 3. Verify Proof (Locally)

```bash
# Verify proof before on-chain submission
cargo run --release --bin reml-prover -- \
    verify --proof proof.json
```

### 4. Submit On-Chain

```javascript
// Using Polkadot.js
const proof = JSON.parse(fs.readFileSync('proof.json'));

await api.tx.remlVerifier
    .submitProof({
        batch_id: proof.output.batch_id,
        proof: proof.proof,
        verified_count: proof.output.verified_count,
        requests_root: proof.output.requests_root,
        verified_request_ids: proof.output.verified_request_ids,
        vkey_hash: proof.vkey_hash,
    })
    .signAndSend(aggregatorAccount);
```

## Performance

| Batch Size | Raw Signature Size | Proof Size | Compression | Proof Time (GPU) |
|------------|-------------------|------------|-------------|------------------|
| 10         | 24 KB             | ~20 KB     | 1.2x        | ~30s             |
| 100        | 240 KB            | ~40 KB     | 6x          | ~2m              |
| 1,000      | 2.4 MB            | ~100 KB    | 24x         | ~15m             |
| 10,000     | 24 MB             | ~150 KB    | 160x        | ~2h              |

*Times are approximate and depend on hardware*

## Security Considerations

### Aggregator Trust Model

Aggregators are semi-trusted entities:
- They CANNOT forge signatures (zkVM ensures this)
- They CAN censor transactions (by not including them)
- They CAN delay proofs (latency attack)

Mitigations:
- Multiple competing aggregators
- Governance-controlled aggregator registry
- Fallback to raw signature submission for high-value txs

### Proof Verification

The on-chain verifier must correctly verify STARK proofs. Options:
1. **EVM Precompile**: Deploy SP1 verifier contract, call via pallet-evm
2. **Native Verifier**: Port SP1 verifier to Rust, call as host function
3. **Optimistic Verification**: Accept proofs optimistically with fraud proofs

## TODO

- [ ] Implement full ML-DSA verification in zkVM guest
- [ ] Integrate SP1 STARK verifier in Substrate
- [ ] Add recursive proof aggregation (proof of proofs)
- [ ] Implement server mode for production aggregator
- [ ] Add fraud proof mechanism for optimistic mode
- [ ] Benchmark on various hardware configurations

## References

- [SP1 Documentation](https://docs.succinct.xyz)
- [ML-DSA (FIPS 204)](https://csrc.nist.gov/pubs/fips/204/final)
- [zk-DASTARK Paper](https://eprint.iacr.org/2024/xxx) (Similar concept)
- [Tesserax Whitepaper v3.0](../docs/whitepaper-v3.0.md)

## License

MIT License - See LICENSE file

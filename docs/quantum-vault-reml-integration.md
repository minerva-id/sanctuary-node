# Quantum Vault ↔ Re-ML Integration

## Overview

Quantum Vault is now integrated with the Re-ML verification system, enabling vault transfers to be verified using STARK proofs.

## Features

### 1. Optional Re-ML Verification

Vault transfers can now use Re-ML verification as an additional security layer:

```rust
// Transfer with Re-ML verification
QuantumVault::vault_transfer(
    origin,
    signature,
    to,
    amount,
    Some(request_id), // Re-ML request ID that has been verified
)?;

// Transfer without Re-ML (backward compatible)
QuantumVault::vault_transfer(
    origin,
    signature,
    to,
    amount,
    None,
)?;
```

### 2. EVM Integration via Precompiles

Smart contracts can leverage ZK-Coprocessor precompiles for verification:

```solidity
contract QuantumVaultEVM {
    function executeTransfer(uint64 requestId) external {
        // Verify Re-ML proof via precompile (0x21)
        require(ReMLVerifierLib.isRequestVerified(requestId), "Not verified");
        
        // Execute transfer logic...
    }
}
```

See `contracts/ReMLVerifier.sol` for complete examples.

### 3. Events

The integration adds new events:

-   **`VaultTransferVerified`**: Emitted when a transfer is verified via Re-ML
    ```rust
    VaultTransferVerified {
        from: AccountId,
        request_id: u64,
    }
    ```

-   **`VaultTransfer`** (updated): Now includes `request_id` field
    ```rust
    VaultTransfer {
        from: AccountId,
        to: AccountID,
        amount: Balance,
        nonce: u64,
        premium_fee: Balance,
        request_id: Option<u64>, // ✨ NEW
    }
    ```

### 4. Error Handling

New errors:

-   `RequestNotVerified`: Request ID not found in Re-ML verifier or not yet verified

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Quantum Vault Transfer                   │
│                                                             │
│  1. Dilithium Signature ────────┐                          │
│  2. Re-ML Request ID (Optional) │                          │
│                                 ▼                           │
│           ┌─────────────────────────────┐                  │
│           │  Quantum Vault Pallet       │                  │
│           │  ┌──────────────────────┐   │                  │
│           │  │ Verify Dilithium Sig │   │                  │
│           │  └──────────┬───────────┘   │                  │
│           │             │               │                  │
│           │             ▼               │                  │
│           │  ┌──────────────────────┐   │                  │
│           │  │ Check Re-ML (if ID)  │────────┐             │
│           │  └──────────────────────┘   │    │             │
│           │             │               │    │             │
│           │             ▼               │    ▼             │
│           │  ┌──────────────────────┐   │  ┌─────────────┐ │
│           │  │  Execute Transfer    │   │  │   Re-ML     │ │
│           │  └──────────────────────┘   │  │  Verifier   │ │
│           └─────────────────────────────┘  └─────────────┘ │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Use Cases

### 1. High-Value Transfers

For high-value transfers, applications can enforce Re-ML usage:

```rust
// Frontend logic
if transfer_amount > HIGH_VALUE_THRESHOLD {
    // Generate ML-DSA signature off-chain
    let signature = generate_signature(&message);
    
    // Submit to Re-ML aggregator
    let request_id = submit_to_aggregator(signature);
    
    // Wait for verification (polling or event subscription)
    wait_for_verification(request_id);
    
    // Execute transfer with Re-ML proof
    vault_transfer(origin, signature, to, amount, Some(request_id));
} else {
    // Standard vault transfer
    vault_transfer(origin, signature, to, amount, None);
}
```

### 2. Smart Contract Integration

EVM contracts can enforce quantum-safe transfers:

```solidity
contract SecureVault {
    using ReMLVerifierLib for uint64;
    
    mapping(address => uint256) public balances;
    
    function withdraw(
        uint256 amount,
        uint64 requestId
    ) external {
        // Enforce Re-ML verification for all withdrawals
        require(requestId.isRequestVerified(), "Quantum proof required");
        
        // Execute withdrawal
        balances[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
    }
}
```

### 3. Gradual Adoption

Re-ML verification is optional, allowing gradual adoption:

-   **Phase 1**: Vault creation and standard transfers (existing feature)
-   **Phase 2**: Optional Re-ML for sensitive transfers
-   **Phase 3**: Mandatory Re-ML for transfers > threshold (governance decision)

## Testing

Tests verify backward compatibility:

```bash
cargo test -p pallet-quantum-vault
```

All existing tests pass without significant modifications. New tests can be added for:

-   Re-ML verification success/failure scenarios
-   Event emission verification
-   Error handling edge cases

## Performance

Re-ML verification adds minimal overhead:

-   **Storage Read**: 1 additional (check `VerifiedRequestIds`)
-   **Gas Cost** (EVM): ~10,000 gas for precompile call (0x21)
-   **Latency**: Synchronous (verification already performed off-chain)

## Security Considerations

1.  **Request ID Cannot Be Forged**: Re-ML verifier ensures only valid requests are marked as verified
2.  **Replay Protection**: Inherent from nonce-based Dilithium signatures
3.  **Optional Nature**: Does not force all users to use Re-ML, avoiding UX friction
4.  **Precompile Safety**: EVM precompiles only read storage, cannot modify state

## Next Steps

1.  ✅ Quantum Vault ↔ Re-ML integration (DONE)
2.  ⏳ Runtime configuration update
3.  ⏳ Frontend integration guide
4.  ⏳ Production audit preparation

## Related Documents

-   [Re-ML Architecture](Re-ML.md)
-   [ZK-Coprocessor Precompiles](../runtime/src/precompiles.rs)
-   [Solidity Interface](../contracts/ReMLVerifier.sol)
-   [Quantum Vault Specification](../pallets/quantum-vault/README.md)

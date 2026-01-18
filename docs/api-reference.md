# 📖 Tesserax Protocol - API Reference

**Version:** 2.0.0  
**Last Updated:** January 3, 2026

---

## Table of Contents

- [Overview](#overview)
- [Connecting to the Network](#connecting-to-the-network)
- [Pallets](#pallets)
  - [Emission Pallet](#emission-pallet)
  - [Quantum Vault Pallet](#quantum-vault-pallet)
  - [Balances Pallet](#balances-pallet)
- [EVM RPC Methods](#evm-rpc-methods)
- [Runtime Metadata](#runtime-metadata)
- [Constants](#constants)

---

## Overview

Tesserax Protocol exposes three types of APIs:

1. **Substrate RPC** - Standard Polkadot SDK JSON-RPC
2. **Runtime Storage** - On-chain state queries
3. **EVM RPC** - Ethereum-compatible JSON-RPC

---

## Connecting to the Network

### WebSocket (Substrate)

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

const wsProvider = new WsProvider('wss://testnet.tesserax.network/ws');
const api = await ApiPromise.create({ provider: wsProvider });
```

### HTTP (EVM)

```javascript
const { ethers } = require('ethers');

const provider = new ethers.JsonRpcProvider('https://testnet.tesserax.network/eth');
```

---

## Pallets

### Emission Pallet

The emission pallet manages the sigmoid emission schedule for TSRX token distribution.

#### Constants

| Constant | Type | Value | Description |
|----------|------|-------|-------------|
| `MAX_SUPPLY` | `u128` | 13,817,580 × 10^18 | Maximum token supply in planck |
| `TOTAL_ERAS` | `usize` | 7,300 | Total eras in emission schedule |
| `BLOCKS_PER_ERA` | `u32` | 14,400 | Blocks per era (~24 hours) |

#### Storage

*No on-chain storage - emission is stateless*

#### Events

```rust
/// Block reward minted to author
RewardMinted {
    block_number: BlockNumber,
    era: u32,
    author: AccountId,
    reward: Balance,
}

/// Emission schedule completed
EmissionEnded {
    block_number: BlockNumber,
    total_eras: u32,
}
```

#### Runtime APIs

```rust
/// Get current era for block number
fn current_era(block_number: BlockNumber) -> u32

/// Get reward for specific era
fn reward_for_era(era: u32) -> u128

/// Get maximum supply
fn max_supply() -> u128

/// Get total eras in schedule
fn total_eras() -> u32

/// Check if emission has ended
fn is_emission_ended(block_number: BlockNumber) -> bool

/// Get total emitted tokens to date
fn total_emitted(block_number: BlockNumber) -> u128
```

#### JavaScript Examples

```javascript
// Query emission constants
const maxSupply = api.consts.emission.maxSupply;
const totalEras = api.consts.emission.totalEras;
const blocksPerEra = api.consts.emission.blocksPerEra;

console.log(`Max Supply: ${maxSupply.toHuman()}`);
console.log(`Total Eras: ${totalEras.toNumber()}`);
console.log(`Blocks Per Era: ${blocksPerEra.toNumber()}`);

// Calculate current era
const blockNumber = (await api.rpc.chain.getHeader()).number.toNumber();
const currentEra = Math.floor((blockNumber - 1) / 14400);
console.log(`Current Era: ${currentEra}`);
```

---

### Quantum Vault Pallet

The quantum vault pallet provides post-quantum cryptographic protection using CRYSTALS-Dilithium.

#### Configuration Constants

| Constant | Type | Value | Description |
|----------|------|-------|-------------|
| `VaultCreationFee` | `Balance` | 10 TSRX | Fee to create a vault (burned) |
| `VaultTransferFeeMultiplier` | `u32` | 100 | Fee multiplier for vault transfers |
| `MaxPublicKeySize` | `u32` | 1,312 | Dilithium2 public key size |
| `MaxSignatureSize` | `u32` | 2,420 | Dilithium2 signature size |

#### Storage

```rust
/// Map of account -> VaultInfo
Vaults: StorageMap<AccountId, VaultInfo>

/// Map of account -> nonce (for replay protection)
Nonces: StorageMap<AccountId, u64>

/// Total number of vaults created
TotalVaults: StorageValue<u32>
```

##### VaultInfo Structure

```rust
pub struct VaultInfo {
    /// Dilithium2 public key (1312 bytes)
    pub public_key: BoundedVec<u8, MaxPublicKeySize>,
    /// Block number when vault was created
    pub created_at: BlockNumber,
}
```

#### Extrinsics

##### `create_vault(public_key)`

Creates a quantum vault for the calling account.

| Parameter | Type | Description |
|-----------|------|-------------|
| `public_key` | `Vec<u8>` | Dilithium2 public key (1312 bytes) |

**Requirements:**
- Account must not already be a vault
- Must have sufficient balance for creation fee (10 TSRX)
- Public key must be exactly 1312 bytes

**Events:**
```rust
VaultCreated { account: AccountId, public_key_hash: H256 }
```

**Example:**
```javascript
const publicKey = '0x' + '00'.repeat(1312); // Your Dilithium key

const tx = api.tx.quantumVault.createVault(publicKey);
await tx.signAndSend(account, ({ status }) => {
    console.log(`Status: ${status.type}`);
});
```

---

##### `vault_transfer(signature, to, amount)`

Transfers funds from a vault using PQC signature.

| Parameter | Type | Description |
|-----------|------|-------------|
| `signature` | `Vec<u8>` | Dilithium2 signature (2420 bytes) |
| `to` | `AccountId` | Recipient address |
| `amount` | `Compact<Balance>` | Amount to transfer |

**Signature Message Format:**
```
TESSERAX_VAULT_TRANSFER:<sender><recipient><amount><nonce>
```

**Requirements:**
- Account must be an active vault
- Signature must be valid against stored public key
- Sufficient balance for transfer + fee

**Events:**
```rust
VaultTransfer { from: AccountId, to: AccountId, amount: Balance, nonce: u64 }
```

**Example:**
```javascript
// Create signature offline
const message = createTransferMessage(sender, recipient, amount, nonce);
const signature = dilithiumSign(privateKey, message);

const tx = api.tx.quantumVault.vaultTransfer(signature, recipient, amount);
await tx.signAndSend(sender);
```

---

##### `destroy_vault(signature)`

Destroys a vault, returning account to normal operation.

| Parameter | Type | Description |
|-----------|------|-------------|
| `signature` | `Vec<u8>` | Dilithium2 signature (2420 bytes) |

**Signature Message Format:**
```
TESSERAX_VAULT_DESTROY:<account><nonce>
```

**Events:**
```rust
VaultDestroyed { account: AccountId }
```

---

#### Helper Functions

```rust
/// Check if account is a vault
fn is_vault(account: &AccountId) -> bool

/// Check if account can perform standard transfers
fn can_transfer(account: &AccountId) -> bool
```

**JavaScript Example:**
```javascript
// Check if account is a vault
const vaultInfo = await api.query.quantumVault.vaults(account);
const isVault = vaultInfo.isSome;

// Get vault nonce
const nonce = await api.query.quantumVault.nonces(account);
console.log(`Vault Nonce: ${nonce.toNumber()}`);

// Get total vaults
const totalVaults = await api.query.quantumVault.totalVaults();
console.log(`Total Vaults: ${totalVaults.toNumber()}`);
```

---

### Balances Pallet

Standard Substrate balances pallet for token management.

#### Key Extrinsics

| Extrinsic | Description |
|-----------|-------------|
| `transfer_allow_death(dest, amount)` | Transfer, may kill sender account |
| `transfer_keep_alive(dest, amount)` | Transfer, keeps sender alive |
| `force_transfer(source, dest, amount)` | Sudo transfer between accounts |

**Note:** Standard transfers are blocked for Quantum Vault accounts.

#### Storage

```rust
/// Account balance information
Account: StorageMap<AccountId, AccountData>

/// Total token issuance
TotalIssuance: StorageValue<Balance>
```

---

## EVM RPC Methods

Tesserax supports standard Ethereum JSON-RPC methods:

### Supported Methods

| Method | Description |
|--------|-------------|
| `eth_chainId` | Returns chain ID (13817) |
| `eth_blockNumber` | Current block number |
| `eth_getBalance` | Account balance |
| `eth_sendRawTransaction` | Submit signed transaction |
| `eth_call` | Call contract method |
| `eth_estimateGas` | Estimate gas for transaction |
| `eth_getTransactionReceipt` | Get transaction receipt |
| `eth_getTransactionByHash` | Get transaction details |
| `eth_getLogs` | Get event logs |
| `eth_getCode` | Get contract bytecode |
| `eth_getStorageAt` | Get contract storage |
| `net_version` | Network version |
| `web3_clientVersion` | Client version |

### Example Requests

```javascript
// Using fetch
const response = await fetch('https://testnet.tesserax.network/eth', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
        jsonrpc: '2.0',
        method: 'eth_chainId',
        params: [],
        id: 1
    })
});
const { result } = await response.json();
console.log(`Chain ID: ${parseInt(result, 16)}`); // 13817

// Using ethers.js
const provider = new ethers.JsonRpcProvider('https://testnet.tesserax.network/eth');
const blockNumber = await provider.getBlockNumber();
const balance = await provider.getBalance('0xYourAddress');
```

---

## Runtime Metadata

Query runtime metadata for complete API information:

```javascript
const metadata = await api.rpc.state.getMetadata();
console.log(JSON.stringify(metadata.toHuman(), null, 2));
```

---

## Constants

### Tesserax Constants

```javascript
// Query all Tesserax constants
const constants = {
    // Token
    tokenDecimals: 18,
    tokenSymbol: 'TSRX',
    tokenName: 'Tesserax',
    
    // Supply (from tesserax_constants module)
    maxSupplyUnits: 13817422,
    maxSupply: '13817422000000000000000000', // planck
    
    // Mathematical constants (× 10^9)
    pi: 3141592653,
    e: 2718281828,
    phi: 1618033988,
    
    // EVM
    chainId: 13817,
    
    // Time
    blockTime: 6000, // ms
    blocksPerMinute: 10,
    blocksPerHour: 600,
    blocksPerDay: 14400,
    
    // Emission
    totalEras: 7300,
    blocksPerEra: 14400,
    
    // Quantum Vault
    vaultCreationFee: '10000000000000000000', // 10 TSRX
    vaultTransferFeeMultiplier: 100,
    dilithiumPublicKeySize: 1312,
    dilithiumSignatureSize: 2420,
    
    // Other
    existentialDeposit: '1000000000000000000' // 1 TSRX
};
```

---

## Error Codes

### Quantum Vault Errors

| Error | Code | Description |
|-------|------|-------------|
| `AlreadyVault` | 0 | Account is already a vault |
| `NotVault` | 1 | Account is not a vault |
| `InvalidPublicKeySize` | 2 | Public key must be 1312 bytes |
| `InvalidSignatureSize` | 3 | Signature must be 2420 bytes |
| `InvalidSignature` | 4 | Signature verification failed |
| `InsufficientBalance` | 5 | Not enough balance |

### Transaction Extension Errors

| Error | Code | Description |
|-------|------|-------------|
| `VaultTransferBlocked` | 100 | Standard transfer blocked for vault |

---

## SDK Examples

### Python (py-substrate-interface)

```python
from substrateinterface import SubstrateInterface

substrate = SubstrateInterface(
    url="wss://testnet.tesserax.network/ws"
)

# Get chain info
chain = substrate.rpc_request("system_chain", [])
print(f"Chain: {chain['result']}")

# Check if account is vault
result = substrate.query(
    module='QuantumVault',
    storage_function='Vaults',
    params=['5YourAccountId...']
)
print(f"Is Vault: {result.value is not None}")
```

### Rust (subxt)

```rust
use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url(
        "wss://testnet.tesserax.network/ws"
    ).await?;
    
    let block = api.blocks().at_latest().await?;
    println!("Block: {:?}", block.number());
    
    Ok(())
}
```

---

*For more examples, visit the [GitHub repository](https://github.com/tesserax/tesserax-node)*

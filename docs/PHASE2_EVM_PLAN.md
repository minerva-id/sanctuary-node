# Phase 2: EVM Layer Implementation Plan

## Current Status
- **Sanctuary Node**: Using Polkadot SDK with frame-support v40.1.0 (approx stable2412)
- **Frontier Master**: Uses stable2506 (newer, incompatible)

## Challenge
Frontier pallets are not published to crates.io with matching versions.
We need to use git dependencies with a compatible Frontier revision.

## Solution Options

### Option A: Upgrade Polkadot SDK to stable2506 (Recommended)
1. Update all workspace dependencies to stable2506
2. Use Frontier master branch
3. Full compatibility, latest features

### Option B: Find Compatible Frontier Revision
1. Check Frontier git history for commit compatible with stable2412
2. Use specific git rev for Frontier dependencies

## Implementation Steps (Option A Chosen)

### Step 1: Backup Current State
```bash
git checkout -b backup/pre-evm-upgrade
git push origin backup/pre-evm-upgrade
```

### Step 2: Update Polkadot SDK Dependencies
- Update Cargo.toml workspace dependencies to use stable2506
- This requires changing version numbers or switching to git deps

### Step 3: Add Frontier Dependencies
Required crates:
- pallet-evm
- pallet-ethereum  
- pallet-base-fee
- pallet-evm-chain-id
- fp-evm
- fp-rpc
- fp-self-contained
- fp-account

### Step 4: Configure Runtime
1. Add EVM config to runtime/src/configs/mod.rs
2. Configure precompiles
3. Set Chain ID (13817 - derived from Tesserax Constant)
4. Configure EIP-1559 base fee

### Step 5: Update Node for EVM RPC
1. Add fc-rpc dependencies
2. Configure Ethereum-compatible RPC endpoints
3. Add block mapping database

### Step 6: Testing
1. Start node in dev mode
2. Connect Metamask to localhost:9944
3. Deploy test contract via Remix

## Timeline
- Week 1: SDK Upgrade + Frontier dependencies
- Week 2: Runtime configuration + Testing

## References
- Frontier Template: https://github.com/polkadot-evm/frontier/tree/master/template
- Frontier Docs: https://polkadot-evm.github.io/frontier/

# 🚀 Tesserax Protocol - Public Testnet Guide

<p align="center">
  <img src="../assets/tesserax-banner.png" alt="Tesserax Protocol" width="600">
</p>

<p align="center">
  <strong>Where Mathematics Becomes Money</strong>
</p>

<p align="center">
  <a href="#quick-start">Quick Start</a> •
  <a href="#network-info">Network Info</a> •
  <a href="#wallets">Wallets</a> •
  <a href="#faucet">Faucet</a> •
  <a href="#developers">Developers</a>
</p>

---

## 📋 Table of Contents

- [Network Information](#-network-information)
- [Quick Start](#-quick-start)
- [Wallet Setup](#-wallet-setup)
- [Getting Testnet Tokens](#-getting-testnet-tokens)
- [Interacting with the Network](#-interacting-with-the-network)
- [EVM Integration](#-evm-integration)
- [Quantum Vault](#-quantum-vault)
- [For Validators](#-for-validators)
- [For Developers](#-for-developers)
- [Troubleshooting](#-troubleshooting)
- [Resources](#-resources)

---

## 🌐 Network Information

### Testnet Specifications

| Parameter | Value |
|-----------|-------|
| **Network Name** | Tesserax Testnet |
| **Native Token** | TSRX (test) |
| **Token Decimals** | 18 |
| **Block Time** | 6 seconds |
| **Consensus** | Aura + GRANDPA |
| **Chain ID (EVM)** | 13817 |
| **SS58 Format** | 42 (Generic Substrate) |

### RPC Endpoints

| Type | URL |
|------|-----|
| **WebSocket RPC** | `wss://testnet.tesserax.network/ws` |
| **HTTP RPC** | `https://testnet.tesserax.network/rpc` |
| **EVM RPC (ETH)** | `https://testnet.tesserax.network/eth` |

### Explorers

| Explorer | URL |
|----------|-----|
| **Polkadot.js Apps** | https://polkadot.js.org/apps/?rpc=wss://testnet.tesserax.network/ws |
| **Block Explorer** | https://explorer.tesserax.network |
| **EVM Explorer** | https://evm.tesserax.network |

---

## ⚡ Quick Start

### 1. Connect to Testnet

**Using Polkadot.js Apps:**
1. Go to https://polkadot.js.org/apps
2. Click on the network selector (top left)
3. Choose "Development" → "Custom Endpoint"
4. Enter: `wss://testnet.tesserax.network/ws`
5. Click "Switch"

**Or run a local node:**
```bash
# Using Docker
docker run -d --name tesserax-testnet \
  -p 30333:30333 \
  -p 9944:9944 \
  tesserax/node:latest \
  --chain testnet \
  --name "MyNode"

# Or build from source
git clone https://github.com/tesserax/tesserax-node.git
cd tesserax-node
cargo build --release
./target/release/tesserax-node --chain testnet
```

### 2. Create a Wallet

1. Go to https://polkadot.js.org/apps/#/accounts
2. Click "+ Account"
3. Save your mnemonic phrase securely
4. Set a password and create the account

### 3. Get Testnet Tokens

Visit the faucet: https://faucet.tesserax.network

Or use the Discord bot:
```
!faucet <your-address>
```

### 4. Start Exploring!

- View your balance in Polkadot.js Apps
- Make transfers to other accounts
- Create a Quantum Vault for enhanced security
- Deploy smart contracts via EVM

---

## 👛 Wallet Setup

### Supported Wallets

| Wallet | Substrate | EVM | Link |
|--------|-----------|-----|------|
| **Polkadot.js Extension** | ✅ | ❌ | [Install](https://polkadot.js.org/extension/) |
| **Talisman** | ✅ | ✅ | [Install](https://talisman.xyz/) |
| **SubWallet** | ✅ | ✅ | [Install](https://subwallet.app/) |
| **MetaMask** | ❌ | ✅ | [Install](https://metamask.io/) |

### MetaMask Configuration (EVM)

To connect MetaMask to Tesserax Testnet:

1. Open MetaMask → Settings → Networks → Add Network
2. Enter the following details:

| Field | Value |
|-------|-------|
| Network Name | Tesserax Testnet |
| New RPC URL | `https://testnet.tesserax.network/eth` |
| Chain ID | `13817` |
| Currency Symbol | `TSRX` |
| Block Explorer URL | `https://evm.tesserax.network` |

3. Click "Save"

---

## 💧 Getting Testnet Tokens

### Web Faucet

1. Visit https://faucet.tesserax.network
2. Enter your Tesserax address (starts with `5...`)
3. Complete the captcha
4. Click "Request Tokens"
5. Receive 10 TSRX (test tokens)

**Rate Limit:** 10 TSRX per address per 24 hours

### Discord Faucet

1. Join our Discord: https://discord.gg/tesserax
2. Go to the `#faucet` channel
3. Type: `!faucet 5YourAddressHere...`
4. Bot will send 10 TSRX to your address

### CLI Faucet (for developers)

```bash
curl -X POST https://faucet.tesserax.network/api/drip \
  -H "Content-Type: application/json" \
  -d '{"address": "5YourAddressHere..."}'
```

---

## 🔄 Interacting with the Network

### Making Transfers

**Via Polkadot.js Apps:**
1. Go to Accounts → Transfer
2. Select sender and recipient
3. Enter amount
4. Sign and submit

**Via Command Line:**
```bash
# Using subxt or polkadot-js CLI
polkadot-js-api --ws wss://testnet.tesserax.network/ws \
  tx.balances.transferKeepAlive \
  5RecipientAddress... \
  1000000000000000000 \
  --seed "your seed phrase"
```

### Viewing Emission Schedule

The testnet uses the same sigmoid emission curve as mainnet:

- **Max Supply:** 13,817,580 TSRX
- **Duration:** 20 years (7,300 eras)
- **Era Length:** 14,400 blocks (~24 hours)
- **Peak Rewards:** Era 3,650 (Year 10)

View current emission statistics:
1. Go to Chain State → Constants
2. Select `emission` pallet
3. View `maxSupply`, `totalEras`, etc.

---

## 💎 EVM Integration

### Deploying Smart Contracts

Tesserax is fully EVM-compatible. Deploy Solidity contracts using standard tools:

**Using Hardhat:**
```javascript
// hardhat.config.js
module.exports = {
  networks: {
    tesserax: {
      url: "https://testnet.tesserax.network/eth",
      chainId: 13817,
      accounts: [process.env.PRIVATE_KEY]
    }
  }
};
```

```bash
npx hardhat run scripts/deploy.js --network tesserax
```

**Using Remix:**
1. Open https://remix.ethereum.org
2. Connect MetaMask (configured for Tesserax)
3. Compile your contract
4. Deploy using "Injected Provider"

### Pre-deployed Contracts

| Contract | Address |
|----------|---------|
| WTSRX (Wrapped TSRX) | `0x...` |
| Multicall3 | `0x...` |

### Supported Precompiles

| Address | Precompile |
|---------|------------|
| `0x01` | ecRecover |
| `0x02` | SHA256 |
| `0x03` | RIPEMD160 |
| `0x04` | Identity |
| `0x05` | Modexp |

---

## 🔐 Quantum Vault

### What is Quantum Vault?

Quantum Vault provides **post-quantum cryptographic protection** for your TSRX holdings using CRYSTALS-Dilithium Level 2 signatures.

### Why Use It?

- **Future-Proof:** Resistant to quantum computer attacks
- **Cold Storage:** Enhanced security for long-term holdings
- **Controlled Access:** Standard transfers blocked, only PQC-signed transfers allowed

### Creating a Quantum Vault

**Via Polkadot.js Apps:**
1. Go to Developer → Extrinsics
2. Select `quantumVault` → `createVault`
3. Enter your Dilithium2 public key (1312 bytes, hex-encoded)
4. Submit transaction (costs 10 TSRX creation fee)

**Requirements:**
- 10 TSRX creation fee (burned)
- Dilithium2 public key (generate offline for security)

### Vault Transfer

Once your account is a vault, standard transfers are blocked. You must use `vault_transfer`:

```
quantumVault.vaultTransfer(signature, destination, amount)
```

**Signature Format:**
- Algorithm: CRYSTALS-Dilithium Level 2
- Message: `TESSERAX_VAULT_TRANSFER:` + sender + recipient + amount + nonce
- Size: 2,420 bytes

---

## 🏗️ For Validators

### Hardware Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 4 cores | 8+ cores |
| RAM | 8 GB | 16+ GB |
| Storage | 100 GB SSD | 500 GB NVMe |
| Network | 100 Mbps | 1 Gbps |

### Running a Validator Node

```bash
# Pull the Docker image
docker pull tesserax/node:latest

# Run as validator
docker run -d --name tesserax-validator \
  -p 30333:30333 \
  -p 9944:9944 \
  -p 9615:9615 \
  -v tesserax-data:/data \
  tesserax/node:latest \
  --chain testnet \
  --validator \
  --name "MyValidator" \
  --telemetry-url "wss://telemetry.tesserax.network/submit 0"
```

### Generating Session Keys

```bash
# Via RPC
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
  http://localhost:9944
```

### Registering as Validator

1. Generate session keys (see above)
2. Go to Polkadot.js Apps → Developer → Extrinsics
3. Submit `session.setKeys(keys, proof)`
4. Contact team for testnet validator whitelist

---

## 👩‍💻 For Developers

### Building from Source

```bash
# Clone repository
git clone https://github.com/tesserax/tesserax-node.git
cd tesserax-node

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup target add wasm32-unknown-unknown

# Build
cargo build --release

# Run tests
cargo test
```

### Local Development Chain

```bash
# Start dev chain (single node, instant finality)
./target/release/tesserax-node --dev

# With verbose logging
RUST_LOG=info ./target/release/tesserax-node --dev
```

### Using Polkadot.js API

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
  const wsProvider = new WsProvider('wss://testnet.tesserax.network/ws');
  const api = await ApiPromise.create({ provider: wsProvider });

  // Get chain info
  const chain = await api.rpc.system.chain();
  const lastHeader = await api.rpc.chain.getHeader();
  console.log(`${chain}: last block #${lastHeader.number}`);

  // Query emission pallet
  const maxSupply = api.consts.emission.maxSupply;
  console.log(`Max Supply: ${maxSupply.toHuman()}`);

  // Check if account is a vault
  const isVault = await api.query.quantumVault.vaults('5AccountId...');
  console.log(`Is Vault: ${isVault.isSome}`);
}

main().catch(console.error);
```

### API Documentation

- **Rust Docs:** https://docs.tesserax.network/rust
- **GitHub:** https://github.com/tesserax/tesserax-node
- **Whitepaper:** [docs/whitepaper-v2.0.md](./whitepaper-v2.0.md)
- **Blueprint:** [docs/blueprint.md](./blueprint.md)

---

## ❓ Troubleshooting

### Common Issues

#### "Insufficient Balance" Error
- Ensure you have enough TSRX for the transaction + fees
- Existential deposit: 1 TSRX minimum balance

#### "Vault Transfer Blocked" Error
- Your account is a Quantum Vault
- Use `quantumVault.vaultTransfer()` instead of regular transfer

#### Node Not Syncing
- Check your internet connection
- Ensure ports 30333 (P2P) and 9944 (RPC) are open
- Try: `--bootnodes /dns/bootnode.tesserax.network/tcp/30333/p2p/12D3...`

#### MetaMask Transaction Stuck
- Increase gas price slightly
- Check if you have sufficient TSRX balance

### Getting Help

- **Discord:** https://discord.gg/tesserax
- **Telegram:** https://t.me/tesserax
- **GitHub Issues:** https://github.com/tesserax/tesserax-node/issues

---

## 📚 Resources

### Documentation
- [Whitepaper v2.0](./whitepaper-v2.0.md)
- [Technical Blueprint](./blueprint.md)
- [Test Results](./test-results.md)
- [API Reference](./api-reference.md)

### Tools
- [Polkadot.js Apps](https://polkadot.js.org/apps)
- [Block Explorer](https://explorer.tesserax.network)
- [Faucet](https://faucet.tesserax.network)

### Community
- [Discord](https://discord.gg/tesserax)
- [Twitter](https://twitter.com/tesserax)
- [Telegram](https://t.me/tesserax)

---

## 📜 Testnet Terms

1. **No Real Value:** Testnet TSRX has no monetary value
2. **Subject to Resets:** Testnet may be reset without notice
3. **Use at Own Risk:** Testnet is for testing purposes only
4. **Bug Reports Welcome:** Report issues via GitHub

---

<p align="center">
  <strong>Happy Testing! 🎉</strong>
</p>

<p align="center">
  <em>The Tesserax Protocol Team</em>
</p>

---

*Last Updated: January 3, 2026*

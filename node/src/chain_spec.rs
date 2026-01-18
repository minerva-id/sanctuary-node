// This file is part of Tesserax Protocol.
//
// Copyright (C) 2025 Minerva & Gemini (The Architect)
// SPDX-License-Identifier: MIT-0

//! Chain specification for Tesserax Protocol.
//!
//! Defines the genesis state and network configuration for different environments:
//! - Development: Single-node for local development
//! - Local Testnet: Multi-node for integration testing
//! - (Future) Mainnet: Production network

use sc_service::ChainType;
use tesserax_runtime::WASM_BINARY;

/// Specialized `ChainSpec` for Tesserax Protocol.
pub type ChainSpec = sc_service::GenericChainSpec;

/// ═══════════════════════════════════════════════════════════════════════════
/// DEVELOPMENT CHAIN SPECIFICATION
/// ═══════════════════════════════════════════════════════════════════════════
///
/// Configuration for single-node development environment.
///
/// Features:
/// - Single validator (Alice)
/// - Instant block finality
/// - Pre-funded development accounts
/// - Sudo enabled for runtime upgrades
///
/// Usage: `tesserax-node --dev`
/// ═══════════════════════════════════════════════════════════════════════════
pub fn development_chain_spec() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("Tesserax Development")
    .with_id("tesserax_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    .with_protocol_id("tesserax")
    .with_properties(chain_properties())
    .build())
}

/// ═══════════════════════════════════════════════════════════════════════════
/// LOCAL TESTNET CHAIN SPECIFICATION  
/// ═══════════════════════════════════════════════════════════════════════════
///
/// Configuration for multi-node local testing.
///
/// Features:
/// - Two validators (Alice and Bob)
/// - GRANDPA finality (requires 2/3 consensus)
/// - All keyring accounts funded
/// - Suitable for integration testing
///
/// Usage:
/// - Node 1: `tesserax-node --chain local --alice`
/// - Node 2: `tesserax-node --chain local --bob`
/// ═══════════════════════════════════════════════════════════════════════════
pub fn local_chain_spec() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("Tesserax Local Testnet")
    .with_id("tesserax_local")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .with_protocol_id("tesserax")
    .with_properties(chain_properties())
    .build())
}

/// ═══════════════════════════════════════════════════════════════════════════
/// CHAIN PROPERTIES
/// ═══════════════════════════════════════════════════════════════════════════
///
/// Metadata properties exposed to wallets and block explorers.
///
/// These properties tell Polkadot.js and other clients:
/// - tokenSymbol: $TSRX
/// - tokenDecimals: 18 (EVM compatible)
/// ═══════════════════════════════════════════════════════════════════════════
fn chain_properties() -> sc_service::Properties {
    let mut properties = sc_service::Properties::new();
    properties.insert("tokenSymbol".into(), "TSRX".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), 42.into()); // Generic Substrate format
    properties
}

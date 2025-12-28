// This file is part of Sanctuary Protocol.
//
// Copyright (C) 2025 Minerva & Gemini (The Architect)
// SPDX-License-Identifier: MIT-0

//! Chain specification for Sanctuary Protocol.
//!
//! Defines the genesis state and network configuration for different environments:
//! - Development: Single-node for local development
//! - Local Testnet: Multi-node for integration testing
//! - (Future) Mainnet: Production network

use sc_service::ChainType;
use sanctuary_runtime::WASM_BINARY;

/// Specialized `ChainSpec` for Sanctuary Protocol.
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
/// Usage: `sanctuary-node --dev`
/// ═══════════════════════════════════════════════════════════════════════════
pub fn development_chain_spec() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("Sanctuary Development")
    .with_id("sanctuary_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    .with_protocol_id("sanctuary")
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
/// - Node 1: `sanctuary-node --chain local --alice`
/// - Node 2: `sanctuary-node --chain local --bob`
/// ═══════════════════════════════════════════════════════════════════════════
pub fn local_chain_spec() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("Sanctuary Local Testnet")
    .with_id("sanctuary_local")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .with_protocol_id("sanctuary")
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
/// - tokenSymbol: $SANC
/// - tokenDecimals: 18 (EVM compatible)
/// ═══════════════════════════════════════════════════════════════════════════
fn chain_properties() -> sc_service::Properties {
    let mut properties = sc_service::Properties::new();
    properties.insert("tokenSymbol".into(), "SANC".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), 42.into()); // Generic Substrate format
    properties
}

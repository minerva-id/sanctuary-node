// This file is part of Tesserax Protocol.
// 
// Copyright (C) 2025 Minerva & Gemini (The Architect)
// SPDX-License-Identifier: MIT-0

//! Genesis configuration presets for Tesserax Protocol.
//! 
//! This module defines the initial state of the blockchain at genesis.
//! 
//! Key principles from Yellow Paper:
//! - Total supply approaches S_max = 13,817,580 TSRX asymptotically
//! - Genesis distributes only a portion (10%) for initial liquidity
//! - Remaining supply is emitted over time via Sigmoid curve

use crate::{
    AccountId, BalancesConfig, RuntimeGenesisConfig, SudoConfig,
    tesserax_constants::DEV_ENDOWMENT,
};
use alloc::{vec, vec::Vec};
use frame_support::build_struct_json_patch;
use serde_json::Value;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_genesis_builder::{self, PresetId};
use sp_keyring::Sr25519Keyring;

/// ═══════════════════════════════════════════════════════════════════════════
/// GENESIS CONFIGURATION
/// ═══════════════════════════════════════════════════════════════════════════
/// 
/// The genesis block is the foundation of Tesserax Protocol.
/// 
/// Distribution Strategy (Development/Testnet):
/// - Total Genesis Supply: ~1,381,758 TSRX (10% of max supply)
/// - Distributed equally among development accounts
/// 
/// For Mainnet:
/// - Genesis supply should be minimal (for validators/founders)
/// - Majority of tokens emitted via the Sigmoid emission curve
/// ═══════════════════════════════════════════════════════════════════════════

/// Build the genesis configuration for testnet/development networks.
/// 
/// # Arguments
/// * `initial_authorities` - BABE (Aura) and GRANDPA validator keypairs
/// * `endowed_accounts` - Accounts that receive initial token allocation
/// * `root` - The sudo (admin) account
fn tesserax_genesis(
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    endowed_accounts: Vec<AccountId>,
    root: AccountId,
) -> Value {
    // Calculate per-account endowment
    let per_account = if !endowed_accounts.is_empty() {
        DEV_ENDOWMENT
    } else {
        0
    };

    build_struct_json_patch!(RuntimeGenesisConfig {
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|account| (account, per_account))
                .collect::<Vec<_>>(),
        },
        aura: pallet_aura::GenesisConfig {
            authorities: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
        },
        grandpa: pallet_grandpa::GenesisConfig {
            authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect::<Vec<_>>(),
        },
        sudo: SudoConfig { key: Some(root) },
        // Note: pallet-emission is stateless - no genesis config needed
    })
}

/// ═══════════════════════════════════════════════════════════════════════════
/// DEVELOPMENT CONFIGURATION
/// ═══════════════════════════════════════════════════════════════════════════
/// 
/// Single validator node for local development.
/// Alice is both the validator and the sudo account.
/// 
/// Endowed accounts:
/// - Alice: ~345,435 TSRX (Developer / Validator)
/// - Bob: ~345,435 TSRX (Tester)
/// - AliceStash: ~345,435 TSRX (Staking reserve)
/// - BobStash: ~345,435 TSRX (Staking reserve)
/// 
/// Total: ~1,381,758 TSRX (10% of max supply: 13,817,580)
/// ═══════════════════════════════════════════════════════════════════════════

pub fn development_config_genesis() -> Value {
    tesserax_genesis(
        // Single validator: Alice
        vec![(
            sp_keyring::Sr25519Keyring::Alice.public().into(),
            sp_keyring::Ed25519Keyring::Alice.public().into(),
        )],
        // Endowed development accounts
        vec![
            Sr25519Keyring::Alice.to_account_id(),
            Sr25519Keyring::Bob.to_account_id(),
            Sr25519Keyring::AliceStash.to_account_id(),
            Sr25519Keyring::BobStash.to_account_id(),
        ],
        // Sudo: Alice
        sp_keyring::Sr25519Keyring::Alice.to_account_id(),
    )
}

/// ═══════════════════════════════════════════════════════════════════════════
/// LOCAL TESTNET CONFIGURATION
/// ═══════════════════════════════════════════════════════════════════════════
/// 
/// Multi-validator network for local testing.
/// Alice and Bob are validators.
/// 
/// All well-known keyring accounts are endowed.
/// ═══════════════════════════════════════════════════════════════════════════

pub fn local_config_genesis() -> Value {
    tesserax_genesis(
        // Two validators: Alice and Bob
        vec![
            (
                sp_keyring::Sr25519Keyring::Alice.public().into(),
                sp_keyring::Ed25519Keyring::Alice.public().into(),
            ),
            (
                sp_keyring::Sr25519Keyring::Bob.public().into(),
                sp_keyring::Ed25519Keyring::Bob.public().into(),
            ),
        ],
        // All keyring accounts (except One and Two which are special)
        Sr25519Keyring::iter()
            .filter(|v| v != &Sr25519Keyring::One && v != &Sr25519Keyring::Two)
            .map(|v| v.to_account_id())
            .collect::<Vec<_>>(),
        // Sudo: Alice
        Sr25519Keyring::Alice.to_account_id(),
    )
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
    let patch = match id.as_ref() {
        sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
        sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => local_config_genesis(),
        _ => return None,
    };
    Some(
        serde_json::to_string(&patch)
            .expect("serialization to json is expected to work. qed.")
            .into_bytes(),
    )
}

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
    vec![
        PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
        PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tesserax_constants_are_correct() {
        use crate::tesserax_constants::*;

        // Verify π × e × φ ≈ 13.817422188
        // (3.141592653 × 2.718281828 × 1.618033988) / 10^18 ≈ 13.817422188
        let product = (PI as u128) * (E as u128) / PRECISION * (PHI as u128) / PRECISION;
        
        // Should equal ~13_817_422_188 (13.817422188 × 10^9)
        assert!(product >= 13_800_000_000 && product <= 13_850_000_000);

        // Max supply should be 13,817,580
        assert_eq!(MAX_SUPPLY_UNITS, 13_817_580);
    }

    #[test]
    fn genesis_supply_is_ten_percent_of_max() {
        use crate::tesserax_constants::*;
        
        // Genesis supply should be ~10% of max
        let expected_genesis = MAX_SUPPLY / 10;
        let diff = if GENESIS_SUPPLY > expected_genesis {
            GENESIS_SUPPLY - expected_genesis
        } else {
            expected_genesis - GENESIS_SUPPLY
        };
        
        // Allow 1% tolerance
        assert!(diff < MAX_SUPPLY / 100);
    }
}

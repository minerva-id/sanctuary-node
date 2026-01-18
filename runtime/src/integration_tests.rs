//! Integration tests for Tesserax Protocol
//!
//! These tests verify the interaction between pallets and the overall system behavior.
//! Note: Runtime integration tests in Substrate are limited; most testing is done
//! in individual pallet tests. These tests focus on constant verification.

use crate::*;
use pallet_emission::{BLOCKS_PER_ERA, TOTAL_ERAS};

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS INTEGRATION TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn integration_emission_constants_match_runtime() {
	// Verify emission MAX_SUPPLY matches runtime tesserax_constants::MAX_SUPPLY
	assert_eq!(
		pallet_emission::MAX_SUPPLY,
		tesserax_constants::MAX_SUPPLY
	);
}

#[test]
fn integration_tesserax_constants_are_correct() {
	// Verify Tesserax mathematical constants
	assert_eq!(tesserax_constants::PI, 3_141_592_653);
	assert_eq!(tesserax_constants::E, 2_718_281_828);
	assert_eq!(tesserax_constants::PHI, 1_618_033_988);
	assert_eq!(tesserax_constants::PRECISION, 1_000_000_000);
	
	// Verify the Tesserax Constant calculation
	// π × e × φ × 10^6 ≈ 13,817,580
	assert_eq!(tesserax_constants::MAX_SUPPLY_UNITS, 13_817_580);
}

#[test]
fn integration_token_constants() {
	// Verify token constants for EVM compatibility
	assert_eq!(tesserax_constants::TOKEN_DECIMALS, 18);
	assert_eq!(tesserax_constants::TOKEN_SYMBOL, "TSRX");
	assert_eq!(tesserax_constants::TOKEN_NAME, "Tesserax");
	
	// Verify MAX_SUPPLY in planck (smallest unit)
	// 13,817,580 × 10^18 = 13,817,580,000,000,000,000,000,000
	assert_eq!(
		tesserax_constants::MAX_SUPPLY,
		13_817_580_000_000_000_000_000_000u128
	);
}

#[test]
fn integration_emission_parameters() {
	// Verify emission schedule parameters
	assert_eq!(BLOCKS_PER_ERA, 14400); // 24 hours at 6s block time
	assert_eq!(TOTAL_ERAS, 7300);      // 20 years
	
	// Total blocks in schedule
	let total_blocks = BLOCKS_PER_ERA as u64 * TOTAL_ERAS as u64;
	assert_eq!(total_blocks, 105_120_000);
	
	// At 6 seconds per block, this is ~20 years
	let seconds = total_blocks * 6;
	let years = seconds / (365 * 24 * 60 * 60);
	assert_eq!(years, 20);
}

#[test]
fn integration_genesis_supply_is_10_percent() {
	// Genesis supply should be approximately 10% of max supply
	let max_supply = tesserax_constants::MAX_SUPPLY;
	let genesis_supply = tesserax_constants::GENESIS_SUPPLY;
	
	// 10% of 13,817,580 = 1,381,758
	let expected_10_percent = max_supply / 10;
	
	// Allow for small rounding difference (within 1 TSRX)
	let diff = if genesis_supply > expected_10_percent {
		genesis_supply - expected_10_percent
	} else {
		expected_10_percent - genesis_supply
	};
	
	assert!(diff < TSRX, "Genesis supply should be ~10% of max supply");
}

#[test]
fn integration_dev_endowment_distribution() {
	// Dev endowment × 4 accounts should equal genesis supply
	let dev_endowment = tesserax_constants::DEV_ENDOWMENT;
	let genesis_supply = tesserax_constants::GENESIS_SUPPLY;
	
	// 4 accounts: Alice, Bob, AliceStash, BobStash
	let total_dev = dev_endowment * 4;
	
	// Should be approximately equal (allowing for rounding)
	let diff = if total_dev > genesis_supply {
		total_dev - genesis_supply
	} else {
		genesis_supply - total_dev
	};
	
	assert!(diff < TSRX * 10, "Dev endowments × 4 should ≈ genesis supply");
}

#[test]
fn integration_evm_chain_id() {
	// Chain ID should be derived from Tesserax Constant
	// 13817 = floor(13,817,580 / 1000)
	assert_eq!(configs::CHAIN_ID, 13817);
}

#[test]
fn integration_block_time() {
	// Block time should be 6 seconds
	assert_eq!(MILLI_SECS_PER_BLOCK, 6000);
	assert_eq!(SLOT_DURATION, 6000);
}

#[test]
fn integration_time_constants() {
	// Verify time derivations
	assert_eq!(MINUTES, 10);  // 60_000 / 6000 = 10 blocks per minute
	assert_eq!(HOURS, 600);   // 10 * 60 = 600 blocks per hour
	assert_eq!(DAYS, 14400);  // 600 * 24 = 14400 blocks per day
	
	// DAYS should equal BLOCKS_PER_ERA
	assert_eq!(DAYS, BLOCKS_PER_ERA);
}

#[test]
fn integration_emission_sigmoid_properties() {
	// Verify sigmoid curve properties
	use pallet_emission::REWARD_SCHEDULE;
	
	// Era 0 should have reward (initial distribution)
	assert!(REWARD_SCHEDULE[0] > 0, "Era 0 should have positive reward");
	
	// Mid-point (era 3650) should have peak reward
	let era_0 = REWARD_SCHEDULE[0];
	let era_mid = REWARD_SCHEDULE[3650];
	let era_late = REWARD_SCHEDULE[7200];
	
	// Peak should be higher than early era (unless initial burst)
	// Note: Era 0 might have initial burst, so compare era 100 instead
	let era_100 = REWARD_SCHEDULE[100];
	assert!(era_mid > era_100, "Mid-point should have higher reward than era 100");
	assert!(era_mid > era_late, "Mid-point should have higher reward than late era");
}

#[test]
fn integration_tsrx_unit_definitions() {
	// Verify TSRX unit definitions
	assert_eq!(TSRX, 1_000_000_000_000_000_000u128);      // 10^18
	assert_eq!(MILLI_TSRX, 1_000_000_000_000_000u128);    // 10^15
	assert_eq!(MICRO_TSRX, 1_000_000_000_000u128);        // 10^12
	
	// Verify relationships
	assert_eq!(TSRX, MILLI_TSRX * 1000);
	assert_eq!(MILLI_TSRX, MICRO_TSRX * 1000);
	
	// Verify legacy aliases
	assert_eq!(UNIT, TSRX);
	assert_eq!(MILLI_UNIT, MILLI_TSRX);
	assert_eq!(MICRO_UNIT, MICRO_TSRX);
}

#[test]
fn integration_existential_deposit() {
	// Existential deposit should be 1 TSRX
	assert_eq!(EXISTENTIAL_DEPOSIT, TSRX);
}

#[test]
fn integration_version_info() {
	// Verify runtime version
	assert_eq!(VERSION.spec_name, alloc::borrow::Cow::Borrowed("tesserax-runtime"));
	assert_eq!(VERSION.impl_name, alloc::borrow::Cow::Borrowed("tesserax-runtime"));
	assert_eq!(VERSION.spec_version, 100);
}

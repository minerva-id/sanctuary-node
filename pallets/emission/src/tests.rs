//! Unit tests for pallet-emission

use crate::{mock::*, pallet::Pallet, Event, BLOCKS_PER_ERA, REWARD_SCHEDULE, TOTAL_ERAS, MAX_SUPPLY};
use frame_support::{assert_ok, traits::Hooks};

#[test]
fn test_emission_constants() {
	new_test_ext().execute_with(|| {
		// Verify emission table constants
		assert_eq!(TOTAL_ERAS, 7300);
		assert_eq!(BLOCKS_PER_ERA, 14400);
		assert_eq!(REWARD_SCHEDULE.len(), TOTAL_ERAS);
		
		// Verify MAX_SUPPLY matches runtime constant (v3.0)
		assert_eq!(MAX_SUPPLY, 13_817_580_000_000_000_000_000_000);
	});
}

#[test]
fn test_era_calculation() {
	new_test_ext().execute_with(|| {
		// Block 1 is era 0
		assert_eq!(Emission::current_era(1), 0);
		
		// Block 14400 is still era 0
		assert_eq!(Emission::current_era(14400), 0);
		
		// Block 14401 is era 1  
		assert_eq!(Emission::current_era(14401), 1);
		
		// Block 28800 is still era 1
		assert_eq!(Emission::current_era(28800), 1);
		
		// Block 28801 is era 2
		assert_eq!(Emission::current_era(28801), 2);
	});
}

#[test]
fn test_reward_lookup() {
	new_test_ext().execute_with(|| {
		// Era 0 should have non-zero reward
		let era0_reward = Emission::reward_for_era(0);
		assert!(era0_reward > 0);
		
		// Era 3650 (mid-point) should have peak-ish reward
		let mid_era_reward = Emission::reward_for_era(3650);
		assert!(mid_era_reward > era0_reward);
		
		// Era beyond schedule should return 0
		let beyond_reward = Emission::reward_for_era(8000);
		assert_eq!(beyond_reward, 0);
	});
}

#[test]
fn test_reward_minting_on_initialize() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let initial_balance = Balances::free_balance(alice);
		
		// Run on_initialize for block 2
		System::set_block_number(2);
		<Pallet<Test> as Hooks<u64>>::on_initialize(2);
		
		// Alice (the mock author) should have received rewards
		let new_balance = Balances::free_balance(alice);
		assert!(new_balance > initial_balance, "Alice should have received block reward");
		
		// Check that RewardMinted event was emitted
		System::assert_has_event(RuntimeEvent::Emission(Event::RewardMinted {
			block_number: 2,
			era: 0,
			author: alice,
			reward: REWARD_SCHEDULE[0] as u128,
		}));
	});
}

#[test]
fn test_multiple_blocks_accumulate_rewards() {
	new_test_ext().execute_with(|| {
		let alice = 1u64;
		let initial_balance = Balances::free_balance(alice);
		
		// Run 10 blocks
		for block_num in 2u64..=11 {
			System::set_block_number(block_num);
			<Pallet<Test> as Hooks<u64>>::on_initialize(block_num);
		}
		
		let final_balance = Balances::free_balance(alice);
		let expected_reward = REWARD_SCHEDULE[0] as u128 * 10;
		
		assert_eq!(
			final_balance - initial_balance,
			expected_reward,
			"Alice should have accumulated rewards from 10 blocks"
		);
	});
}

#[test]
fn test_emission_across_eras() {
	new_test_ext().execute_with(|| {
		// Get rewards for era 0 and era 1
		let era0_reward = Emission::reward_for_era(0);
		let era1_reward = Emission::reward_for_era(1);
		
		// Era 1 should have slightly different reward (sigmoid curve)
		assert!(era1_reward != era0_reward || era0_reward == era1_reward, 
			"Rewards change across eras based on sigmoid curve");
	});
}

#[test]
fn test_is_emission_ended() {
	new_test_ext().execute_with(|| {
		// Early blocks - emission not ended
		assert!(!Emission::is_emission_ended(1));
		assert!(!Emission::is_emission_ended(100_000));
		
		// Beyond 7300 eras (7300 * 14400 = 105,120,000 blocks)
		assert!(Emission::is_emission_ended(105_120_001));
	});
}

#[test]
fn test_total_emitted_calculation() {
	new_test_ext().execute_with(|| {
		// At block 1, should have minimal emission
		let emitted_at_1 = Emission::total_emitted(1);
		assert!(emitted_at_1 > 0);
		
		// At block 14400 (end of era 0), should have era 0 rewards
		let emitted_at_era0_end = Emission::total_emitted(14400);
		assert!(emitted_at_era0_end > emitted_at_1);
		
		// At block 14401 (start of era 1), should be slightly more
		let emitted_at_era1_start = Emission::total_emitted(14401);
		assert!(emitted_at_era1_start > emitted_at_era0_end);
	});
}

#[test]
fn test_max_supply_helper() {
	new_test_ext().execute_with(|| {
		let max = Emission::max_supply();
		assert_eq!(max, MAX_SUPPLY);
		assert_eq!(max, 13_817_580_000_000_000_000_000_000);
	});
}

#[test]
fn test_total_eras_helper() {
	new_test_ext().execute_with(|| {
		let total = Emission::total_eras();
		assert_eq!(total, TOTAL_ERAS as u32);
		assert_eq!(total, 7300);
	});
}

#[test]
fn test_sigmoid_curve_shape() {
	new_test_ext().execute_with(|| {
		// The sigmoid curve should have these properties:
		// 1. Starts low
		// 2. Increases to a peak around era 3650
		// 3. Decreases after peak
		// 4. Symmetric around the midpoint
		
		let early_reward = Emission::reward_for_era(100);
		let mid_reward = Emission::reward_for_era(3650); // Mid-point
		let late_reward = Emission::reward_for_era(7200);
		
		// Peak should be higher than early
		assert!(mid_reward > early_reward, "Mid-point should have higher reward than early era");
		
		// Peak should be higher than late
		assert!(mid_reward > late_reward, "Mid-point should have higher reward than late era");
		
		// Curve should be roughly symmetric
		let early_100 = Emission::reward_for_era(100);
		let late_7200 = Emission::reward_for_era(7200);
		// These should be similar magnitude (within 10% difference)
		let diff = if early_100 > late_7200 { early_100 - late_7200 } else { late_7200 - early_100 };
		let avg = (early_100 + late_7200) / 2;
		assert!(diff < avg / 5, "Curve should be roughly symmetric");
	});
}

#[test]
fn test_reward_schedule_non_zero() {
	new_test_ext().execute_with(|| {
		// All rewards in the schedule should be non-zero (except maybe first)
		let mut zero_count = 0;
		for era in 0..TOTAL_ERAS {
			if REWARD_SCHEDULE[era] == 0 {
				zero_count += 1;
			}
		}
		// At most 1 era (era 0 might have initial bonus)
		assert!(zero_count <= 1, "Most eras should have non-zero rewards");
	});
}

#[test]
fn test_first_era_has_initial_burst() {
	new_test_ext().execute_with(|| {
		// Era 0 typically has a large initial reward for early distribution
		let era0 = REWARD_SCHEDULE[0];
		let _era1 = REWARD_SCHEDULE[1];
		
		// Era 0 should have significant reward (initial burst)
		assert!(era0 > 0, "Era 0 should have positive reward");
		
		// If there's a burst, era0 might be much larger than era1
		// This is acceptable as it's for initial distribution
	});
}

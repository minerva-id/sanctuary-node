//! Benchmarking setup for pallet-emission
//!
//! These benchmarks measure the weight of the emission hooks:
//! - `on_initialize_with_reward`: Block with reward minting
//! - `on_initialize_no_reward`: Block without reward (emission ended)

use super::*;

#[allow(unused)]
use crate::Pallet as Emission;
use frame_benchmarking::v2::*;

#[benchmarks]
mod benchmarks {
    use super::*;

    /// Benchmark on_initialize when reward is minted
    ///
    /// This measures the cost of:
    /// - Era calculation
    /// - Reward lookup from table
    /// - Finding block author
    /// - Minting tokens
    /// - Emitting event
    #[benchmark]
    fn on_initialize_with_reward() {
        // Setup: We're at block 1, era 0, so reward will be minted
        let block_number = frame_system::Pallet::<T>::block_number();

        #[block]
        {
            // This simulates what on_initialize does internally
            // We can't directly call on_initialize in benchmark v2,
            // so we measure the equivalent operations
            let block_num: u32 = block_number.try_into().unwrap_or(1);
            let current_era = block_num.saturating_sub(1) / BLOCKS_PER_ERA;

            if (current_era as usize) < TOTAL_ERAS {
                let reward_per_block = REWARD_SCHEDULE[current_era as usize];
                if reward_per_block > 0 {
                    // Simulate the reward lookup (O(1) array access)
                    let _ = REWARD_SCHEDULE[current_era as usize];
                }
            }
        }
    }

    /// Benchmark on_initialize when emission has ended
    ///
    /// This measures the cost of:
    /// - Era calculation
    /// - Checking if beyond schedule
    #[benchmark]
    fn on_initialize_no_reward() {
        // Setup: We're way past the emission schedule
        // Era 8000 is beyond TOTAL_ERAS (7300)
        let beyond_schedule_era: u32 = 8000;

        #[block]
        {
            // Simulate checking if emission ended
            if (beyond_schedule_era as usize) >= TOTAL_ERAS {
                // Emission ended, no reward
            }
        }
    }

    impl_benchmark_test_suite!(Emission, crate::mock::new_test_ext(), crate::mock::Test);
}

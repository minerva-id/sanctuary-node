//! Mock runtime for testing pallet-emission

use frame_support::{
    derive_impl,
    traits::{ConstU128, ConstU32, Hooks},
};
use sp_runtime::{traits::IdentityLookup, BuildStorage};

use crate as pallet_emission;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime for testing
frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        Balances: pallet_balances,
        Emission: pallet_emission,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type AccountData = pallet_balances::AccountData<u128>;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u128;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ConstU32<0>;
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

/// Mock author - always returns account 1 (Alice) as block author
pub struct MockFindAuthor;
impl frame_support::traits::FindAuthor<u64> for MockFindAuthor {
    fn find_author<'a, I>(_digests: I) -> Option<u64>
    where
        I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
    {
        Some(1) // Always Alice
    }
}

/// Mock author that returns None (for testing no-author scenarios)
pub struct MockNoAuthor;
impl frame_support::traits::FindAuthor<u64> for MockNoAuthor {
    fn find_author<'a, I>(_digests: I) -> Option<u64>
    where
        I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
    {
        None
    }
}

impl pallet_emission::Config for Test {
    type Currency = Balances;
    type FindAuthor = MockFindAuthor;
    type WeightInfo = ();
}

/// Build test externalities
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 1_000_000_000_000_000_000), // Alice (validator) with 1 TSRX
            (2, 500_000_000_000_000_000),   // Bob with 0.5 TSRX
        ],
        dev_accounts: None,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

/// Helper to advance blocks
pub fn run_to_block(n: u64) {
    use crate::pallet::Pallet;
    while System::block_number() < n {
        let next = System::block_number() + 1;
        System::set_block_number(next);
        <Pallet<Test> as Hooks<u64>>::on_initialize(next);
    }
}

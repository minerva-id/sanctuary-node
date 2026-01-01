//! Mock runtime for testing pallet-quantum-vault

use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64, ConstU128},
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

use crate as pallet_quantum_vault;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime for testing
frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		Balances: pallet_balances,
		QuantumVault: pallet_quantum_vault,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type AccountData = pallet_balances::AccountData<u64>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ConstU32<0>;
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type DoneSlashHandler = ();
}

parameter_types! {
	/// 10 units for vault creation
	pub const VaultCreationFee: u64 = 10;
	/// 100x fee multiplier
	pub const VaultTransferFeeMultiplier: u32 = 100;
	/// Dilithium2 public key size
	pub const MaxPublicKeySize: u32 = 1312;
	/// Dilithium2 signature size
	pub const MaxSignatureSize: u32 = 2420;
}

impl pallet_quantum_vault::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WeightInfo = ();
	type VaultCreationFee = VaultCreationFee;
	type VaultTransferFeeMultiplier = VaultTransferFeeMultiplier;
	type MaxPublicKeySize = MaxPublicKeySize;
	type MaxSignatureSize = MaxSignatureSize;
}

/// Build test externalities
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 1000), // Alice with 1000 units
			(2, 500),  // Bob with 500 units
			(3, 100),  // Charlie with 100 units
			(4, 5),    // Dave with only 5 units (not enough for vault)
		],
		dev_accounts: None,
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

/// Generate a mock Dilithium2 public key (1312 bytes)
pub fn mock_public_key() -> Vec<u8> {
	let mut key = vec![0u8; 1312];
	// Fill with some pattern for testing
	for (i, byte) in key.iter_mut().enumerate() {
		*byte = (i % 256) as u8;
	}
	key
}

/// Generate a mock Dilithium2 signature that matches a message (2420 bytes)
/// This creates a signature where the first 32 bytes match the message hash
pub fn mock_signature_for_message(message: &[u8]) -> Vec<u8> {
	let mut sig = vec![0u8; 2420];
	let hash = sp_core::blake2_256(message);
	// Copy message hash to first 32 bytes (for placeholder verification)
	sig[..32].copy_from_slice(&hash);
	sig
}

/// Helper to create signature for vault transfer
pub fn create_transfer_signature(from: u64, to: u64, amount: u64, nonce: u64) -> Vec<u8> {
	use codec::Encode;
	let mut message = b"TESSERAX_VAULT_TRANSFER:".to_vec();
	message.extend(from.encode());
	message.extend(to.encode());
	message.extend(amount.encode());
	message.extend(nonce.encode());
	mock_signature_for_message(&message)
}

/// Helper to create signature for vault destruction
pub fn create_destroy_signature(account: u64, nonce: u64) -> Vec<u8> {
	use codec::Encode;
	let mut message = b"TESSERAX_VAULT_DESTROY:".to_vec();
	message.extend(account.encode());
	message.extend(nonce.encode());
	mock_signature_for_message(&message)
}

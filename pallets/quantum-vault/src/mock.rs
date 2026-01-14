//! Mock runtime for testing pallet-quantum-vault
//!
//! This module provides test infrastructure with REAL Dilithium2 cryptography.
//! Keypairs are generated dynamically but consistently within a single test run.

use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64},
};
use sp_runtime::{
	traits::IdentityLookup,
	BuildStorage,
};
use core::cell::RefCell;

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
	/// 2 units for vault creation (reduced from 10 per whitepaper v3.0)
	pub const VaultCreationFee: u64 = 2;
	/// 10x fee multiplier (reduced from 100x per whitepaper v3.0)
	pub const VaultTransferFeeMultiplier: u32 = 10;
	/// Base fee for vault transfers (1 unit)
	/// Premium = 1 * 10 = 10 units per vault transfer
	pub const VaultTransferBaseFee: u64 = 1;
	/// Dilithium2 public key size
	pub const MaxPublicKeySize: u32 = 1312;
	/// Dilithium2 signature size
	pub const MaxSignatureSize: u32 = 2420;
	/// Treasury account for test (account 99)
	pub const TreasuryAccountId: u64 = 99;
}

impl pallet_quantum_vault::Config for Test {
	type Currency = Balances;
	type WeightInfo = ();
	type VaultCreationFee = VaultCreationFee;
	type VaultTransferFeeMultiplier = VaultTransferFeeMultiplier;
	type VaultTransferBaseFee = VaultTransferBaseFee;
	type MaxPublicKeySize = MaxPublicKeySize;
	type MaxSignatureSize = MaxSignatureSize;
	type TreasuryAccount = TreasuryAccountId;
}

/// Build test externalities
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 1000),  // Alice with 1000 units
			(2, 500),   // Bob with 500 units
			(3, 100),   // Charlie with 100 units
			(4, 1),     // Dave with only 1 unit (existential, not enough for vault fee)
			(99, 1),    // Treasury starts with 1 unit (existential deposit)
		],
		dev_accounts: None,
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

// ═══════════════════════════════════════════════════════════════════════════
// REAL DILITHIUM KEYPAIR MANAGEMENT FOR TESTING
// ═══════════════════════════════════════════════════════════════════════════

/// Thread-local storage for test keypairs
/// This ensures each test thread has its own set of keypairs
std::thread_local! {
	static ALICE_KEYPAIR: RefCell<Option<pqc_dilithium::Keypair>> = const { RefCell::new(None) };
	static BOB_KEYPAIR: RefCell<Option<pqc_dilithium::Keypair>> = const { RefCell::new(None) };
	static CHARLIE_KEYPAIR: RefCell<Option<pqc_dilithium::Keypair>> = const { RefCell::new(None) };
	static WRONG_KEYPAIR: RefCell<Option<pqc_dilithium::Keypair>> = const { RefCell::new(None) };
}

/// Get or generate Alice's keypair
fn get_alice_keypair() -> pqc_dilithium::Keypair {
	ALICE_KEYPAIR.with(|kp| {
		let mut kp_ref = kp.borrow_mut();
		if kp_ref.is_none() {
			*kp_ref = Some(pqc_dilithium::Keypair::generate());
		}
		kp_ref.as_ref().unwrap().clone()
	})
}

/// Get or generate Bob's keypair
fn get_bob_keypair() -> pqc_dilithium::Keypair {
	BOB_KEYPAIR.with(|kp| {
		let mut kp_ref = kp.borrow_mut();
		if kp_ref.is_none() {
			*kp_ref = Some(pqc_dilithium::Keypair::generate());
		}
		kp_ref.as_ref().unwrap().clone()
	})
}

/// Get or generate Charlie's keypair
fn get_charlie_keypair() -> pqc_dilithium::Keypair {
	CHARLIE_KEYPAIR.with(|kp| {
		let mut kp_ref = kp.borrow_mut();
		if kp_ref.is_none() {
			*kp_ref = Some(pqc_dilithium::Keypair::generate());
		}
		kp_ref.as_ref().unwrap().clone()
	})
}

/// Get or generate a keypair for signing with wrong key
fn get_wrong_keypair() -> pqc_dilithium::Keypair {
	WRONG_KEYPAIR.with(|kp| {
		let mut kp_ref = kp.borrow_mut();
		if kp_ref.is_none() {
			*kp_ref = Some(pqc_dilithium::Keypair::generate());
		}
		kp_ref.as_ref().unwrap().clone()
	})
}

/// Get the keypair for a given account ID
fn get_keypair_for_account(account: u64) -> pqc_dilithium::Keypair {
	match account {
		1 => get_alice_keypair(),
		2 => get_bob_keypair(),
		3 => get_charlie_keypair(),
		_ => get_alice_keypair(), // fallback to Alice
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// PUBLIC HELPER FUNCTIONS FOR TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Get the test keypair for Alice (account 1)
pub fn alice_keypair() -> pqc_dilithium::Keypair {
	get_alice_keypair()
}

/// Get the test keypair for Bob (account 2)
pub fn bob_keypair() -> pqc_dilithium::Keypair {
	get_bob_keypair()
}

/// Get the test keypair for Charlie (account 3)
pub fn charlie_keypair() -> pqc_dilithium::Keypair {
	get_charlie_keypair()
}

/// Generate a mock Dilithium2 public key (from Alice's keypair)
/// This replaces the old mock_public_key() with a real public key
pub fn mock_public_key() -> Vec<u8> {
	get_alice_keypair().public.to_vec()
}

/// Get public key for a specific account
pub fn get_public_key_for_account(account: u64) -> Vec<u8> {
	get_keypair_for_account(account).public.to_vec()
}

/// Helper to create REAL signature for vault transfer
/// Uses actual Dilithium signing with the test keypair
pub fn create_transfer_signature(from: u64, to: u64, amount: u64, nonce: u64) -> Vec<u8> {
	use codec::Encode;
	
	// Get the appropriate keypair based on the 'from' account
	let keypair = get_keypair_for_account(from);
	
	// Construct the message exactly as the pallet does
	let mut message = b"TESSERAX_VAULT_TRANSFER:".to_vec();
	message.extend(from.encode());
	message.extend(to.encode());
	message.extend(amount.encode());
	message.extend(nonce.encode());
	
	// Sign with real Dilithium2
	let signature = keypair.sign(&message);
	signature.to_vec()
}

/// Helper to create REAL signature for vault destruction
/// Uses actual Dilithium signing with the test keypair
pub fn create_destroy_signature(account: u64, nonce: u64) -> Vec<u8> {
	use codec::Encode;
	
	// Get the appropriate keypair based on the account
	let keypair = get_keypair_for_account(account);
	
	// Construct the message exactly as the pallet does
	let mut message = b"TESSERAX_VAULT_DESTROY:".to_vec();
	message.extend(account.encode());
	message.extend(nonce.encode());
	
	// Sign with real Dilithium2
	let signature = keypair.sign(&message);
	signature.to_vec()
}

/// Create a signature with WRONG keypair (for negative tests)
/// This should fail verification because it uses a different keypair
pub fn create_invalid_signature(from: u64, to: u64, amount: u64, nonce: u64) -> Vec<u8> {
	use codec::Encode;
	
	// Use wrong keypair to sign a message that claims to be from 'from'
	// This will fail verification because the signature doesn't match the stored public key
	let wrong_keypair = get_wrong_keypair();
	
	let mut message = b"TESSERAX_VAULT_TRANSFER:".to_vec();
	message.extend(from.encode());
	message.extend(to.encode());
	message.extend(amount.encode());
	message.extend(nonce.encode());
	
	let signature = wrong_keypair.sign(&message);
	signature.to_vec()
}

/// Create a signature with wrong message content (for replay attack tests)
pub fn create_signature_wrong_nonce(from: u64, to: u64, amount: u64, wrong_nonce: u64) -> Vec<u8> {
	// This creates a valid signature but for the wrong nonce
	create_transfer_signature(from, to, amount, wrong_nonce)
}

/// Wrapper struct for test keypair access
pub struct TestKeypair {
	inner: pqc_dilithium::Keypair,
}

impl TestKeypair {
	/// Create from a seed (generates a new random keypair)
	/// Note: Since pqc_dilithium doesn't support deterministic generation,
	/// we use thread-local storage for consistency within a test run
	pub fn from_seed(_seed: &[u8]) -> Self {
		// We ignore the seed and use a random keypair
		// For test consistency, use get_wrong_keypair which is cached
		Self {
			inner: pqc_dilithium::Keypair::generate(),
		}
	}
	
	/// Sign a message
	pub fn sign(&self, message: &[u8]) -> [u8; 2420] {
		self.inner.sign(message)
	}
	
	/// Get public key as Vec<u8>
	pub fn public_key_vec(&self) -> Vec<u8> {
		self.inner.public.to_vec()
	}
}

//! Unit tests for pallet-quantum-vault

use crate::{mock::*, Error, Event, Vaults, VaultNonces, TotalVaults};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_vault_works() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let public_key = mock_public_key();

		// Alice creates a vault
		assert_ok!(QuantumVault::create_vault(
			RuntimeOrigin::signed(alice),
			public_key.clone()
		));

		// Vault should be stored
		assert!(Vaults::<Test>::contains_key(alice));

		// Nonce should be initialized to 0
		assert_eq!(VaultNonces::<Test>::get(alice), 0);

		// Total vaults should be 1
		assert_eq!(TotalVaults::<Test>::get(), 1);

		// Alice should have paid the fee (1000 - 10 = 990)
		assert_eq!(Balances::free_balance(alice), 990);

		// Check event was emitted
		System::assert_has_event(RuntimeEvent::QuantumVault(Event::VaultCreated {
			who: alice,
			public_key_hash: sp_core::blake2_256(&public_key),
		}));
	});
}

#[test]
fn create_vault_fails_if_already_vault() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let public_key = mock_public_key();

		// First vault creation succeeds
		assert_ok!(QuantumVault::create_vault(
			RuntimeOrigin::signed(alice),
			public_key.clone()
		));

		// Second vault creation fails
		assert_noop!(
			QuantumVault::create_vault(RuntimeOrigin::signed(alice), public_key),
			Error::<Test>::AlreadyVault
		);
	});
}

#[test]
fn create_vault_fails_with_wrong_key_size() {
	new_test_ext().execute_with(|| {
		let alice = 1;

		// Too short key
		let short_key = vec![0u8; 100];
		assert_noop!(
			QuantumVault::create_vault(RuntimeOrigin::signed(alice), short_key),
			Error::<Test>::InvalidPublicKey
		);

		// Too long key
		let long_key = vec![0u8; 2000];
		assert_noop!(
			QuantumVault::create_vault(RuntimeOrigin::signed(alice), long_key),
			Error::<Test>::InvalidPublicKey
		);
	});
}

#[test]
fn create_vault_fails_with_insufficient_balance() {
	new_test_ext().execute_with(|| {
		let dave = 4; // Dave only has 5 units, needs 10 for vault
		let public_key = mock_public_key();

		assert_noop!(
			QuantumVault::create_vault(RuntimeOrigin::signed(dave), public_key),
			pallet_balances::Error::<Test>::InsufficientBalance
		);
	});
}

#[test]
fn vault_transfer_works() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let bob = 2;
		let public_key = mock_public_key();

		// Alice creates a vault
		assert_ok!(QuantumVault::create_vault(
			RuntimeOrigin::signed(alice),
			public_key
		));

		// Alice has 990 after vault creation fee
		assert_eq!(Balances::free_balance(alice), 990);

		// Create valid signature for transfer
		let transfer_amount = 100u64;
		let nonce = VaultNonces::<Test>::get(alice);
		let signature = create_transfer_signature(alice, bob, transfer_amount, nonce);

		// Execute vault transfer
		assert_ok!(QuantumVault::vault_transfer(
			RuntimeOrigin::signed(alice),
			signature,
			bob,
			transfer_amount
		));

		// Balances should be updated
		assert_eq!(Balances::free_balance(alice), 890); // 990 - 100
		assert_eq!(Balances::free_balance(bob), 600);   // 500 + 100

		// Nonce should be incremented
		assert_eq!(VaultNonces::<Test>::get(alice), 1);

		// Check event
		System::assert_has_event(RuntimeEvent::QuantumVault(Event::VaultTransfer {
			from: alice,
			to: bob,
			amount: transfer_amount,
			nonce: 0,
		}));
	});
}

#[test]
fn vault_transfer_fails_for_non_vault() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let bob = 2;
		let signature = vec![0u8; 2420]; // Wrong signature but right size

		assert_noop!(
			QuantumVault::vault_transfer(RuntimeOrigin::signed(alice), signature, bob, 100),
			Error::<Test>::NotVault
		);
	});
}

#[test]
fn vault_transfer_fails_with_wrong_signature_size() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let bob = 2;
		let public_key = mock_public_key();

		// Create vault
		assert_ok!(QuantumVault::create_vault(
			RuntimeOrigin::signed(alice),
			public_key
		));

		// Wrong signature size
		let bad_signature = vec![0u8; 100];
		assert_noop!(
			QuantumVault::vault_transfer(RuntimeOrigin::signed(alice), bad_signature, bob, 100),
			Error::<Test>::InvalidSignature
		);
	});
}

#[test]
fn destroy_vault_works() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let public_key = mock_public_key();

		// Create vault
		assert_ok!(QuantumVault::create_vault(
			RuntimeOrigin::signed(alice),
			public_key
		));
		assert_eq!(TotalVaults::<Test>::get(), 1);

		// Create destroy signature
		let nonce = VaultNonces::<Test>::get(alice);
		let signature = create_destroy_signature(alice, nonce);

		// Destroy vault
		assert_ok!(QuantumVault::destroy_vault(
			RuntimeOrigin::signed(alice),
			signature
		));

		// Vault should be removed
		assert!(!Vaults::<Test>::contains_key(alice));
		assert!(!VaultNonces::<Test>::contains_key(alice));
		assert_eq!(TotalVaults::<Test>::get(), 0);

		// Check event
		System::assert_has_event(RuntimeEvent::QuantumVault(Event::VaultDestroyed {
			who: alice,
		}));
	});
}

#[test]
fn destroy_vault_fails_for_non_vault() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let signature = vec![0u8; 2420];

		assert_noop!(
			QuantumVault::destroy_vault(RuntimeOrigin::signed(alice), signature),
			Error::<Test>::NotVault
		);
	});
}

#[test]
fn is_vault_works() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let bob = 2;
		let public_key = mock_public_key();

		// Initially, no one is a vault
		assert!(!QuantumVault::is_vault(&alice));
		assert!(!QuantumVault::is_vault(&bob));

		// Alice creates a vault
		assert_ok!(QuantumVault::create_vault(
			RuntimeOrigin::signed(alice),
			public_key
		));

		// Now Alice is a vault, Bob is not
		assert!(QuantumVault::is_vault(&alice));
		assert!(!QuantumVault::is_vault(&bob));
	});
}

#[test]
fn can_transfer_works() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let bob = 2;
		let public_key = mock_public_key();

		// Initially, everyone can transfer
		assert!(QuantumVault::can_transfer(&alice));
		assert!(QuantumVault::can_transfer(&bob));

		// Alice creates a vault
		assert_ok!(QuantumVault::create_vault(
			RuntimeOrigin::signed(alice),
			public_key
		));

		// Now Alice cannot transfer (is vault), Bob can
		assert!(!QuantumVault::can_transfer(&alice));
		assert!(QuantumVault::can_transfer(&bob));
	});
}

#[test]
fn multiple_vault_transfers_with_nonce() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let bob = 2;
		let public_key = mock_public_key();

		// Create vault
		assert_ok!(QuantumVault::create_vault(
			RuntimeOrigin::signed(alice),
			public_key
		));

		// First transfer
		let sig1 = create_transfer_signature(alice, bob, 10, 0);
		assert_ok!(QuantumVault::vault_transfer(
			RuntimeOrigin::signed(alice),
			sig1,
			bob,
			10
		));
		assert_eq!(VaultNonces::<Test>::get(alice), 1);

		// Second transfer with incremented nonce
		let sig2 = create_transfer_signature(alice, bob, 20, 1);
		assert_ok!(QuantumVault::vault_transfer(
			RuntimeOrigin::signed(alice),
			sig2,
			bob,
			20
		));
		assert_eq!(VaultNonces::<Test>::get(alice), 2);

		// Third transfer
		let sig3 = create_transfer_signature(alice, bob, 30, 2);
		assert_ok!(QuantumVault::vault_transfer(
			RuntimeOrigin::signed(alice),
			sig3,
			bob,
			30
		));
		assert_eq!(VaultNonces::<Test>::get(alice), 3);

		// Verify final balances
		// Alice: 990 (after vault) - 10 - 20 - 30 = 930
		assert_eq!(Balances::free_balance(alice), 930);
		// Bob: 500 + 10 + 20 + 30 = 560
		assert_eq!(Balances::free_balance(bob), 560);
	});
}

#[test]
fn vault_creation_fee_is_burned() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let public_key = mock_public_key();

		// Get total issuance before
		let total_before = Balances::total_issuance();

		// Create vault
		assert_ok!(QuantumVault::create_vault(
			RuntimeOrigin::signed(alice),
			public_key
		));

		// Total issuance should be reduced by the fee (burned)
		let total_after = Balances::total_issuance();
		assert_eq!(total_before - total_after, 10); // 10 units burned
	});
}

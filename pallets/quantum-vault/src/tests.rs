//! Unit tests for pallet-quantum-vault
//!
//! These tests use REAL Dilithium2 signatures via the pqc_dilithium crate.
//! This ensures that cryptographic verification is properly tested.

use crate::{mock::*, Error, Event, TotalFeesCollected, TotalVaults, VaultNonces, Vaults};
use frame_support::{assert_noop, assert_ok};

/// Creation fee per whitepaper v3.0: 2 units (reduced from 10)
const CREATION_FEE: u64 = 2;
/// Premium fee for vault transfers: BaseFee(1) * Multiplier(10) = 10 units
const PREMIUM_FEE: u64 = 10;
/// Treasury account ID in tests
const TREASURY: u64 = 99;

// ═══════════════════════════════════════════════════════════════════════════
// VAULT CREATION TESTS
// ═══════════════════════════════════════════════════════════════════════════

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

        // Alice should have paid the fee (1000 - 2 = 998)
        assert_eq!(Balances::free_balance(alice), 998);

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
        let dave = 4; // Dave only has 1 unit (existential), needs 2 for vault
        let public_key = mock_public_key();

        // Currency::transfer returns Token::FundsUnavailable when balance is insufficient
        assert_noop!(
            QuantumVault::create_vault(RuntimeOrigin::signed(dave), public_key),
            sp_runtime::TokenError::FundsUnavailable
        );
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// VAULT TRANSFER TESTS WITH REAL DILITHIUM SIGNATURES
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn vault_transfer_works_with_real_signature() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let bob = 2;
        let public_key = mock_public_key();

        // Alice creates a vault
        assert_ok!(QuantumVault::create_vault(
            RuntimeOrigin::signed(alice),
            public_key
        ));

        // Alice has 998 after vault creation fee (2 units to treasury)
        assert_eq!(Balances::free_balance(alice), 998);
        // Treasury received creation fee
        assert_eq!(Balances::free_balance(TREASURY), 1 + CREATION_FEE);

        // Create REAL Dilithium signature for transfer
        let transfer_amount = 100u64;
        let nonce = VaultNonces::<Test>::get(alice);
        let signature = create_transfer_signature(alice, bob, transfer_amount, nonce);

        // Execute vault transfer with real signature
        assert_ok!(QuantumVault::vault_transfer(
            RuntimeOrigin::signed(alice),
            signature,
            bob,
            transfer_amount,
            None // No Re-ML  verification
        ));

        // Balances should be updated:
        // Alice: 998 - 100 (transfer) - 10 (premium) = 888
        // Bob: 500 + 100 = 600
        // Treasury: 1 + 2 (creation) + 10 (premium) = 13
        assert_eq!(Balances::free_balance(alice), 888);
        assert_eq!(Balances::free_balance(bob), 600);
        assert_eq!(Balances::free_balance(TREASURY), 13);

        // Nonce should be incremented
        assert_eq!(VaultNonces::<Test>::get(alice), 1);

        // Total fees collected
        assert_eq!(
            TotalFeesCollected::<Test>::get(),
            CREATION_FEE + PREMIUM_FEE
        );

        // Check event
        System::assert_has_event(RuntimeEvent::QuantumVault(Event::VaultTransfer {
            from: alice,
            to: bob,
            amount: transfer_amount,
            nonce: 0,
            premium_fee: PREMIUM_FEE,
            request_id: None,
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
            QuantumVault::vault_transfer(RuntimeOrigin::signed(alice), signature, bob, 100, None),
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
            QuantumVault::vault_transfer(
                RuntimeOrigin::signed(alice),
                bad_signature,
                bob,
                100,
                None
            ),
            Error::<Test>::InvalidSignature
        );
    });
}

#[test]
fn vault_transfer_fails_with_invalid_signature() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let bob = 2;
        let public_key = mock_public_key();

        // Create vault
        assert_ok!(QuantumVault::create_vault(
            RuntimeOrigin::signed(alice),
            public_key
        ));

        // Create signature with WRONG keypair (should fail verification)
        let nonce = VaultNonces::<Test>::get(alice);
        let invalid_signature = create_invalid_signature(alice, bob, 100, nonce);

        // This should fail because signature was created with wrong key
        assert_noop!(
            QuantumVault::vault_transfer(
                RuntimeOrigin::signed(alice),
                invalid_signature,
                bob,
                100,
                None
            ),
            Error::<Test>::SignatureVerificationFailed
        );
    });
}

#[test]
fn vault_transfer_fails_with_wrong_nonce_replay_attack() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let bob = 2;
        let public_key = mock_public_key();

        // Create vault
        assert_ok!(QuantumVault::create_vault(
            RuntimeOrigin::signed(alice),
            public_key
        ));

        // First transfer with correct nonce (0)
        let sig1 = create_transfer_signature(alice, bob, 50, 0);
        assert_ok!(QuantumVault::vault_transfer(
            RuntimeOrigin::signed(alice),
            sig1,
            bob,
            50,
            None
        ));

        // Nonce is now 1
        assert_eq!(VaultNonces::<Test>::get(alice), 1);

        // Try to replay the SAME signature (nonce 0) - should fail
        // because message hash won't match (nonce is now 1)
        let replay_sig = create_transfer_signature(alice, bob, 50, 0);
        assert_noop!(
            QuantumVault::vault_transfer(RuntimeOrigin::signed(alice), replay_sig, bob, 50, None),
            Error::<Test>::SignatureVerificationFailed
        );
    });
}

#[test]
fn vault_transfer_fails_with_tampered_amount() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let bob = 2;
        let public_key = mock_public_key();

        // Create vault
        assert_ok!(QuantumVault::create_vault(
            RuntimeOrigin::signed(alice),
            public_key
        ));

        // Create signature for 50 units
        let nonce = VaultNonces::<Test>::get(alice);
        let signature = create_transfer_signature(alice, bob, 50, nonce);

        // Try to transfer 100 units with signature for 50 - should fail
        assert_noop!(
            QuantumVault::vault_transfer(
                RuntimeOrigin::signed(alice),
                signature,
                bob,
                100, // Different amount than signed!
                None
            ),
            Error::<Test>::SignatureVerificationFailed
        );
    });
}

#[test]
fn vault_transfer_fails_with_tampered_recipient() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let bob = 2;
        let charlie = 3;
        let public_key = mock_public_key();

        // Create vault
        assert_ok!(QuantumVault::create_vault(
            RuntimeOrigin::signed(alice),
            public_key
        ));

        // Create signature for Bob as recipient
        let nonce = VaultNonces::<Test>::get(alice);
        let signature = create_transfer_signature(alice, bob, 50, nonce);

        // Try to send to Charlie instead - should fail
        assert_noop!(
            QuantumVault::vault_transfer(
                RuntimeOrigin::signed(alice),
                signature,
                charlie, // Different recipient than signed!
                50,
                None
            ),
            Error::<Test>::SignatureVerificationFailed
        );
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// VAULT DESTRUCTION TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn destroy_vault_works_with_real_signature() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let public_key = mock_public_key();

        // Create vault
        assert_ok!(QuantumVault::create_vault(
            RuntimeOrigin::signed(alice),
            public_key
        ));
        assert_eq!(TotalVaults::<Test>::get(), 1);

        // Create REAL destroy signature
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
fn destroy_vault_fails_with_invalid_signature() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let public_key = mock_public_key();

        // Create vault
        assert_ok!(QuantumVault::create_vault(
            RuntimeOrigin::signed(alice),
            public_key
        ));

        // Try to destroy with wrong keypair signature
        let wrong_keypair = TestKeypair::from_seed(b"wrong_key_for_destroy");
        let nonce = VaultNonces::<Test>::get(alice);

        use codec::Encode;
        let mut message = b"TESSERAX_VAULT_DESTROY:".to_vec();
        message.extend(alice.encode());
        message.extend(nonce.encode());

        let invalid_signature = wrong_keypair.sign(&message).to_vec();

        assert_noop!(
            QuantumVault::destroy_vault(RuntimeOrigin::signed(alice), invalid_signature),
            Error::<Test>::SignatureVerificationFailed
        );
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTION TESTS
// ═══════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════
// MULTI-TRANSFER AND NONCE TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn multiple_vault_transfers_with_incrementing_nonce() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let bob = 2;
        let public_key = mock_public_key();

        // Create vault
        assert_ok!(QuantumVault::create_vault(
            RuntimeOrigin::signed(alice),
            public_key
        ));

        // First transfer (nonce = 0): 10 + 10 premium = 20
        let sig1 = create_transfer_signature(alice, bob, 10, 0);
        assert_ok!(QuantumVault::vault_transfer(
            RuntimeOrigin::signed(alice),
            sig1,
            bob,
            10,
            None
        ));
        assert_eq!(VaultNonces::<Test>::get(alice), 1);

        // Second transfer with incremented nonce (nonce = 1): 20 + 10 premium = 30
        let sig2 = create_transfer_signature(alice, bob, 20, 1);
        assert_ok!(QuantumVault::vault_transfer(
            RuntimeOrigin::signed(alice),
            sig2,
            bob,
            20,
            None
        ));
        assert_eq!(VaultNonces::<Test>::get(alice), 2);

        // Third transfer (nonce = 2): 30 + 10 premium = 40
        let sig3 = create_transfer_signature(alice, bob, 30, 2);
        assert_ok!(QuantumVault::vault_transfer(
            RuntimeOrigin::signed(alice),
            sig3,
            bob,
            30,
            None
        ));
        assert_eq!(VaultNonces::<Test>::get(alice), 3);

        // Verify final balances
        // Alice: 998 (after vault) - 20 - 30 - 40 = 908
        // Bob: 500 + 10 + 20 + 30 = 560
        // Treasury: 1 + 2 (creation) + 30 (3x10 premium) = 33
        assert_eq!(Balances::free_balance(alice), 908);
        assert_eq!(Balances::free_balance(bob), 560);
        assert_eq!(Balances::free_balance(TREASURY), 33);
    });
}

#[test]
fn vault_creation_fee_goes_to_treasury() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let public_key = mock_public_key();

        // Get treasury balance before
        let treasury_before = Balances::free_balance(TREASURY);

        // Get total issuance before
        let total_before = Balances::total_issuance();

        // Create vault
        assert_ok!(QuantumVault::create_vault(
            RuntimeOrigin::signed(alice),
            public_key
        ));

        // Treasury should have received the fee
        assert_eq!(
            Balances::free_balance(TREASURY),
            treasury_before + CREATION_FEE
        );

        // Total issuance should remain the same (not burned!)
        let total_after = Balances::total_issuance();
        assert_eq!(total_before, total_after); // No tokens burned
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// CRYPTOGRAPHIC EDGE CASES
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn signature_verification_is_strict() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let bob = 2;
        let public_key = mock_public_key();

        // Create vault
        assert_ok!(QuantumVault::create_vault(
            RuntimeOrigin::signed(alice),
            public_key
        ));

        // Create valid signature
        let nonce = VaultNonces::<Test>::get(alice);
        let mut signature = create_transfer_signature(alice, bob, 100, nonce);

        // Corrupt a single byte in the signature
        signature[100] ^= 0xFF;

        // Even a single bit flip should fail verification
        assert_noop!(
            QuantumVault::vault_transfer(RuntimeOrigin::signed(alice), signature, bob, 100, None),
            Error::<Test>::SignatureVerificationFailed
        );
    });
}

#[test]
fn different_users_have_different_keypairs() {
    new_test_ext().execute_with(|| {
        // Verify that different users get different keypairs
        let alice_kp = alice_keypair();
        let bob_kp = bob_keypair();
        let charlie_kp = charlie_keypair();

        // Public keys should all be different
        assert_ne!(alice_kp.public, bob_kp.public);
        assert_ne!(alice_kp.public, charlie_kp.public);
        assert_ne!(bob_kp.public, charlie_kp.public);

        // Note: secret keys are not directly accessible in pqc_dilithium::Keypair
        // but they are guaranteed to be different if public keys are different
    });
}

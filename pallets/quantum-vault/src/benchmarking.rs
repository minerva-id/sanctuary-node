//! Benchmarking setup for pallet-quantum-vault
//!
//! These benchmarks measure the weight of the vault extrinsics:
//! - `create_vault`: Creating a new quantum vault with Dilithium public key
//! - `destroy_vault`: Destroying a vault with signature verification  
//! - `vault_transfer`: Transferring funds from a vault with signature verification

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use super::*;

#[allow(unused)]
use crate::Pallet as QuantumVault;
use frame_benchmarking::v2::*;
use frame_support::traits::{Currency, Get};
use frame_system::RawOrigin;

/// Generate a mock Dilithium2 public key (1312 bytes)
fn mock_public_key() -> Vec<u8> {
    vec![0u8; DILITHIUM_PUBLIC_KEY_SIZE]
}

/// Generate a mock Dilithium2 signature (2420 bytes)
fn mock_signature() -> Vec<u8> {
    vec![0u8; DILITHIUM_SIGNATURE_SIZE]
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_vault() {
        // Setup: Create a funded account
        let caller: T::AccountId = whitelisted_caller();
        let public_key = mock_public_key();

        // Fund the account with enough balance for the vault creation fee
        let deposit = T::VaultCreationFee::get() + T::Currency::minimum_balance() * 10u32.into();
        let _ = T::Currency::make_free_balance_be(&caller, deposit);

        #[extrinsic_call]
        create_vault(RawOrigin::Signed(caller.clone()), public_key.clone());

        // Verify vault was created
        assert!(Vaults::<T>::contains_key(&caller));
    }

    #[benchmark]
    fn destroy_vault() {
        // Setup: Create a vault first
        let caller: T::AccountId = whitelisted_caller();
        let public_key = mock_public_key();

        // Fund and create vault
        let deposit = T::VaultCreationFee::get() + T::Currency::minimum_balance() * 10u32.into();
        let _ = T::Currency::make_free_balance_be(&caller, deposit);

        let _ = Pallet::<T>::create_vault(RawOrigin::Signed(caller.clone()).into(), public_key);

        // Create mock signature for destroy
        let signature = mock_signature();

        #[extrinsic_call]
        destroy_vault(RawOrigin::Signed(caller.clone()), signature);

        // Note: In mock environment, signature verification is bypassed
        // The benchmark only measures the storage operations weight
    }

    #[benchmark]
    fn vault_transfer() {
        // Setup: Create a vault with sufficient funds
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let public_key = mock_public_key();

        // Fund caller generously
        let deposit = T::VaultCreationFee::get() + T::Currency::minimum_balance() * 100u32.into();
        let _ = T::Currency::make_free_balance_be(&caller, deposit);
        let _ = T::Currency::make_free_balance_be(&recipient, T::Currency::minimum_balance());

        // Create vault
        let _ = Pallet::<T>::create_vault(RawOrigin::Signed(caller.clone()).into(), public_key);

        // Create mock signature and transfer amount
        let signature = mock_signature();
        let amount: BalanceOf<T> = T::Currency::minimum_balance() * 10u32.into();

        #[extrinsic_call]
        vault_transfer(
            RawOrigin::Signed(caller.clone()),
            signature,
            recipient.clone(),
            amount,
        );

        // Note: In mock environment, signature verification is bypassed
    }

    impl_benchmark_test_suite!(QuantumVault, crate::mock::new_test_ext(), crate::mock::Test);
}

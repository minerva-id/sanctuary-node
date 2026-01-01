//! Quantum Vault Transfer Blocker
//!
//! This module provides a `TransactionExtension` that blocks standard `pallet_balances::transfer*`
//! calls from accounts that have been converted to Quantum Vaults.
//!
//! Vault accounts can only transfer funds using `pallet_quantum_vault::vault_transfer`
//! which requires a valid Dilithium signature.

use crate::{Runtime, RuntimeCall};
use codec::{Decode, DecodeWithMemTracking, Encode};
use frame_support::{pallet_prelude::TransactionSource, traits::OriginTrait};
use scale_info::TypeInfo;
use sp_runtime::{
    impl_tx_ext_default,
    traits::{DispatchInfoOf, TransactionExtension},
    transaction_validity::InvalidTransaction,
    Weight,
};

/// Custom error code for vault transfer block
const VAULT_TRANSFER_BLOCKED: u8 = 100;

/// Transaction extension that blocks standard transfers from vault accounts.
///
/// When an account is converted to a Quantum Vault, they can only transfer funds
/// using the `vault_transfer` extrinsic which requires PQC signature verification.
#[derive(Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, TypeInfo, Default)]
pub struct CheckVaultTransfer;

impl core::fmt::Debug for CheckVaultTransfer {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "CheckVaultTransfer")
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
        Ok(())
    }
}

impl CheckVaultTransfer {
    /// Create new `TransactionExtension` to check vault transfers.
    pub fn new() -> Self {
        Self
    }
}

impl TransactionExtension<RuntimeCall> for CheckVaultTransfer {
    const IDENTIFIER: &'static str = "CheckVaultTransfer";
    type Implicit = ();
    type Val = ();
    type Pre = ();

    fn weight(&self, _: &RuntimeCall) -> Weight {
        // Minimal weight - just a storage read check
        Weight::from_parts(1_000, 0)
    }

    fn validate(
        &self,
        origin: <Runtime as frame_system::Config>::RuntimeOrigin,
        call: &RuntimeCall,
        _info: &DispatchInfoOf<RuntimeCall>,
        _len: usize,
        _self_implicit: Self::Implicit,
        _inherited_implication: &impl Encode,
        _source: TransactionSource,
    ) -> sp_runtime::traits::ValidateResult<Self::Val, RuntimeCall> {
        // Check if sender is a vault account attempting a balance transfer
        if let Some(who) = origin.as_signer() {
            // Check if this is a balance transfer call
            let is_balance_transfer = matches!(
                call,
                RuntimeCall::Balances(
                    pallet_balances::Call::transfer_allow_death { .. }
                        | pallet_balances::Call::transfer_keep_alive { .. }
                        | pallet_balances::Call::transfer_all { .. }
                )
            );

            // If it's a balance transfer, check if the sender is a vault
            if is_balance_transfer && pallet_quantum_vault::Pallet::<Runtime>::is_vault(who) {
                log::warn!(
                    target: "quantum-vault",
                    "🚫 Blocked standard transfer from vault account. Use vault_transfer instead."
                );
                return Err(InvalidTransaction::Custom(VAULT_TRANSFER_BLOCKED).into());
            }
        }

        Ok((Default::default(), (), origin))
    }

    impl_tx_ext_default!(RuntimeCall; prepare);
}

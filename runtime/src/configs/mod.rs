// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <http://unlicense.org>

// Substrate and Polkadot dependencies
use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, VariantCountOf},
	weights::{
		constants::{RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
		IdentityFee, Weight,
	},
};
use frame_system::limits::{BlockLength, BlockWeights};
use pallet_transaction_payment::{ConstFeeMultiplier, FungibleAdapter, Multiplier};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::{traits::One, Perbill};
use sp_version::RuntimeVersion;

// Local module imports
use super::{
	AccountId, Aura, Balance, Balances, Block, BlockNumber, Hash, Nonce, PalletInfo, Runtime,
	RuntimeCall, RuntimeEvent, RuntimeFreezeReason, RuntimeHoldReason, RuntimeOrigin, RuntimeTask,
	System, Timestamp, EXISTENTIAL_DEPOSIT, SLOT_DURATION, VERSION,
};

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;

	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::with_sensible_defaults(
		Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
		NORMAL_DISPATCH_RATIO,
	);
	pub RuntimeBlockLength: BlockLength = BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;
}

/// The default types are being injected by [`derive_impl`](`frame_support::derive_impl`) from
/// [`SoloChainDefaultConfig`](`struct@frame_system::config_preludes::SolochainDefaultConfig`),
/// but overridden as needed.
#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
	/// The block type for the runtime.
	type Block = Block;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The type for storing how many extrinsics an account has signed.
	type Nonce = Nonce;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<32>;
	type AllowMultipleBlocksPerSlot = ConstBool<false>;
	type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Runtime>;
}

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;

	type WeightInfo = ();
	type MaxAuthorities = ConstU32<32>;
	type MaxNominators = ConstU32<0>;
	type MaxSetIdSessionEntries = ConstU64<0>;

	type KeyOwnerProof = sp_core::Void;
	type EquivocationReportSystem = ();
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	type WeightInfo = ();
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxFreezes = VariantCountOf<RuntimeFreezeReason>;
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type DoneSlashHandler = ();
}

parameter_types! {
	pub FeeMultiplier: Multiplier = Multiplier::one();
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = FungibleAdapter<Balances, ()>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = IdentityFee<Balance>;
	type LengthToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
	type WeightInfo = pallet_transaction_payment::weights::SubstrateWeight<Runtime>;
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

/// Configure the pallet-template in pallets/template.
impl pallet_template::Config for Runtime {
	type WeightInfo = pallet_template::weights::SubstrateWeight<Runtime>;
}

// ═══════════════════════════════════════════════════════════════════════════
// AUTHORSHIP PALLET - Required for finding block author
// ═══════════════════════════════════════════════════════════════════════════

/// Converts Aura AuthorityId to AccountId for authorship tracking
pub struct AuraAccountAdapter;
impl frame_support::traits::FindAuthor<AccountId> for AuraAccountAdapter {
	fn find_author<'a, I>(digests: I) -> Option<AccountId>
	where
		I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
	{
		pallet_aura::AuraAuthorId::<Runtime>::find_author(digests)
			.map(|aura_id| {
				// AuraId is sr25519::Public which wraps CryptoBytes<32>
				// We extract the 32-byte public key and convert to AccountId
				use sp_core::crypto::ByteArray;
				let raw: &[u8] = aura_id.as_slice();
				let mut bytes = [0u8; 32];
				bytes.copy_from_slice(&raw[..32]);
				AccountId::from(bytes)
			})
	}
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = AuraAccountAdapter;
	type EventHandler = ();
}

// ═══════════════════════════════════════════════════════════════════════════
// EMISSION PALLET CONFIGURATION (v2.0 - Stateless)
// ═══════════════════════════════════════════════════════════════════════════
//
// Simple pre-computed sigmoid emission curve.
// No storage, no complex ASM - just lookup and mint.
//
// Emission is distributed to block author (validator) on each block.
// ═══════════════════════════════════════════════════════════════════════════

impl pallet_emission::Config for Runtime {
	type Currency = Balances;
	type FindAuthor = AuraAccountAdapter;
	type WeightInfo = ();
}

// ═══════════════════════════════════════════════════════════════════════════
// EVM CONFIGURATION (Frontier Integration)
// ═══════════════════════════════════════════════════════════════════════════
//
// Chain ID: 7777 (Tesserax Network)
// Features: Full EVM compatibility, EIP-1559 base fee
// ═══════════════════════════════════════════════════════════════════════════

use pallet_evm::{
	AddressMapping, IsPrecompileResult,
	Precompile, PrecompileHandle, PrecompileResult, PrecompileSet,
};
use sp_core::{H160, U256};
use core::marker::PhantomData;
use sp_runtime::Permill;

/// Tesserax Chain ID: 7777
pub const CHAIN_ID: u64 = 7777;

/// Block gas limit
pub const BLOCK_GAS_LIMIT: u64 = 75_000_000;

/// Weight per millisecond (approximation)
pub const WEIGHT_MILLISECS_PER_BLOCK: u64 = 2000 * WEIGHT_REF_TIME_PER_SECOND / 1000;

parameter_types! {
	/// Block gas limit
	pub BlockGasLimit: U256 = U256::from(BLOCK_GAS_LIMIT);
	/// Gas/PoV ratio
	pub const GasLimitPovSizeRatio: u64 = 4;
	/// Storage growth ratio
	pub const GasLimitStorageGrowthRatio: u64 = 4;
	/// Weight per gas unit
	pub WeightPerGas: Weight = Weight::from_parts(
		fp_evm::weight_per_gas(BLOCK_GAS_LIMIT, NORMAL_DISPATCH_RATIO, WEIGHT_MILLISECS_PER_BLOCK),
		0
	);
	/// Default base fee per gas (1 Gwei in wei)
	pub DefaultBaseFeePerGas: U256 = U256::from(1_000_000_000);
	/// Default elasticity (12.5% as per EIP-1559)
	pub DefaultElasticity: Permill = Permill::from_parts(125_000);
	/// Quick clear limit
	pub const SuicideQuickClearLimit: u32 = 0;
	/// Post log content: block and txn hashes
	pub const PostBlockAndTxnHashes: pallet_ethereum::PostLogContent = pallet_ethereum::PostLogContent::BlockAndTxnHashes;
	/// Min gas price bound divisor
	pub BoundDivision: U256 = U256::from(1024);
}

/// Custom address mapping: H160 -> AccountId32
/// Pads H160 (20 bytes) with zeros to create AccountId32 (32 bytes)
pub struct HashedAddressMapping;
impl AddressMapping<AccountId> for HashedAddressMapping {
	fn into_account_id(address: H160) -> AccountId {
		let mut data = [0u8; 32];
		data[0..20].copy_from_slice(&address[..]);
		AccountId::from(data)
	}
}

/// Get account from H160 address
pub struct FindAuthorTruncated<F>(PhantomData<F>);
impl<F: frame_support::traits::FindAuthor<AccountId>> frame_support::traits::FindAuthor<H160>
	for FindAuthorTruncated<F>
{
	fn find_author<'a, I>(digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
	{
		F::find_author(digests).map(|account| {
			let bytes: [u8; 32] = account.into();
			let mut h160_bytes = [0u8; 20];
			h160_bytes.copy_from_slice(&bytes[0..20]);
			H160::from(h160_bytes)
		})
	}
}

/// Custom EnsureAddressOrigin that allows any signed account to interact with EVM
/// Maps H160 address to AccountId32 and verifies the signer matches
pub struct EnsureAddressTruncated;

impl<OuterOrigin> pallet_evm::EnsureAddressOrigin<OuterOrigin> for EnsureAddressTruncated
where
	OuterOrigin: Into<Result<frame_system::RawOrigin<AccountId>, OuterOrigin>> + Clone,
{
	type Success = AccountId;

	fn try_address_origin(address: &H160, origin: OuterOrigin) -> Result<AccountId, OuterOrigin> {
		origin.clone().into().and_then(|o| match o {
			frame_system::RawOrigin::Signed(who) => {
				// Convert AccountId to H160 and compare
				let who_bytes: [u8; 32] = who.clone().into();
				let mut who_h160 = [0u8; 20];
				who_h160.copy_from_slice(&who_bytes[0..20]);
				if H160::from(who_h160) == *address {
					Ok(who)
				} else {
					Err(origin)
				}
			}
			_ => Err(origin),
		})
	}
}

/// Standard Ethereum precompiles
pub struct TesseraxPrecompiles<R>(PhantomData<R>);

impl<R> TesseraxPrecompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(PhantomData)
	}

	pub fn used_addresses() -> [H160; 5] {
		[
			hash(1),  // ECRecover
			hash(2),  // Sha256
			hash(3),  // Ripemd160
			hash(4),  // Identity
			hash(5),  // Modexp
		]
	}
}

impl<R> Default for TesseraxPrecompiles<R>
where
	R: pallet_evm::Config,
{
	fn default() -> Self {
		Self::new()
	}
}

/// Convert a number to H160 address
fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

impl<R> PrecompileSet for TesseraxPrecompiles<R>
where
	R: pallet_evm::Config,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			// ECRecover
			a if a == hash(1) => Some(pallet_evm_precompile_simple::ECRecover::execute(handle)),
			// Sha256
			a if a == hash(2) => Some(pallet_evm_precompile_simple::Sha256::execute(handle)),
			// Ripemd160
			a if a == hash(3) => Some(pallet_evm_precompile_simple::Ripemd160::execute(handle)),
			// Identity
			a if a == hash(4) => Some(pallet_evm_precompile_simple::Identity::execute(handle)),
			// Modexp
			a if a == hash(5) => Some(pallet_evm_precompile_modexp::Modexp::execute(handle)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
		IsPrecompileResult::Answer {
			is_precompile: Self::used_addresses().contains(&address),
			extra_cost: 0,
		}
	}
}

parameter_types! {
	pub PrecompilesValue: TesseraxPrecompiles<Runtime> = TesseraxPrecompiles::<Runtime>::new();
}

/// Configure EVM Chain ID
impl pallet_evm_chain_id::Config for Runtime {}

parameter_types! {
	pub const ChainId: u64 = CHAIN_ID;
}

/// Configure EVM
impl pallet_evm::Config for Runtime {
	type AccountProvider = pallet_evm::FrameSystemAccountProvider<Self>;
	type FeeCalculator = pallet_base_fee::Pallet<Self>;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressTruncated;
	type CreateOriginFilter = ();
	type CreateInnerOriginFilter = ();
	type WithdrawOrigin = EnsureAddressTruncated;
	type AddressMapping = HashedAddressMapping;
	type Currency = Balances;
	type PrecompilesType = TesseraxPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = ChainId;
	type BlockGasLimit = BlockGasLimit;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction = ();
	type OnCreate = ();
	type FindAuthor = FindAuthorTruncated<AuraAccountAdapter>;
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type GasLimitStorageGrowthRatio = GasLimitStorageGrowthRatio;
	type Timestamp = Timestamp;
	type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
}

/// Configure Ethereum compatibility layer
impl pallet_ethereum::Config for Runtime {
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self::Version>;
	type PostLogContent = PostBlockAndTxnHashes;
	type ExtraDataLength = ConstU32<30>;
}

/// Base fee threshold implementation
pub struct BaseFeeThreshold;
impl pallet_base_fee::BaseFeeThreshold for BaseFeeThreshold {
	fn lower() -> Permill {
		Permill::zero()
	}
	fn ideal() -> Permill {
		Permill::from_parts(500_000)
	}
	fn upper() -> Permill {
		Permill::from_parts(1_000_000)
	}
}

/// Configure Base Fee (EIP-1559)
impl pallet_base_fee::Config for Runtime {
	type Threshold = BaseFeeThreshold;
	type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
	type DefaultElasticity = DefaultElasticity;
}

/// Configure Dynamic Fee adjustment
impl pallet_dynamic_fee::Config for Runtime {
	type MinGasPriceBoundDivisor = BoundDivision;
}

// ═══════════════════════════════════════════════════════════════════════════
// QUANTUM VAULT CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════
//
// Post-Quantum Cryptographic Cold Storage
// Uses CRYSTALS-Dilithium Level 2 (NIST PQC Standard)
//
// Why Level 2?
// - NIST's recommended baseline for post-quantum security
// - AES-128 equivalent (50+ years secure)
// - 32% smaller keys than Level 3 = lower storage costs
// - 26% smaller signatures = lower transaction costs
// - Faster verification = better throughput
//
// Features:
// - 10 TSRX fee to create vault (spam prevention)
// - 100x fee multiplier for vault transfers (security premium)
// - Standard transfers blocked for vault accounts
// ═══════════════════════════════════════════════════════════════════════════

use super::TSRX;

parameter_types! {
	/// Fee to create a quantum vault: 10 TSRX
	pub const VaultCreationFee: Balance = 10 * TSRX;
	/// Fee multiplier for vault transfers: 100x normal
	pub const VaultTransferFeeMultiplier: u32 = 100;
	/// Maximum public key size: Dilithium2 = 1312 bytes
	pub const MaxPublicKeySize: u32 = 1312;
	/// Maximum signature size: Dilithium2 = 2420 bytes
	pub const MaxSignatureSize: u32 = 2420;
}

impl pallet_quantum_vault::Config for Runtime {
	type Currency = Balances;
	type WeightInfo = pallet_quantum_vault::weights::SubstrateWeight<Self>;
	type VaultCreationFee = VaultCreationFee;
	type VaultTransferFeeMultiplier = VaultTransferFeeMultiplier;
	type MaxPublicKeySize = MaxPublicKeySize;
	type MaxSignatureSize = MaxSignatureSize;
}

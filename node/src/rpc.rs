//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.
//!
//! Includes basic Ethereum-compatible JSON-RPC methods.

#![warn(missing_docs)]

use std::sync::Arc;

use jsonrpsee::RpcModule;
use sc_transaction_pool_api::TransactionPool;
use tesserax_runtime::{opaque::Block, AccountId, Balance, Nonce};
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// Tesserax Chain ID: 13817 (derived from floor(π × e × φ × 10^6) = 13,817,422)
pub const CHAIN_ID: u64 = 13817;

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool } = deps;

	// Substrate RPC
	module.merge(System::new(client.clone(), pool).into_rpc())?;
	module.merge(TransactionPayment::new(client).into_rpc())?;

	// ═══════════════════════════════════════════════════════════════════════════
	// ETHEREUM RPC ENDPOINTS
	// ═══════════════════════════════════════════════════════════════════════════
	// Basic eth_* and net_* methods for Metamask detection
	// Full eth_* RPC requires Frontier backend setup (Phase 2.3)

	module.register_method("eth_chainId", |_, _, _| {
		Ok::<String, jsonrpsee::types::ErrorObjectOwned>(format!("0x{:x}", CHAIN_ID))
	})?;

	module.register_method("net_version", |_, _, _| {
		Ok::<String, jsonrpsee::types::ErrorObjectOwned>(CHAIN_ID.to_string())
	})?;

	module.register_method("web3_clientVersion", |_, _, _| {
		Ok::<String, jsonrpsee::types::ErrorObjectOwned>("Tesserax/v0.1.0".to_string())
	})?;

	module.register_method("eth_syncing", |_, _, _| {
		Ok::<bool, jsonrpsee::types::ErrorObjectOwned>(false)
	})?;

	module.register_method("eth_gasPrice", |_, _, _| {
		// 1 Gwei default gas price
		Ok::<String, jsonrpsee::types::ErrorObjectOwned>("0x3b9aca00".to_string())
	})?;

	module.register_method("net_listening", |_, _, _| {
		Ok::<bool, jsonrpsee::types::ErrorObjectOwned>(true)
	})?;

	module.register_method("net_peerCount", |_, _, _| {
		Ok::<String, jsonrpsee::types::ErrorObjectOwned>("0x0".to_string())
	})?;

	Ok(module)
}

//! Ethereum-compatible RPC and backend configuration for Tesserax.
//!
//! This module provides full Ethereum JSON-RPC compatibility including:
//! - eth_* methods (getBalance, blockNumber, sendRawTransaction, etc.)
//! - net_* methods (version, peerCount, listening)
//! - web3_* methods (clientVersion, sha3)

use std::{collections::BTreeMap, sync::Arc, time::Duration};

use futures::StreamExt;
use jsonrpsee::RpcModule;

// Substrate
use sc_client_api::{
	backend::{Backend, StorageProvider},
	client::BlockchainEvents,
	AuxStore, UsageProvider,
};
use sc_network::service::traits::NetworkService;
use sc_network_sync::SyncingService;
use sc_rpc::SubscriptionTaskExecutor;
use sc_service::{Configuration, TaskManager};
use sc_transaction_pool_api::TransactionPool;
use sp_api::{CallApiAt, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder as BlockBuilderApi;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus_aura::{sr25519::AuthorityId as AuraId, AuraApi};
use sp_core::H256;
use sp_inherents::CreateInherentDataProviders;
use sp_runtime::traits::Block as BlockT;

// Frontier
pub use fc_rpc::{EthBlockDataCacheTask, EthConfig};
pub use fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit, FilterPool};
use fc_storage::StorageOverride;
use fp_rpc::{ConvertTransactionRuntimeApi, EthereumRuntimeRPCApi};

use tesserax_runtime::opaque::Block;

/// Frontier backend type alias
pub type FrontierBackend<C> = fc_db::kv::Backend<Block, C>;

/// Extra dependencies for Ethereum compatibility.
pub struct EthDeps<B: BlockT, C, P, CT, CIDP> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Graph pool instance (same as pool for our use case)
	pub graph: Arc<P>,
	/// Ethereum transaction converter.
	pub converter: Option<CT>,
	/// The Node authority flag
	pub is_authority: bool,
	/// Whether to enable dev signer
	pub enable_dev_signer: bool,
	/// Network service
	pub network: Arc<dyn NetworkService>,
	/// Chain syncing service
	pub sync: Arc<SyncingService<B>>,
	/// Frontier Backend.
	pub frontier_backend: Arc<dyn fc_api::Backend<B>>,
	/// Ethereum data access overrides.
	pub storage_override: Arc<dyn StorageOverride<B>>,
	/// Cache for Ethereum block data.
	pub block_data_cache: Arc<EthBlockDataCacheTask<B>>,
	/// EthFilterApi pool.
	pub filter_pool: Option<FilterPool>,
	/// Maximum number of logs in a query.
	pub max_past_logs: u32,
	/// Fee history cache.
	pub fee_history_cache: FeeHistoryCache,
	/// Maximum fee history cache size.
	pub fee_history_cache_limit: FeeHistoryCacheLimit,
	/// Maximum allowed gas limit for eth_call/eth_estimateGas.
	pub execute_gas_limit_multiplier: u64,
	/// Mandated parent hashes for a given block hash.
	pub forced_parent_hashes: Option<BTreeMap<H256, H256>>,
	/// Something that can create the inherent data providers for pending state
	pub pending_create_inherent_data_providers: CIDP,
}

/// Default EthConfig implementation for Tesserax
pub struct TesseraxEthConfig<C, BE>(std::marker::PhantomData<(C, BE)>);

impl<B, C, BE> EthConfig<B, C> for TesseraxEthConfig<C, BE>
where
	B: BlockT,
	C: StorageProvider<B, BE> + Sync + Send + 'static,
	BE: Backend<B> + 'static,
{
	type EstimateGasAdapter = ();
	type RuntimeStorageOverride =
		fc_rpc::frontier_backend_client::SystemAccountId20StorageOverride<B, C, BE>;
}

/// Instantiate Ethereum-compatible RPC extensions.
pub fn create_eth<B, C, BE, P, CT, CIDP, EC>(
	mut io: RpcModule<()>,
	deps: EthDeps<B, C, P, CT, CIDP>,
	subscription_task_executor: SubscriptionTaskExecutor,
	pubsub_notification_sinks: Arc<
		fc_mapping_sync::EthereumBlockNotificationSinks<
			fc_mapping_sync::EthereumBlockNotification<B>,
		>,
	>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	B: BlockT<Hash = H256>,
	C: CallApiAt<B> + ProvideRuntimeApi<B>,
	C::Api: AuraApi<B, AuraId>
		+ BlockBuilderApi<B>
		+ ConvertTransactionRuntimeApi<B>
		+ EthereumRuntimeRPCApi<B>,
	C: HeaderBackend<B> + HeaderMetadata<B, Error = BlockChainError>,
	C: BlockchainEvents<B> + AuxStore + UsageProvider<B> + StorageProvider<B, BE> + 'static,
	BE: Backend<B> + 'static,
	P: TransactionPool<Block = B, Hash = B::Hash> + 'static,
	CT: fp_rpc::ConvertTransaction<<B as BlockT>::Extrinsic> + Send + Sync + 'static,
	CIDP: CreateInherentDataProviders<B, ()> + Send + 'static,
	EC: EthConfig<B, C>,
{
	use fc_rpc::{
		pending::AuraConsensusDataProvider, Eth, EthApiServer, EthDevSigner,
		EthFilter, EthFilterApiServer, EthPubSub, EthPubSubApiServer, EthSigner, Net, NetApiServer,
		Web3, Web3ApiServer,
	};

	let EthDeps {
		client,
		pool,
		graph,
		converter,
		is_authority,
		enable_dev_signer,
		network,
		sync,
		frontier_backend,
		storage_override,
		block_data_cache,
		filter_pool,
		max_past_logs,
		fee_history_cache,
		fee_history_cache_limit,
		execute_gas_limit_multiplier,
		forced_parent_hashes,
		pending_create_inherent_data_providers,
	} = deps;

	let mut signers = Vec::new();
	if enable_dev_signer {
		signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
	}

	io.merge(
		Eth::<B, C, P, CT, BE, CIDP, EC>::new(
			client.clone(),
			pool.clone(),
			graph.clone(),
			converter,
			sync.clone(),
			signers,
			storage_override.clone(),
			frontier_backend.clone(),
			is_authority,
			block_data_cache.clone(),
			fee_history_cache,
			fee_history_cache_limit,
			execute_gas_limit_multiplier,
			forced_parent_hashes,
			pending_create_inherent_data_providers,
			Some(Box::new(AuraConsensusDataProvider::new(client.clone()))),
		)
		.replace_config::<EC>()
		.into_rpc(),
	)?;

	if let Some(filter_pool) = filter_pool {
		io.merge(
			EthFilter::new(
				client.clone(),
				frontier_backend.clone(),
				graph.clone(),
				filter_pool,
				500_usize, // max stored filters
				max_past_logs,
				block_data_cache.clone(),
			)
			.into_rpc(),
		)?;
	}

	io.merge(
		EthPubSub::new(
			pool,
			client.clone(),
			sync,
			subscription_task_executor,
			storage_override,
			pubsub_notification_sinks,
		)
		.into_rpc(),
	)?;

	io.merge(
		Net::new(
			client.clone(),
			network,
			true, // Format peer_count as hex
		)
		.into_rpc(),
	)?;

	io.merge(Web3::new(client).into_rpc())?;

	Ok(io)
}

/// Create the Frontier backend (key-value store for Ethereum block mapping)
pub fn new_frontier_backend<C>(
	client: Arc<C>,
	config: &Configuration,
) -> Result<Arc<FrontierBackend<C>>, String>
where
	C: HeaderBackend<Block> + Send + Sync + 'static,
{
	let db_config_dir = config.database.path()
		.ok_or("Database path must exist for Frontier backend")?;
	
	Ok(Arc::new(fc_db::kv::Backend::open(
		client,
		&config.database,
		db_config_dir,
	)?))
}

/// Spawn the background tasks required for Frontier (block mapping sync, fee history)
pub fn spawn_frontier_tasks<B, BE, C>(
	task_manager: &TaskManager,
	client: Arc<C>,
	backend: Arc<BE>,
	frontier_backend: Arc<fc_db::kv::Backend<B, C>>,
	filter_pool: Option<FilterPool>,
	storage_override: Arc<dyn StorageOverride<B>>,
	fee_history_cache: FeeHistoryCache,
	fee_history_cache_limit: FeeHistoryCacheLimit,
	sync_service: Arc<SyncingService<B>>,
	pubsub_notification_sinks: Arc<
		fc_mapping_sync::EthereumBlockNotificationSinks<
			fc_mapping_sync::EthereumBlockNotification<B>,
		>,
	>,
) where
	B: BlockT<Hash = H256>,
	BE: Backend<B> + 'static,
	C: ProvideRuntimeApi<B> + BlockchainEvents<B> + HeaderBackend<B> + StorageProvider<B, BE>,
	C: Send + Sync + 'static,
	C::Api: EthereumRuntimeRPCApi<B>,
{
	// Spawn block mapping sync task
	task_manager.spawn_essential_handle().spawn(
		"frontier-mapping-sync-worker",
		Some("frontier"),
		fc_mapping_sync::kv::MappingSyncWorker::new(
			client.import_notification_stream(),
			Duration::new(6, 0),
			client.clone(),
			backend,
			storage_override.clone(),
			frontier_backend,
			3,
			0u32.into(),
			fc_mapping_sync::SyncStrategy::Normal,
			sync_service,
			pubsub_notification_sinks,
		)
		.for_each(|()| futures::future::ready(())),
	);

	// Spawn EthFilterApi maintenance task
	if let Some(filter_pool) = filter_pool {
		const FILTER_RETAIN_THRESHOLD: u64 = 100;
		task_manager.spawn_essential_handle().spawn(
			"frontier-filter-pool",
			Some("frontier"),
			fc_rpc::EthTask::filter_pool_task(
				client.clone(),
				filter_pool,
				FILTER_RETAIN_THRESHOLD,
			),
		);
	}

	// Spawn fee history task
	task_manager.spawn_essential_handle().spawn(
		"frontier-fee-history",
		Some("frontier"),
		fc_rpc::EthTask::fee_history_task(
			client,
			storage_override,
			fee_history_cache,
			fee_history_cache_limit,
		),
	);
}

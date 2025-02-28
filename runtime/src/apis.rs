//! Runtime API Implementation

use polkadot_sdk::*;

use scale_codec::Encode;

use frame_support::{traits::KeyOwnerProofSystem, weights::Weight};
// Can't use `FungibleAdapter` here until Treasury pallet migrates to fungibles
// <https://github.com/paritytech/polkadot-sdk/issues/226>
#[allow(deprecated)]
pub use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
use sp_api::impl_runtime_apis;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::crypto::KeyTypeId;
use sp_core::OpaqueMetadata;
use sp_inherents::{CheckInherentsResult, InherentData};
use sp_metadata_ir::RuntimeApiMetadataIR;
use sp_runtime::{
	traits::{Block as BlockT, NumberFor},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub use time_primitives::{MembersInterface, NetworksInterface};

#[cfg(feature = "testnet")]
use time_primitives::{
	BatchId, BlockNumber, ChainName, ChainNetwork, Commitment, ErrorMsg, Gateway, GatewayMessage,
	MemberStatus, NetworkId, PeerId, PublicKey, ShardId, ShardStatus, Task, TaskId,
};
// Local module imports
use super::{
	AccountId, AuthorityDiscovery, Babe, Balance, Block, EpochDuration, Executive, Grandpa,
	Historical, InherentDataExt, Nonce, Runtime, RuntimeCall, SessionKeys, System,
	TransactionPayment, BABE_GENESIS_EPOCH_CONFIG, VERSION,
};
#[cfg(feature = "genesis-builder")]
use crate::RuntimeGenesisConfig;
#[cfg(feature = "testnet")]
use crate::{Members, Networks, Shards, Staking, Tasks};

// HASHI Bridge
use crate::configs::bridge::NetworkId as BridgeNetworkId;
use crate::EthBridge;
use eth_bridge::{
	common::{AssetId as BridgeAssetId, BalancePrecision as BridgeBalancePrecision},
	offchain::SignatureParams as BridgeSignatureParams,
	requests::{
		AssetKind as BridgeAssetKind, OffchainRequest, OutgoingRequestEncoded,
		RequestStatus as BridgeRequestStatus,
	},
};
use sp_runtime::DispatchError;

// Original Author: ntn-x2 @ KILTprotocol
// Workaround for runtime API impls not exposed in metadata if implemented in a
// different file than the runtime's `lib.rs`. Related issue (subxt) -> https://github.com/paritytech/subxt/issues/1873.
pub(crate) trait _InternalImplRuntimeApis {
	fn runtime_metadata(&self) -> Vec<RuntimeApiMetadataIR>;
}
impl<T> _InternalImplRuntimeApis for T
where
	T: InternalImplRuntimeApis,
{
	#[inline(always)]
	fn runtime_metadata(&self) -> Vec<RuntimeApiMetadataIR> {
		<T as InternalImplRuntimeApis>::runtime_metadata(self)
	}
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> sp_std::vec::Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl sp_consensus_babe::BabeApi<Block> for Runtime {
		fn configuration() -> sp_consensus_babe::BabeConfiguration {
			let epoch_config = Babe::epoch_config().unwrap_or(BABE_GENESIS_EPOCH_CONFIG);
			sp_consensus_babe::BabeConfiguration {
				slot_duration: Babe::slot_duration(),
				epoch_length: EpochDuration::get(),
				c: epoch_config.c,
				authorities: Babe::authorities().to_vec(),
				randomness: Babe::randomness(),
				allowed_slots: epoch_config.allowed_slots,
			}
		}

		fn current_epoch_start() -> sp_consensus_babe::Slot {
			Babe::current_epoch_start()
		}

		fn current_epoch() -> sp_consensus_babe::Epoch {
			Babe::current_epoch()
		}

		fn next_epoch() -> sp_consensus_babe::Epoch {
			Babe::next_epoch()
		}

		fn generate_key_ownership_proof(
			_slot: sp_consensus_babe::Slot,
			authority_id: sp_consensus_babe::AuthorityId,
		) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
			Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
			key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			//let key_owner_proof = key_owner_proof.decode()?;

			Babe::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof.decode()?,
			)
		}
	}

	impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> sp_consensus_grandpa::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: sp_consensus_grandpa::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Grandpa::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}

		fn generate_key_ownership_proof(
			_set_id: sp_consensus_grandpa::SetId,
			authority_id: GrandpaId,
		) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
			Historical::prove((sp_consensus_grandpa::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(sp_consensus_grandpa::OpaqueKeyOwnershipProof::new)
		}
	}

	#[cfg(feature = "testnet")]
	impl pallet_staking_runtime_api::StakingApi<Block, Balance, AccountId> for Runtime {
		fn nominations_quota(balance: Balance) -> u32 {
			Staking::api_nominations_quota(balance)
		}

		fn eras_stakers_page_count(era: sp_staking::EraIndex, account: AccountId) -> sp_staking::Page {
			Staking::api_eras_stakers_page_count(era, account)
		}

		fn pending_rewards(era: sp_staking::EraIndex, account: AccountId) -> bool {
			Staking::api_pending_rewards(era, account)
		}
	}

	impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
		fn authorities() -> Vec<AuthorityDiscoveryId> {
			AuthorityDiscovery::authorities()
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
		fn account_nonce(account: AccountId) -> Nonce {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}

		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}

		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}

		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(call: RuntimeCall, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(call: RuntimeCall, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	#[cfg(feature = "testnet")]
	impl time_primitives::MembersApi<Block> for Runtime {
		fn get_member_peer_id(account: &AccountId) -> Option<PeerId> {
			Members::member_peer_id(account)
		}

		fn get_heartbeat_timeout() -> BlockNumber {
			Members::get_heartbeat_timeout()
		}

		fn get_min_stake() -> Balance {
			Members::get_min_stake()
		}
	}

	#[cfg(feature = "testnet")]
	impl time_primitives::NetworksApi<Block> for Runtime {
		fn get_network(network_id: NetworkId) -> Option<(ChainName, ChainNetwork)> {
			Networks::get_network(network_id)
		}

		fn get_gateway(network: NetworkId) -> Option<Gateway> {
			Networks::gateway(network)
		}
	}

	#[cfg(feature = "testnet")]
	impl time_primitives::ShardsApi<Block> for Runtime {
		fn get_shards(account: &AccountId) -> Vec<ShardId> {
			Shards::get_shards(account)
		}

		fn get_shard_members(shard_id: ShardId) -> Vec<(AccountId, MemberStatus)> {
			Shards::get_shard_members(shard_id)
		}

		fn get_shard_threshold(shard_id: ShardId) -> u16 {
			Shards::get_shard_threshold(shard_id)
		}

		fn get_shard_status(shard_id: ShardId) -> ShardStatus {
			Shards::get_shard_status(shard_id)
		}

		fn get_shard_commitment(shard_id: ShardId) -> Option<Commitment> {
			Shards::get_shard_commitment(shard_id)
		}
	}

	#[cfg(feature = "testnet")]
	impl time_primitives::TasksApi<Block> for Runtime {
		fn get_shard_tasks(shard_id: ShardId) -> Vec<TaskId> {
			Tasks::get_shard_tasks(shard_id)
		}

		fn get_task(task_id: TaskId) -> Option<Task>{
			Tasks::get_task(task_id)
		}

		fn get_task_submitter(task_id: TaskId) -> Option<PublicKey> {
			Tasks::get_task_submitter(task_id)
		}

		fn get_task_result(task_id: TaskId) -> Option<Result<(), ErrorMsg>>{
			Tasks::get_task_result(task_id)
		}

		fn get_task_shard(task_id: TaskId) -> Option<ShardId>{
			Tasks::get_task_shard(task_id)
		}

		fn get_batch_message(batch_id: BatchId) -> Option<GatewayMessage> {
			Tasks::get_batch_message(batch_id)
		}

		fn get_failed_tasks() -> Vec<TaskId> {
			Tasks::get_failed_tasks()
		}
	}

	#[cfg(feature = "testnet")]
	impl time_primitives::SubmitTransactionApi<Block> for Runtime {
		fn submit_transaction(encoded_transaction: Vec<u8>) -> Result<(), ()> {
			sp_io::offchain::submit_transaction(encoded_transaction)
		}
	}

	impl
		eth_bridge_runtime_api::EthBridgeRuntimeApi<
			Block,
			sp_core::H256,
			BridgeSignatureParams,
			AccountId,
			BridgeAssetKind,
			BridgeAssetId,
			sp_core::H160,
			OffchainRequest<Runtime>,
			BridgeRequestStatus,
			OutgoingRequestEncoded,
			BridgeNetworkId,
			BridgeBalancePrecision,
		> for Runtime
	{
		fn get_requests(
			hashes: Vec<sp_core::H256>,
			network_id: Option<BridgeNetworkId>,
			redirect_finished_load_requests: bool,
		) -> Result<
			Vec<(
				OffchainRequest<Runtime>,
				BridgeRequestStatus,
			)>,
			DispatchError,
		> {
			EthBridge::get_requests(&hashes, network_id, redirect_finished_load_requests)
		}

		fn get_approved_requests(
			hashes: Vec<sp_core::H256>,
			network_id: Option<BridgeNetworkId>
		) -> Result<
			Vec<(
				OutgoingRequestEncoded,
				Vec<BridgeSignatureParams>,
			)>,
			DispatchError,
		> {
			EthBridge::get_approved_requests(&hashes, network_id)
		}

		fn get_approvals(
			hashes: Vec<sp_core::H256>,
			network_id: Option<BridgeNetworkId>
		) -> Result<Vec<Vec<BridgeSignatureParams>>, DispatchError> {
			EthBridge::get_approvals(&hashes, network_id)
		}

		fn get_account_requests(account_id: AccountId, status_filter: Option<BridgeRequestStatus>) -> Result<Vec<(BridgeNetworkId, sp_core::H256)>, DispatchError> {
			EthBridge::get_account_requests(&account_id, status_filter)
		}

		fn get_registered_assets(
			network_id: Option<BridgeNetworkId>
		) -> Result<Vec<(
				BridgeAssetKind,
				(BridgeAssetId, BridgeBalancePrecision),
				Option<(sp_core::H160, BridgeBalancePrecision)
		>)>, DispatchError> {
			EthBridge::get_registered_assets(network_id)
		}
	}

	/// Optional runtime interfaces controlled by feature flags

	/// - __genesis-builder__: support generation of custom genesis
	#[cfg(feature = "genesis-builder")]
	impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {

		fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
			use frame_support::genesis_builder_helper::build_state;
			build_state::<RuntimeGenesisConfig>(config)
		}

		fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
			use frame_support::genesis_builder_helper::get_preset;
			get_preset::<RuntimeGenesisConfig>(id, |_| None)
		}

		fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
			vec![]
		}
	}

	/// - __runtime-benchmarks__: support runtime benchmarking
	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {

		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;

			// Trying to add benchmarks directly to the Session Pallet caused cyclic dependency
			// issues. To get around that, we separated the Session benchmarks into its own crate,
			// which is why we need these two lines below.
			use pallet_session_benchmarking::Pallet as SessionBench;
			use pallet_nomination_pools_benchmarking::Pallet as NominationPoolsBench;
			use pallet_offences_benchmarking::Pallet as OffencesBench;
			use pallet_election_provider_support_benchmarking::Pallet as EPSBench;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			// Import substrate macros created by macros (and all pallets by effects)
			use crate::*;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);
			let storage_info = AllPalletsWithSystem::storage_info();
			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch};

			// Trying to add benchmarks directly to the Session Pallet caused cyclic dependency
			// issues. To get around that, we separated the Session benchmarks into its own crate,
			// which is why we need these two lines below.
			use pallet_session_benchmarking::Pallet as SessionBench;
			use pallet_nomination_pools_benchmarking::Pallet as NominationPoolsBench;
			use pallet_offences_benchmarking::Pallet as OffencesBench;
			use pallet_election_provider_support_benchmarking::Pallet as EPSBench;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			impl pallet_session_benchmarking::Config for Runtime {}
			impl pallet_nomination_pools_benchmarking::Config for Runtime {}
			impl pallet_offences_benchmarking::Config for Runtime {}
			impl pallet_election_provider_support_benchmarking::Config for Runtime {}
			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			use frame_support::traits::{TrackedStorageKey, WhitelistedStorageKeys};
			let mut whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

			// Import substrate macros created by macros (and all pallets by effects)
			use crate::*;

			// Treasury Account
			// TODO: this is manual for now, someday we might be able to use a
			// macro for this particular key
			#[cfg(feature = "testnet")]
			whitelist.push(frame_system::Account::<Runtime>::hashed_key_for(Treasury::account_id()).to_vec().into());

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);
			Ok(batches)
		}
	}

	/// - __try-runtime__: support try runtime testing
	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade(checks).unwrap();
			(weight, crate::RuntimeBlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			signature_check: bool,
			select: frame_try_runtime::TryStateSelect
		) -> Weight {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
		}
	}

}

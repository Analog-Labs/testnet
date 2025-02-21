use alloy::{
	primitives::{address, Address as Address20, U256},
	providers::ProviderBuilder,
	sol,
};
use sp_core::crypto::AccountId32 as Address32;
use subxt::utils::Static;
use subxt_signer::sr25519::dev;
use tc_subxt::metadata;
use tracing_subscriber::filter::EnvFilter;

use metadata::runtime_types::pallet_assets_bridge::pallet::Call as BridgeCall;
use metadata::runtime_types::pallet_assets_bridge::types::NetworkData;
use time_primitives::NetworkId;

mod common;

use common::TestEnv;

const TC: NetworkId = 1000;
const EVM: NetworkId = 2;

const CONFIG: &str = "local-e2e-bridge.yaml";
const PROFILE: &str = "bridge";

const ERC20: Address20 = address!("0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0");
const BENEFICIARY: Address20 = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
const AMOUNT_OUT: u128 = 15_000_000_000_000;

// TODO could be taken from config
const ANVIL_RPC_URL: &str = "http://localhost:8545";

sol!(
	#[allow(missing_docs)]
	#[sol(rpc)]
	IERC20,
	"../analog-gmp/out/ERC20.sol/ERC20.json"
);

#[tokio::test]
async fn to_erc20() {
	let filter = EnvFilter::from_default_env()
		.add_directive("tc_cli=info".parse().unwrap())
		.add_directive("gmp_evm=info".parse().unwrap())
		.add_directive("bridge_test=info".parse().unwrap());
	tracing_subscriber::fmt().with_env_filter(filter).init();

	let env = TestEnv::spawn(CONFIG, PROFILE, true)
		.await
		.expect("Failed to spawn Test Environment");

	// TODO use TC snapshot with all prerequisite things set up there
	let _ = env.setup_test(EVM, TC).await.expect("failed to setup TC for the test");

	// NOTE we use evm chain snapshot, with state wich already has:
	// 1. GMP Gateway deployed
	// 2. TC network registered to it
	// 3. ERC20 AnlogToken (proxy+implementaion) deployed

	let data = NetworkData {
		nonce: 0,
		dest: Static(Address32::new(ERC20.into_word().0)),
	};

	//	4. Register network EVM as teleportation target at pallet_bridge
	let api = &env.tc.runtime().client;
	let call = metadata::RuntimeCall::Bridge(BridgeCall::register_network {
		network: EVM.into(),
		base_fee: 0,
		data,
	});
	let sudo_tx = metadata::tx().sudo().sudo(call);

	let from = dev::eve();
	let from_acc = Address32::new(from.public_key().to_account_id().0);
	let events = api
		.tx()
		.sign_and_submit_then_watch_default(&sudo_tx, &from)
		.await
		.unwrap()
		.wait_for_finalized_success()
		.await
		.unwrap();
	tracing::info!(target: "bridge_test", "Network {EVM} registered to bridge");

	let reg_event = events
		.find_first::<metadata::bridge::events::BridgeStatusChanged>()
		.ok()
		.flatten()
		.expect("BridgeStatusChanged event missed");
	tracing::info!(target: "bridge_test", "BridgeStatusChanged event found: {reg_event:?}");

	let tc_bal_before =
		&env.tc.runtime().balance(&from_acc).await.expect("cannot query sender balance");
	tracing::info!(target: "bridge_test", "Sender bal before: {tc_bal_before}");
	let query = metadata::constants().bridge().bridge_pot();
	let bridge_pot = api.constants().at(&query).expect("cannot query bridge pot address");
	let bridge_bal_before = &env
		.tc
		.runtime()
		.balance(&bridge_pot)
		.await
		.expect("cannot query bridge balance");
	tracing::info!(target: "bridge_test", "Bridge bal before: {bridge_bal_before}");

	//	5. Dispatch extrinsic for teleporting TC->ERC20
	let tx = metadata::tx().bridge().teleport_keep_alive(
		EVM.into(),
		BENEFICIARY.into_word().0,
		AMOUNT_OUT,
	);
	let events = api
		.tx()
		.sign_and_submit_then_watch_default(&tx, &from)
		.await
		.unwrap()
		.wait_for_finalized_success()
		.await
		.unwrap();
	tracing::info!(target: "bridge_test", "Amount of {AMOUNT_OUT} teleported out");
	let tel_event = events
		.find_first::<metadata::bridge::events::Teleported>()
		.ok()
		.flatten()
		.expect("Teleported event missed");
	tracing::info!(target: "bridge_test", "Teleported event found: {tel_event:?}");

	let mut blocks_sub = api
		.blocks()
		.subscribe_finalized()
		.await
		.expect("cant' subscribe to finalized blocks");
	let _ = blocks_sub.next().await.expect("cant' get next block");
	// wait for the next block to get TaskCreated event
	let block = blocks_sub.next().await.map(|x| x.ok()).flatten().expect("cant' get next block");
	tracing::info!(target: "bridge_test", "Next finalized block: #{}", block.number());
	let events = api.events().at(block.hash()).await.expect("cant get latest block events");
	let tsk_event = events
		.find_first::<metadata::tasks::events::TaskCreated>()
		.ok()
		.flatten()
		.expect("TaskCreated event missed");
	tracing::info!(target: "bridge_test", "TaskCreated event found: {tsk_event:?}");

	let task_id = tsk_event.0;

	// 6. Check source balance(s)
	let tc_bal_after =
		&env.tc.runtime().balance(&from_acc).await.expect("cannot query sender balance");
	tracing::info!(target: "bridge_test", "Sender bal after: {tc_bal_after}");
	let bridge_bal_after = &env
		.tc
		.runtime()
		.balance(&bridge_pot)
		.await
		.expect("cannot query bridge balance");
	tracing::info!(target: "bridge_test", "Bridge bal after: {bridge_bal_after}");
	// Sender paid teleported amount plus some fees
	assert!(tc_bal_after.saturating_add(AMOUNT_OUT) < *tc_bal_before);
	// Bridge Pot should get the exact teleported amount
	assert_eq!(bridge_bal_after.saturating_sub(*bridge_bal_before), AMOUNT_OUT);

	// 7. Wait for task to execute
	let query = metadata::storage().tasks().batch_id_counter();
	let batch_id = api
		.storage()
		.at_latest()
		.await
		.unwrap()
		.fetch(&query)
		.await
		.ok()
		.flatten()
		.expect("cant query batch_id")
		.saturating_sub(1);
	let query = metadata::storage().tasks().batch_task_id(batch_id);
	let task_id1 = api
		.storage()
		.at_latest()
		.await
		.unwrap()
		.fetch(&query)
		.await
		.ok()
		.flatten()
		.expect("cant query batch_task_id");
	assert_eq!(task_id, task_id1);

	tracing::info!(target: "bridge_test", "Teleport task: {task_id}, batch: {batch_id}");

	let query = metadata::storage().tasks().batch_tx_hash(batch_id);
	let start = block.number();
	const TIMEOUT: u64 = 65;

	let tx_hash = loop {
		let bn = blocks_sub
			.next()
			.await
			.map(|x| x.ok())
			.flatten()
			.expect("cant' get next block")
			.number();

		tracing::info!(target: "bridge_test", "Waiting for task: {task_id} to be executed... TC block: #{bn}");

		if let Some(tx_hash) = api
			.storage()
			.at_latest()
			.await
			.unwrap()
			.fetch(&query)
			.await
			.expect("cant query batch_id")
		{
			break tx_hash;
		}

		if (bn.saturating_sub(start) as u64) > TIMEOUT {
			panic!("Teleport task was not executed within {TIMEOUT} blocks")
		}
	};

	tracing::info!(target: "bridge_test", "Teleport tx_hash: {:?}", hex::encode(&tx_hash));

	// 8. Check destination balance
	let provider = ProviderBuilder::new().on_http(ANVIL_RPC_URL.parse().expect("bad RPC_URL"));
	let contract = IERC20::new(ERC20, provider);
	let target_bal = contract.balanceOf(BENEFICIARY).call().await.unwrap()._0;
	tracing::info!(target: "bridge_test", "Resulting ERC20 balance: {target_bal}");

	assert_eq!(target_bal, U256::from(AMOUNT_OUT));
}

#[ignore]
#[tokio::test]
async fn from_erc20() {
	/*
	ERC20->TC
	6. call estimateTeleport
	7. send teleport tx ERC20->TC
	8. Wait for task to be completed & check the resulting balance(s)
	*/
	todo!()
}

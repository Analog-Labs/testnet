use std::str::FromStr;
use futures::prelude::stream::StreamExt;
use alloy::{
	eips::BlockNumberOrTag,
	network::EthereumWallet,
	primitives::{address, Address as Address20, FixedBytes, U256},
	providers::{Provider, ProviderBuilder, WsConnect},
	rpc::types::Filter,
	signers::local::PrivateKeySigner,
	sol, sol_types::SolEvent,
};
use sp_core::crypto::AccountId32 as Address32;
use subxt::utils::Static;
use subxt_signer::sr25519::dev;


use tc_subxt::metadata;

use metadata::runtime_types::pallet_assets_bridge::pallet::Call as BridgeCall;
use metadata::runtime_types::pallet_assets_bridge::types::NetworkData;
use time_primitives::NetworkId;

mod common;

use common::TestEnv;

const TC: NetworkId = 1000;
const EVM: NetworkId = 2;
const TIMEOUT: u64 = 65;

const CONFIG: &str = "local-e2e-bridge.yaml";
const PROFILE: &str = "bridge";

const TOKEN: Address20 = address!("0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0");
const BENEFICIARY: Address20 = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
const PAYER: Address20 = address!("0x70997970C51812dc3A010C7d01b50e0d17dc79C8");
const PAYER_KEY: &str = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const AMOUNT_OUT: u128 = 15_000_000_000_000;
const AMOUNT_IN: u128 = 7_000_000_000_000;

// TODO could be taken from config
const ANVIL_RPC_URL: &str = "http://localhost:8545";
const ANVIL_RPC_URL_WS: &str = "ws://localhost:8545";

sol! {
	#[allow(missing_docs)]
	#[sol(rpc)]
	contract Token {
		event OutboundTransfer(bytes32 indexed id, address indexed source, bytes32 indexed recipient, uint256 amount);

		function totalSupply() public view virtual returns (uint256) { return 100500; }
		function balanceOf(address account) public view virtual returns (uint256) { return 7; }

		function estimateTeleportCost() public view returns (uint256) {	return 42; }
		function teleport(bytes32 to, uint256 value) external payable returns (bytes32 messageID) { return bytes32(0); }
	}
}

async fn prepare() -> (TestEnv<'static>, Static<Address32>, u128) {
	let env = TestEnv::spawn(CONFIG, PROFILE, false)
		.await
		.expect("Failed to spawn Test Environment");

	// TODO use TC snapshot with all prerequisite things set up there
	let _ = env.setup_test(EVM, TC).await.expect("failed to setup TC for the test");

	// NOTE we use evm chain snapshot, with state wich already has:
	// 1. GMP Gateway deployed
	// 2. TC network (route) registered to it
	// 3. TOKEN AnlogToken (proxy+implementaion) deployed,
	//    and PAYER has AMOUNT_IN of tokens.

	let data = NetworkData {
		nonce: 0,
		dest: Static(Address32::new(TOKEN.into_word().0)),
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

	let query = metadata::constants().bridge().bridge_pot();
	let bridge_pot = api.constants().at(&query).expect("cannot query bridge pot address");
	let bridge_bal_before = &env
		.tc
		.runtime()
		.balance(&bridge_pot)
		.await
		.expect("cannot query bridge balance");
	tracing::info!(target: "bridge_test", "Bridge bal before: {bridge_bal_before}");

	(env, bridge_pot, *bridge_bal_before)
}

#[tokio::test]
async fn to_erc20() {
	let (env, bridge_pot, bridge_bal_before) = prepare().await;
	let api = &env.tc.runtime().client;

	let from = dev::eve();
	let from_acc = Address32::new(from.public_key().to_account_id().0);
	let tc_bal_before =
		&env.tc.runtime().balance(&from_acc).await.expect("cannot query sender balance");
	tracing::info!(target: "bridge_test", "Sender bal before: {tc_bal_before}");

	//	5. Dispatch extrinsic for teleporting TC->TOKEN
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
	assert_eq!(bridge_bal_after.saturating_sub(bridge_bal_before), AMOUNT_OUT);

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
	let contract = Token::new(TOKEN, provider);
	let target_bal = contract.balanceOf(BENEFICIARY).call().await.unwrap()._0;
	tracing::info!(target: "bridge_test", "Resulting TOKEN balance: {target_bal}");

	assert_eq!(target_bal, U256::from(AMOUNT_OUT));
}

#[tokio::test]
async fn from_erc20() {
	let (env, _bridge_pot, _bridge_bal_before) = prepare().await;
	let _api = &env.tc.runtime().client;
	// let _env = TestEnv::spawn(CONFIG, PROFILE, true)
	// 	.await
	// 	.expect("Failed to spawn Test Environment");

	let signer = PrivateKeySigner::from_str(PAYER_KEY).expect("can't parse signer key");
	let wallet = EthereumWallet::from(signer);

	let ws = WsConnect::new(ANVIL_RPC_URL_WS);
	let provider = ProviderBuilder::new()
		.wallet(wallet)
		.on_ws(ws)
		.await
		.expect("can't build provider");

	let latest_block = provider.get_block_number().await.unwrap();
	tracing::info!(target: "bridge_test", "[network {EVM}] latest block is #{latest_block}");

	let token = Token::new(TOKEN, provider.clone());
	let source_bal = token.balanceOf(PAYER).call().await.unwrap()._0;
	let supply = token.totalSupply().call().await.unwrap()._0;

	assert!(source_bal >= U256::from(AMOUNT_IN));
	assert_eq!(supply, source_bal);

	//	5. call estimateTeleport
	let teleport_cost = token.estimateTeleportCost().call().await.unwrap()._0;
	tracing::info!(target: "bridge_test", "Teleport cost is: {teleport_cost}");

	let filter = Filter::new()
		.event(Token::OutboundTransfer::SIGNATURE)
		.from_block(BlockNumberOrTag::Number(latest_block));
	let sub = provider.subscribe_logs(&filter).await.unwrap();
	let mut stream = sub.into_stream();

	// 6. send teleport tx ERC20->TC
	let to = FixedBytes(dev::dave().public_key().to_account_id().0);
	let tx_hash = token
		.teleport(to, U256::from(AMOUNT_IN))
		.value(teleport_cost)
		.send()
		.await
		.expect("failed to send teleport tx")
		.with_required_confirmations(1)
		.with_timeout(Some(std::time::Duration::from_secs(3)))
		.watch()
		.await
		.expect("teleport tx failed");

	tracing::info!(target: "bridge_test", "Teleport sent, tx_hash: {:#?}", &tx_hash);
	tracing::info!(target: "bridge_test", "Waiting for event...");
	tracing::info!(target: "bridge_test", "Event hash: {}", &Token::OutboundTransfer::SIGNATURE_HASH);


	let teleport_event =
		stream.next().await.expect("can't get OutboundTransfer() event");
	let Token::OutboundTransfer { id, .. } =
		teleport_event.log_decode().expect("can't decode event").inner.data;

	tracing::info!(target: "bridge_test", "Event found, msg_id: {:#?}", &id);

	/*
	7. Wait for task to be completed & check the resulting balance(s)
	*/
	//	todo!()
}

#[ignore]
#[tokio::test]
async fn chronicle_cannot_mint() {
	todo!()
}

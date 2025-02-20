use alloy_core::primitives::{address, Address as Address20};
use futures::StreamExt;
use metadata::runtime_types::pallet_assets_bridge::pallet::Call as BridgeCall;
use metadata::runtime_types::pallet_assets_bridge::types::NetworkData;

use sp_core::crypto::AccountId32 as Address32;
use tc_cli::Tc;
use tracing_subscriber::filter::EnvFilter;

mod common;

use common::TestEnv;
use subxt::utils::Static;
use subxt_signer::sr25519::dev;
use tc_subxt::metadata;

use time_primitives::{Gateway, NetworkId};

const TC: NetworkId = 1000;
const EVM: NetworkId = 2;

const CONFIG: &str = "local-e2e-bridge.yaml";
const PROFILE: &str = "bridge";

const GATEWAY: Address20 = address!("0x49877F1e26d523e716d941a424af46B86EcaF09E");
const ERC20: Address20 = address!("0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0");
const BENEFICIARY: Address20 = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
const AMOUNT_OUT: u128 = 15_000_000_000_000;

#[tokio::test]
async fn to_erc20() {
	let filter = EnvFilter::from_default_env()
		.add_directive("tc_cli=info".parse().unwrap())
		.add_directive("gmp_evm=info".parse().unwrap())
		.add_directive("bridge_test=info".parse().unwrap());
	tracing_subscriber::fmt().with_env_filter(filter).init();

	let env = TestEnv::spawn(CONFIG, PROFILE, false)
		.await
		.expect("Failed to spawn Test Environment");

	//	let (src_addr, _) = env.setup_test(EVM, TC).await.expect("failed to setup test");

	/* PLAN: (follow steps at README)
	PREPARE: NOTE: we have all of this already done and store in the anvil snapshot
	1. Register TC network (route) to the gateway at network 2
	 */
	// let gw: Gateway = GATEWAY.into_word().into();
	// let _ = &env.tc.set_tc_route(EVM, gw).await.expect("failed to set tc route");
	// tracing::info!("TC route registered");
	/*
	2. Deploy ERC20 contracts
	TC->ERC20
	 */

	let data = NetworkData {
		nonce: 0,
		dest: Static(Address32::new(ERC20.into_word().0)),
	};
	//	3. REgister nw 2 @bridge pallet
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

	//	4. Dispatch extrinsic for teleport TC->ERC20
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
	/*
	5. Wait for task to complete (or batch to get tx_hash) & check the resulting balance(s)
	ERC20->TC
	6. call estimateTeleport
	7. send teleport tx ERC20->TC
	8. Wait for task to be completed & check the resulting balance(s)
	*/
}

use futures::StreamExt;
use tc_cli::Tc;
use tracing_subscriber::filter::EnvFilter;

mod common;

use common::TestEnv;
use time_primitives::{Address, NetworkId};

const TC: NetworkId = 1000;
const EVM: NetworkId = 2;

const CONFIG: &str = "local-e2e-bridge.yaml";
const PROFILE: &str = "bridge";

#[tokio::test]
async fn to_erc20_and_back() {
	let filter = EnvFilter::from_default_env()
		.add_directive("tc_cli=info".parse().unwrap())
		.add_directive("gmp_evm=info".parse().unwrap())
		.add_directive("bridge_test=info".parse().unwrap());
	tracing_subscriber::fmt().with_env_filter(filter).init();

	let env = TestEnv::spawn(CONFIG, PROFILE, true)
		.await
		.expect("Failed to spawn Test Environment");

	let (src_addr, _) = env.setup_test(EVM, TC).await.expect("failed to setup test");

	/* PLAN: (follow steps at README)
	PREPARE:
	1. Register TC network (route) to the gateway at network 2
	 */
	let gw = &env.tc.gateway(EVM).await.expect("failed to get gateway address").1;
	let _ = &env.tc.set_tc_route(EVM, *gw).await.expect("failed to set tc route");
	/*
	2. Deploy ERC20 contracts
	TC->ERC20
	3. REgister nw 2 @bridge pallet
	4. Dispatch extrinsic for teleport TC->ERC20
	5. Wait for task to complete (or batch to get tx_hash) & check the resulting balance(s)
	ERC20->TC
	6. call estimateTeleport
	7. send teleport tx ERC20->TC
	8. Wait for task to be completed & check the resulting balance(s)
	*/
}

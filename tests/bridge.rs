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
}

use anyhow::{Context, Result};
use ethers_solc::{artifacts::Source, CompilerInput, Solc};
use rosetta_client::{Blockchain, Wallet};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;
use tc_subxt::{rpc_params, RpcParams, SubxtClient};

#[derive(Clone, Debug)]
pub(crate) struct WalletConfig {
	pub blockchain: Blockchain,
	pub network: String,
	pub url: String,
}

pub(crate) async fn setup_env(config: &WalletConfig, is_gmp: bool) -> (String, u64) {
	set_keys(config).await;
	fund_wallet(config).await;
	deploy_contract(config, is_gmp).await.unwrap()
}

async fn insert_key(url: &str, key_type: &str, suri: &str, public_key: &str) {
	loop {
		let Ok(api) = SubxtClient::new(url, None).await else {
			println!("failed to connect to node {}", url);
			tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
			continue;
		};
		let params: RpcParams = rpc_params![key_type, suri, public_key];
		let _: () = api.rpc("author_insertKey", params).await.unwrap();
		println!("submitted key for node: {}", url);
		break;
	}
}

pub(crate) async fn set_keys(config: &WalletConfig) {
	let suri = |i: u8| {
		format!("owner word vocal dose decline sunset battle example forget excite gentle waste//{i}//time")
	};
	let keys = [
		"0x78af33d076b81fddce1c051a72bb1a23fd32519a2ede7ba7a54b2c76d110c54d",
		"0xcee262950a61e921ac72217fd5578c122bfc91ba5c0580dbfbe42148cf35be2b",
		"0xa01b6ceec7fb1d32bace8ffcac21ffe6839d3a2ebe26d86923be9dd94c0c9a02",
		"0x1e31bbe09138bef48ffaca76214317eb0f7a8fd85959774e41d180f2ad9e741f",
		"0x1843caba7078a699217b23bcec8b57db996fc3d1804948e9ee159fc1dc9b8659",
		"0x72a170526bb41438d918a9827834c38aff8571bfe9203e38b7a6fd93ecf70d69",
	];
	match config.blockchain {
		Blockchain::Ethereum => {
			insert_key("ws://chronicle-eth:9944", "time", &suri(1), keys[0]).await;
		},
		Blockchain::Astar => {
			insert_key("ws://chronicle-astar:9944", "time", &suri(2), keys[1]).await;
		},
		_ => unreachable!(),
	}
}

pub(crate) async fn deploy_contract(config: &WalletConfig, is_gmp: bool) -> Result<(String, u64)> {
	println!("Deploying eth contract");
	let wallet =
		Wallet::new(config.blockchain, &config.network, &config.url, Some("/etc/keyfile".as_ref()))
			.await
			.unwrap();

	let bytes = if is_gmp {
		compile_file("/etc/gateway.sol")?
	} else {
		compile_file("/etc/test_contract.sol")?
	};
	let tx_hash = wallet.eth_deploy_contract(bytes).await?;
	let tx_receipt = wallet.eth_transaction_receipt(&tx_hash).await?;
	let contract_address = tx_receipt
		.get("contractAddress")
		.and_then(|v| v.as_str().map(str::to_string))
		.ok_or(anyhow::anyhow!("Unable to get contract address"))?;
	let status = wallet.status().await?;

	println!("Deploy contract address {:?} on {:?}", contract_address, status.index);
	Ok((contract_address.to_string(), status.index))
}

pub(crate) async fn fund_wallet(config: &WalletConfig) {
	println!("funding wallet for {:?}", config.blockchain);
	let wallet =
		Wallet::new(config.blockchain, &config.network, &config.url, Some("/etc/keyfile".as_ref()))
			.await
			.unwrap();
	wallet.faucet(1000000000000000000000).await.unwrap();
}

pub(crate) fn drop_node(node_name: String) {
	let output = Command::new("docker")
		.args(["stop", &node_name])
		.output()
		.expect("failed to drop node");
	println!("Dropped node {:?}", output.stderr.is_empty());
}

pub(crate) fn start_node(node_name: String) {
	let output = Command::new("docker")
		.args(["start", &node_name])
		.output()
		.expect("failed to start node");
	println!("Start node {:?}", output.stderr.is_empty());
}

pub(crate) fn restart_node(node_name: String) {
	let output = Command::new("docker")
		.args(["restart", &node_name])
		.output()
		.expect("failed to start node");
	println!("Restart node {:?}", output.stderr.is_empty());
}

pub(crate) fn compile_file(path: &str) -> Result<Vec<u8>> {
	let solc = Solc::default();
	let mut sources = BTreeMap::new();
	sources.insert(Path::new(path).into(), Source::read(path).unwrap());
	let input = &CompilerInput::with_sources(sources)[0];
	let output = solc.compile_exact(input)?;
	let file = output.contracts.get(path).unwrap();
	let (key, _) = file.first_key_value().unwrap();
	let contract = file.get(key).unwrap();
	let bytecode = contract
		.evm
		.as_ref()
		.context("evm not found")?
		.bytecode
		.as_ref()
		.context("bytecode not found")?
		.object
		.as_bytes()
		.context("could not convert to bytes")?
		.to_vec();
	Ok(bytecode)
}

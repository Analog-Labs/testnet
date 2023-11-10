use crate::mock::*;
use crate::tasks::*;
use shards as Shards;

use clap::Parser;
use rosetta_client::Blockchain;
use tc_subxt::Network;
use tc_subxt::SubxtClient;

mod mock;
mod shards;
mod tasks;

#[derive(Parser, Debug)]
struct Args {
	#[arg(long, default_value = "ws://validator:9944")]
	url: String,
	#[arg(long)]
	network: String,
	#[clap(subcommand)]
	cmd: TestCommand,
}

#[derive(Parser, Debug)]
enum TestCommand {
	Basic,
	BasicSign,
	BatchTask { tasks: u64, max_cycle: u64 },
	NodeDropTest,
	KeyRecovery { nodes: u8 },
	ShardRestart,
	DeployContract,
	FundWallet,
	InsertTask(InsertTaskParams),
	InsertSignTask(InsertSignTaskParams),
	SetKeys,
	WatchTask { task_id: u64 },
}

#[derive(Parser, Debug)]
struct InsertSignTaskParams {
	#[arg(default_value_t = 2)]
	cycle: u64,
	#[arg(default_value_t = 20)]
	start: u64,
	#[arg(default_value_t = 2)]
	period: u64,
	#[arg(default_value_t = String::new())]
	contract_address: String,
	#[arg(default_value_t = String::new())]
	payload: String,
}

#[derive(Parser, Debug)]
struct InsertTaskParams {
	#[arg(default_value_t=String::from(""))]
	address: String,
	#[arg(default_value_t = 2)]
	cycle: u64,
	#[arg(default_value_t = 20)]
	start: u64,
	#[arg(default_value_t = 2)]
	period: u64,
	#[arg(default_value_t = false)]
	is_payable: bool,
}

#[tokio::main]
async fn main() {
	let args = Args::parse();
	let url = args.url;
	let api = loop {
		let Ok(api) = SubxtClient::new(&url, None).await else {
			println!("waiting for chain to start");
			tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
			continue;
		};
		break api;
	};

	let (network, config) = match args.network.as_str() {
		"ethereum" => (
			Network::Ethereum,
			WalletConfig {
				blockchain: Blockchain::Ethereum,
				network: "dev".to_string(),
				url: "ws://ethereum:8545".to_string(),
			},
		),
		"astar" => (
			Network::Astar,
			WalletConfig {
				blockchain: Blockchain::Astar,
				network: "dev".to_string(),
				url: "ws://astar:9944".to_string(),
			},
		),
		network => panic!("unsupported network {}", network),
	};

	match args.cmd {
		TestCommand::Basic => {
			basic_test_timechain(&api, network, &config).await;
		},
		TestCommand::BasicSign => {
			basic_sign_test(&api, network, &config).await;
		},
		TestCommand::BatchTask { tasks, max_cycle } => {
			batch_test(&api, tasks, max_cycle, &config).await;
		},
		TestCommand::NodeDropTest => {
			node_drop_test(&api, &config).await;
		},
		TestCommand::KeyRecovery { nodes } => {
			key_recovery_after_drop(&api, &config, nodes).await;
		},
		TestCommand::ShardRestart => {
			task_update_after_shard_offline(&api, &config).await;
		},
		TestCommand::SetKeys => {
			set_keys(&config).await;
		},
		TestCommand::FundWallet => {
			fund_wallet(&config).await;
		},
		TestCommand::DeployContract => {
			if let Err(e) = deploy_contract(&config).await {
				println!("error {:?}", e);
			}
		},
		TestCommand::InsertTask(params) => {
			insert_evm_task(
				&api,
				params.address,
				params.cycle,
				params.start,
				params.period,
				network,
				params.is_payable,
			)
			.await
			.unwrap();
		},
		TestCommand::InsertSignTask(params) => {
			insert_sign_task(
				&api,
				params.cycle,
				params.start,
				params.period,
				network,
				params.contract_address.into_bytes().to_vec(),
				params.payload.into_bytes().to_vec(),
			)
			.await
			.unwrap();
		},
		TestCommand::WatchTask { task_id } => {
			while !watch_task(&api, task_id).await {
				tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
			}
		},
	}
}

async fn basic_test_timechain(api: &SubxtClient, network: Network, config: &WalletConfig) {
	let (contract_address, start_block) = setup_env(config).await;

	let task_id = insert_evm_task(
		api,
		contract_address.clone(),
		2, //cycle
		start_block,
		2, //period
		network.clone(),
		false,
	)
	.await
	.unwrap();
	while !watch_task(api, task_id).await {
		tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
	}

	let task_id = insert_evm_task(api, contract_address, 1, start_block, 0, network, true)
		.await
		.unwrap();
	while !watch_task(api, task_id).await {
		tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
	}
}

async fn basic_sign_test(api: &SubxtClient, network: Network, config: &WalletConfig) {
	let (contract_address, start_block) = setup_env(config).await;

	let task_id = insert_sign_task(
		api,
		2, //cycle
		2, //period
		start_block,
		network.clone(),
		contract_address.clone().into(),
		"vote_yes()".into(), //payload
	)
	.await
	.unwrap();
	while !watch_task(api, task_id).await {
		tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
	}
}

async fn batch_test(api: &SubxtClient, total_tasks: u64, max_cycle: u64, config: &WalletConfig) {
	let (contract_address, start_block) = setup_env(config).await;

	let mut task_ids = vec![];

	for _ in 0..total_tasks {
		let task_id = insert_evm_task(
			api,
			contract_address.clone(),
			max_cycle,
			start_block,
			2, //period
			Network::Ethereum,
			false,
		)
		.await
		.unwrap();
		task_ids.push(task_id);
	}
	while !watch_batch(api, task_ids[0], task_ids.len() as u64, max_cycle).await {
		tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
	}
}

async fn node_drop_test(api: &SubxtClient, config: &WalletConfig) {
	let (contract_address, start_block) = setup_env(config).await;

	let task_id = insert_evm_task(
		api,
		contract_address.clone(),
		5, //cycle
		start_block,
		5, //period
		Network::Ethereum,
		false,
	)
	.await
	.unwrap();
	//wait for some cycles to run
	tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
	//drop 1 node
	drop_node("testnet-validator1-1".to_string());
	//watch task
	while !watch_task(api, task_id).await {
		tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
	}
}

async fn task_update_after_shard_offline(api: &SubxtClient, config: &WalletConfig) {
	let (contract_address, start_block) = setup_env(config).await;

	let task_id = insert_evm_task(
		api,
		contract_address.clone(),
		10, //cycle
		start_block,
		5, //period
		Network::Ethereum,
		false,
	)
	.await
	.unwrap();
	// wait for some cycles to run, Note: tasks are running in background
	tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

	// drop 2 nodes
	drop_node("testnet-validator1-1".to_string());
	drop_node("testnet-validator2-1".to_string());
	println!("dropped 2 nodes");

	// wait for some time
	while Shards::is_shard_online(api, Network::Ethereum).await {
		println!("Waiting for shard offline");
		tokio::time::sleep(tokio::time::Duration::from_secs(50)).await;
	}
	println!("Shard is offline, starting nodes");

	// start nodes again
	start_node("validator1".to_string());
	start_node("validator2".to_string());

	// watch task
	while !watch_task(api, task_id).await {
		tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
	}
}

async fn key_recovery_after_drop(api: &SubxtClient, config: &WalletConfig, nodes_to_restart: u8) {
	let (contract_address, start_block) = setup_env(config).await;

	let task_id = insert_evm_task(
		api,
		contract_address.clone(),
		10, //cycle
		start_block,
		5, //period
		Network::Ethereum,
		false,
	)
	.await
	.unwrap();
	// wait for some cycles to run, Note: tasks are running in background
	for i in 1..nodes_to_restart + 1 {
		println!("waiting for 1 min");
		tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
		println!("restarting node {}", i);
		restart_node(format!("testnet-chronicle-eth-{}", i));
	}
	println!("waiting for 20 secs to let node recover completely");
	tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
	// watch task
	while !watch_task(api, task_id).await {
		tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
	}
}

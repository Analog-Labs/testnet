use crate::{
	benchmarking::{inherent_benchmark_data, RemarkBuilder, TransferKeepAliveBuilder},
	chain_spec,
	cli::{Cli, Subcommand},
	service,
};
use frame_benchmarking_cli::{BenchmarkCmd, ExtrinsicFactory, SUBSTRATE_REFERENCE_HARDWARE};
use sc_cli::SubstrateCli;
use sc_service::PartialComponents;
use sp_keyring::Sr25519Keyring;
use timechain_runtime::{Block, EXISTENTIAL_DEPOSIT};

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Substrate Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.analog.one".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"testnet" => Box::new(chain_spec::analog_testnet_config()?),
			"staging" => Box::new(chain_spec::analog_staging_config(false)?),
			"staging-notss" => Box::new(chain_spec::analog_staging_config(true)?),
			"" | "dev" => Box::new(chain_spec::analog_dev_config(false)?),
			"notss" => Box::new(chain_spec::analog_dev_config(true)?),
			path => {
				Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?)
			},
		})
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();
	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					import_queue,
					..
				} = service::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = service::new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = service::new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client,
					task_manager,
					import_queue,
					..
				} = service::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents {
					client, task_manager, backend, ..
				} = service::new_partial(&config)?;
				let aux_revert = Box::new(|client, _, blocks| {
					sc_consensus_grandpa::revert(client, blocks)?;
					Ok(())
				});
				Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd.as_ref())?;

			runner.sync_run(|config| {
				// This switch needs to be in the client, since the client decides
				// which sub-commands it wants to support.
				match cmd.as_ref() {
					BenchmarkCmd::Pallet(cmd) => {
						if !cfg!(feature = "runtime-benchmarks") {
							return Err(
								"Runtime benchmarking wasn't enabled when building the node. \
							You can enable it with `--features runtime-benchmarks`."
									.into(),
							);
						}

						cmd.run::<Block, sp_statement_store::runtime_api::HostFunctions>(config)
					},
					BenchmarkCmd::Block(cmd) => {
						// ensure that we keep the task manager alive
						let partial = service::new_partial(&config)?;

						cmd.run(partial.client)
					},
					#[cfg(not(feature = "runtime-benchmarks"))]
					BenchmarkCmd::Storage(_) => Err(
						"Storage benchmarking can be enabled with `--features runtime-benchmarks`."
							.into(),
					),
					#[cfg(feature = "runtime-benchmarks")]
					BenchmarkCmd::Storage(cmd) => {
						// ensure that we keep the task manager alive
						let partial = service::new_partial(&config)?;
						let db = partial.backend.expose_db();
						let storage = partial.backend.expose_storage();

						cmd.run(config, partial.client, db, storage)
					},
					BenchmarkCmd::Overhead(cmd) => {
						// ensure that we keep the task manager alive
						let partial = service::new_partial(&config)?;
						let ext_builder = RemarkBuilder::new(partial.client.clone());

						cmd.run(
							config,
							partial.client,
							inherent_benchmark_data()?,
							Vec::new(),
							&ext_builder,
						)
					},
					BenchmarkCmd::Extrinsic(cmd) => {
						// ensure that we keep the task manager alive
						let partial = service::new_partial(&config)?;
						// Register the *Remark* and *TKA* builders.
						let ext_factory = ExtrinsicFactory(vec![
							Box::new(RemarkBuilder::new(partial.client.clone())),
							Box::new(TransferKeepAliveBuilder::new(
								partial.client.clone(),
								Sr25519Keyring::Alice.to_account_id(),
								EXISTENTIAL_DEPOSIT,
							)),
						]);

						cmd.run(
							partial.client,
							inherent_benchmark_data()?,
							Vec::new(),
							&ext_factory,
						)
					},
					BenchmarkCmd::Machine(cmd) => {
						cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())
					},
				}
			})
		},
		Some(Subcommand::ChainInfo(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run::<Block>(&config))
		},
		None => {
			let runner = cli.create_runner(&cli.run.base)?;
			runner.run_node_until_exit(|config| async move {
				service::new_full(
					config,
					cli.run.chronicle.map(|args| chronicle::ChronicleConfig {
						blockchain: args.blockchain,
						network: args.network,
						url: args.url,
						timechain_keyfile: args.timechain_keyfile,
						keyfile: args.keyfile,
						timegraph_url: args.timegraph_url.or(std::env::var("TIMEGRAPH_URL").ok()),
						timegraph_ssk: args.timegraph_ssk.or(std::env::var("TIMEGRAPH_SSK").ok()),
						secret: args.secret,
						bind_port: args.bind_port,
						pkarr_relay: args.pkarr_relay,
					}),
				)
				.map_err(sc_cli::Error::Service)
			})
		},
	}
}

//! Substrate Node CLI

mod chain_spec;
#[cfg(feature = "chronicle")]
mod chronicle;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}

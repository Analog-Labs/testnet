use anyhow::{Context, Result};
use std::path::Path;
use std::process;
use tc_cli::{Sender, Tc};
use time_primitives::{Address, NetworkId};
use tracing_subscriber::filter::EnvFilter;

pub struct TestEnv<'a> {
	pub tc: Tc,
	pub profile: &'a str,
}

const ENV: &str = "../config/envs/local";

impl<'a> TestEnv<'a> {
	async fn new(config: &str, profile: &'a str) -> Result<Self> {
		let sender = Sender::new();
		let pbuf = Path::new(ENV).to_path_buf();
		tracing::warn!("PATH: {}", pbuf.display());
		let tc = Tc::new(Path::new(ENV).to_path_buf(), config, sender)
			.await
			.context("Error creating Tc client")?;
		Ok(TestEnv { tc, profile })
	}

	/// spawns new testing env
	pub async fn spawn(config: &str, profile: &'a str, build: bool) -> Result<Self> {
		if build && !build_containers()? {
			anyhow::bail!("Failed to build containers");
		}

		if !docker_up(profile)? {
			anyhow::bail!("Failed to start containers");
		}
		Self::new(config, profile).await
	}

	/// sets up test
	pub async fn setup_test(&self, src: NetworkId, dest: NetworkId) -> Result<(Address, Address)> {
		self.tc.setup_test(src, dest).await
	}

	/// restart container
	pub async fn restart(&self, containers: Vec<&str>) -> Result<bool> {
		docker_restart(containers)
	}
}

impl<'a> Drop for TestEnv<'a> {
	/// Tear-down logic for the tests
	fn drop(&mut self) {
		if !docker_down(self.profile).expect("Failed to stop containers") {
			println!(
				"Failed to stop containers, please stop by hand with:\n\
			               \t $> docker compose --profile=ethereum down"
			);
		};
	}
}

fn build_containers() -> Result<bool> {
	let mut cmd = process::Command::new(Path::new("../scripts/build_docker.sh"));
	let mut child = cmd.spawn().context("Error building containers")?;

	child.wait().map(|c| c.success()).context("Error building containers: {e}")
}

fn docker_up(profile: &str) -> Result<bool> {
	let mut cmd = process::Command::new("docker");
	let profile = format!("--profile={profile}");
	cmd.arg("compose").arg(profile).arg("up").arg("-d").arg("--wait");

	let mut child = cmd.spawn().context("Error starting containers")?;

	// Wait for all containers to start
	child.wait().map(|c| c.success()).context("Error starting containers")
}

fn docker_down(profile: &str) -> Result<bool> {
	let mut cmd = process::Command::new("docker");

	let profile = format!("--profile={profile}");
	cmd.arg("compose").arg(profile).arg("down");

	let mut child = cmd.spawn().context("Error stopping containers")?;

	// Wait for all containers to start
	child.wait().map(|c| c.success()).context("Error stopping containers: {e}")
}

fn docker_restart(containers: Vec<&str>) -> Result<bool> {
	let mut cmd = process::Command::new("docker");
	cmd.arg("compose").arg("stop").args(containers.as_slice());

	let mut child = cmd.spawn().context("Error stopping containers")?;
	// wait for the containers to stop
	child.wait().map(|c| c.success()).context("Error stopping containers")?;
	let mut cmd = process::Command::new("docker");
	cmd.arg("compose").arg("start").args(containers.as_slice());
	let mut child = cmd.spawn().context("Error stopping containers")?;
	// wait for the containers to start
	child.wait().map(|c| c.success()).context("Error starting containers")
}

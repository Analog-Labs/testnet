use convert_case::{Case, Casing};
use hex_literal::hex;
use serde::{Deserialize, Serialize};

use polkadot_sdk::*;

use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::json_merge;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{config::TelemetryEndpoints, ChainType};

use sp_authority_discovery::AuthorityId as DiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::crypto::UncheckedInto;
use sp_keyring::{AccountKeyring, Ed25519Keyring};
use sp_runtime::Perbill;

use mainnet_runtime::WASM_BINARY as MAINNET_RUNTIME;
use testnet_runtime::WASM_BINARY as TESTNET_RUNTIME;

use runtime_common::StakerStatus;
use time_primitives::{AccountId, Balance, Block, ANLOG, TOKEN_DECIMALS};

const SS_58_FORMAT: u32 = 12850;

/// Total supply of token is 90_570_710.
/// Initially we are distributing the total supply to the multiple accounts which is representing
/// its category pool which we will update in later part of development.
const SEED_ROUND_SUPPLY: Balance = ANLOG * 24_275_364;
const INITIAL_PRIVATE_SALE: Balance = ANLOG * 1_837_476;
const PRIVATE_SALE: Balance = ANLOG * 8_919_012;
const PUBLIC_SALE: Balance = ANLOG * 1_449_275;
const TEAM_SUPPLY: Balance = ANLOG * 17_210_160;
const TREASURY_SUPPLY: Balance = ANLOG * 13_224_636;
const COMMUNITY_SUPPLY: Balance = ANLOG * 23_663_800;

/// Stash and float for validators
const PER_VALIDATOR_STASH: Balance = ANLOG * 500_000;
const PER_VALIDATOR_UNLOCKED: Balance = ANLOG * 20_000;

/// Stash for community nominations
const PER_NOMINATION: Balance = ANLOG * 180_000;
const PER_NOMINATOR_STASH: Balance = 8 * PER_NOMINATION;

/// Stash and float for chronicles
const PER_CHRONICLE_STASH: Balance = ANLOG * 100_000;

/// Token supply for prefunded admin accounts
const SUDO_SUPPLY: Balance = ANLOG * 50_000;
const CONTROLLER_SUPPLY: Balance = ANLOG * 50_000;
const PER_COUNCIL_STASH: Balance = ANLOG * 50_000;

/// Minimum needed validators, currently lowered for testing environments
const MIN_VALIDATOR_COUNT: u32 = 1;

/// Default telemetry server for all networks
const DEFAULT_TELEMETRY_URL: &str = "wss://telemetry.analog.one/submit";
const DEFAULT_TELEMETRY_LEVEL: u8 = 1;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

/// Helper enum used in internal calls
enum RuntimeTarget {
	Mainnet,
	Testnet,
}

/// Helper to parse genesis keys json
#[derive(serde::Deserialize)]
pub struct GenesisKeysConfig {
	/// Keys used to bootstrap validator session keys.
	/// Will match and register session keys to stashes and self-stake them.
	/// Balance to be staked is controlled by PER_VALIDATOR_UNLOCKED
	bootstraps: Vec<(BabeId, GrandpaId, ImOnlineId, DiscoveryId)>,
	/// Stashes to be used for chronicles, balances controlled by PER_CHRONICLE_STASH
	chronicles: Vec<AccountId>,
	/// Optional controller account that will control all nominates stakes
	controller: Option<AccountId>,
	/// Genesis members of on-chain council
	councils: Vec<AccountId>,
	/// Additional endowed accounts and their balance in ANLOG.
	endowments: Vec<(AccountId, Balance)>,
	/// Stashes intended for community nominations.
	/// Sizing controlled by PER_NOMINATION_STASH
	nominators: Vec<AccountId>,
	/// Stashes intended to be used to run validators.
	/// There has to be at least one stash for every
	/// session key set. Balance controlled by PER_VALIDATOR_STASH
	stakes: Vec<AccountId>,
	/// Root account to controll sudo pallet
	sudo: AccountId,
}

impl Default for GenesisKeysConfig {
	/// Default configuration using know development keys
	fn default() -> Self {
		use AccountKeyring::*;

		GenesisKeysConfig {
			bootstraps: vec![(
				Alice.to_raw_public().unchecked_into(),
				Ed25519Keyring::Alice.to_raw_public().unchecked_into(),
				Alice.to_raw_public().unchecked_into(),
				Alice.to_raw_public().unchecked_into(),
			)],
			chronicles: vec![],
			// TODO: Would be better to assign individual controllers
			controller: None,
			councils: vec![Bob.into(), Charlie.into(), Dave.into(), Eve.into(), Ferdie.into()],
			endowments: vec![(
				hex!["6d6f646c70792f74727372790000000000000000000000000000000000000000"].into(),
				TREASURY_SUPPLY,
			)],
			nominators: vec![],
			stakes: vec![
				AliceStash.into(),
				BobStash.into(),
				CharlieStash.into(),
				DaveStash.into(),
				EveStash.into(),
				FerdieStash.into(),
			],
			sudo: Alice.into(),
		}
	}
}

impl GenesisKeysConfig {
	/// Deserialize genesis key config from json bytes
	pub fn from_json_bytes(json: &[u8]) -> Result<Self, String> {
		serde_json::from_slice(json).map_err(|e| e.to_string())
	}

	/// Generate chain candidate for live deployment
	pub fn to_mainnet(&self) -> Result<ChainSpec, String> {
		self.to_chain_spec(
			"analog-timechain",
			RuntimeTarget::Mainnet,
			"ANLOG",
			ChainType::Live,
			12,
			8,
		)
	}

	/// Generate mainnet staging chain for supplied sub-identifier
	pub fn to_staging(&self, subid: &str) -> Result<ChainSpec, String> {
		let id = "analog-".to_owned() + subid;
		self.to_chain_spec(
			id.as_str(),
			RuntimeTarget::Mainnet,
			"SANLOG",
			ChainType::Development,
			6,
			4,
		)
	}

	/// Generate testnet development chain for supplied sub-identifier
	pub fn to_development(&self, subid: &str) -> Result<ChainSpec, String> {
		let id = "analog-".to_owned() + subid;
		self.to_chain_spec(
			id.as_str(),
			RuntimeTarget::Testnet,
			"DANLOG",
			ChainType::Development,
			6,
			4,
		)
	}

	/// Generate a local mainnet chain spec
	pub fn to_local_staging(&self) -> Result<ChainSpec, String> {
		self.to_chain_spec(
			"analog-local-mainnet",
			RuntimeTarget::Mainnet,
			"SANLOG",
			ChainType::Local,
			3,
			2,
		)
	}

	/// Generate a local testnet chain spec
	pub fn to_local_development(&self) -> Result<ChainSpec, String> {
		self.to_chain_spec(
			"analog-local-testnet",
			RuntimeTarget::Testnet,
			"DANLOG",
			ChainType::Local,
			3,
			2,
		)
	}

	/// Generate a chain spec from key config
	fn to_chain_spec(
		&self,
		id: &str,
		runtime: RuntimeTarget,
		token_symbol: &str,
		chain_type: ChainType,
		shard_size: u16,
		shard_threshold: u16,
	) -> Result<ChainSpec, String> {
		// Determine name from identifier
		let name = id.to_case(Case::Title);

		// Ensure wasm binary is available
		let wasm_binary = match runtime {
			RuntimeTarget::Mainnet => MAINNET_RUNTIME,
			RuntimeTarget::Testnet => TESTNET_RUNTIME,
		}
		.expect(
			"Development wasm binary is not available. This means the client is built with \
			 `SKIP_WASM_BUILD` flag and it is only usable for production chains. Please rebuild with \
			 the flag disabled.",
		);

		// Setup base currency unit name and decimal places
		let mut properties = sc_chain_spec::Properties::new();
		properties.insert("tokenSymbol".into(), token_symbol.into());
		properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
		properties.insert("ss58Format".into(), SS_58_FORMAT.into());

		// Add default telemetry for all deployed networks
		let telemetry = if chain_type != ChainType::Local {
			Some(
				TelemetryEndpoints::new(vec![(
					DEFAULT_TELEMETRY_URL.to_string(),
					DEFAULT_TELEMETRY_LEVEL,
				)])
				.expect("Default telemetry url is valid"),
			)
		} else {
			None
		};

		// Convert endowments in config according to token decimals
		let mut endowments = self
			.endowments
			.iter()
			.map(|(addr, bal)| (addr.clone(), bal * ANLOG))
			.collect::<Vec<_>>();

		// Budget and endow chronicle stashes
		let chronicle_supply = self.chronicles.len() as u128 * PER_CHRONICLE_STASH;
		endowments.append(
			&mut self
				.chronicles
				.iter()
				.map(|x| (x.clone(), PER_CHRONICLE_STASH))
				.collect::<Vec<_>>(),
		);

		// Endow controller if necessary
		let mut controller_supply = 0u128;
		if let Some(controller) = self.controller.as_ref() {
			controller_supply = CONTROLLER_SUPPLY;
			endowments.append(&mut vec![(controller.clone(), CONTROLLER_SUPPLY)]);
		}

		// Budget and endow council stashes
		let council_supply = self.councils.len() as u128 * PER_COUNCIL_STASH;
		endowments.append(
			&mut self.councils.iter().map(|x| (x.clone(), PER_COUNCIL_STASH)).collect::<Vec<_>>(),
		);

		// Budget and endow nominator stashes
		let nominator_supply = self.nominators.len() as u128 * PER_NOMINATOR_STASH;
		endowments.append(
			&mut self
				.nominators
				.iter()
				.map(|x| (x.clone(), PER_NOMINATOR_STASH))
				.collect::<Vec<_>>(),
		);

		// Budget and endow validator stashes
		let stake_supply = self.stakes.len() as u128 * PER_VALIDATOR_STASH;
		endowments.append(
			&mut self.stakes.iter().map(|x| (x.clone(), PER_VALIDATOR_STASH)).collect::<Vec<_>>(),
		);

		// Endow sudo account
		endowments.append(&mut vec![(self.sudo.clone(), SUDO_SUPPLY)]);

		// Add simulated supplies
		endowments.append(&mut vec![
			(
				hex!["0062466de473bc2686173eed44f49b282bf1615f4287ce8566aeaa5747a70855"].into(),
				SEED_ROUND_SUPPLY,
			),
			(
				hex!["5e489fd2dfc7dceb07c2f767d3e81928378330c2cef4dd58eb184582cc56d649"].into(),
				INITIAL_PRIVATE_SALE,
			),
			(
				hex!["1645738c66053277fdbcf04631805a7392ce23b043dc60862d8af09a329f0a79"].into(),
				PRIVATE_SALE,
			),
			(
				hex!["588de6ea1b423e0fc41995525a1fd63f50ec1e0c0b9bcc8192eb766eb85fce2f"].into(),
				PUBLIC_SALE,
			),
			(
				hex!["62e926d7df56786c766af140cdc9da839c50e60fa0d6722488a1ad235f1c5d1a"].into(),
				TEAM_SUPPLY - SUDO_SUPPLY - controller_supply - council_supply,
			),
			(
				hex!["ca6b881965b230aa52153c972ca0dc3dd0fa0a7453c00b62dec3532716fcd92d"].into(),
				TREASURY_SUPPLY,
			),
			(
				hex!["f612a8386a524dc0159463e5b2d01624d1730603fac6a5a1191aa32569138c4c"].into(),
				COMMUNITY_SUPPLY - stake_supply - nominator_supply - chronicle_supply,
			),
		]);

		// Load session keys to bootstrap validators from file
		let authorities: Vec<_> = self
			.bootstraps
			.iter()
			.enumerate()
			.map(|(i, x)| {
				(
					self.controller.clone().unwrap_or(self.stakes[i].clone()),
					self.stakes[i].clone(),
					testnet_runtime::SessionKeys {
						babe: x.0.clone(),
						grandpa: x.1.clone(),
						im_online: x.2.clone(),
						authority_discovery: x.3.clone(),
					},
				)
			})
			.collect();

		// Self-stake all authorities
		let locked = PER_VALIDATOR_STASH - PER_VALIDATOR_UNLOCKED;
		let stakers = authorities
			.iter()
			.map(|x| (x.1.clone(), x.0.clone(), locked, StakerStatus::<AccountId>::Validator))
			.collect::<Vec<_>>();

		let mut genesis_patch = serde_json::json!({
			"balances": {
				"balances": endowments,
			},
			"babe": {
				"epochConfig": runtime_common::BABE_GENESIS_EPOCH_CONFIG,
			},
			"elections": {
				"shardSize": shard_size,
				"shardThreshold": shard_threshold,
			},
			"networks": {
				"networks": [],
			},
			"session": {
				"keys": authorities,
			},
			"staking": {
				"validatorCount": authorities.len() as u32,
				"minimumValidatorCount": MIN_VALIDATOR_COUNT,
				"invulnerables": authorities.iter().map(|x| x.1.clone()).collect::<Vec<_>>(),
				"slashRewardFraction": Perbill::from_percent(10),
				"stakers": stakers
			},
		});

		let target_patch = match runtime {
			RuntimeTarget::Mainnet => serde_json::json!({
				"technicalCommittee": {
					"members": Some(self.sudo.clone()).iter().chain(self.councils.iter()).collect::<Vec<_>>(),
				},
			}),
			RuntimeTarget::Testnet => serde_json::json!({
				"sudo": {
					"key": Some(self.sudo.clone()),
				},
			}),
		};

		json_merge(&mut genesis_patch, target_patch);

		// Put it all together ...
		let mut builder = ChainSpec::builder(wasm_binary, Default::default())
			.with_name(&name)
			.with_id(id)
			.with_protocol_id(id)
			.with_chain_type(chain_type)
			.with_properties(properties)
			.with_genesis_config_patch(genesis_patch);

		// ... and add optional telemetry
		if let Some(endpoints) = telemetry {
			builder = builder.with_telemetry_endpoints(endpoints);
		}

		// ... to generate chain spec
		Ok(builder.build())
	}
}

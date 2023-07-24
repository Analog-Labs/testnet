use hex_literal::hex;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use runtime_common::currency::{Balance, ANLOG, TOKEN_DECIMALS};
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::crypto::UncheckedInto;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};
use timechain_runtime::{
	AccountId, BalancesConfig, CouncilConfig, GrandpaConfig, ImOnlineConfig,
	RuntimeGenesisConfig as GenesisConfig, Signature, StakerStatus, StakingConfig, SudoConfig,
	SystemConfig, VestingConfig, WASM_BINARY,
};
const TOKEN_SYMBOL: &str = "ANLOG";
const SS_58_FORMAT: u32 = 51;

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
const VALIDATOR_SUPPLY: Balance = ANLOG;

/// TODO: Incoperat into tokenomics
const CONTROLLER_SUPPLY: Balance = ANLOG * 100000;
const PER_VALIDATOR_STASH: Balance = ANLOG * 500000;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{seed}"), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(s),
		get_account_id_from_seed::<sr25519::Public>(&format!("{s}//stash")),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

/// Generate a chain spec for testnet deployment
pub fn analog_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Analog live wasm not available".to_string())?;

	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS_58_FORMAT.into());

	Ok(ChainSpec::from_genesis(
		// Name
		"Analog Testnet",
		// ID
		"analog_testnet",
		ChainType::Live,
		move || {
			generate_analog_genesis(
				wasm_binary,
				// Sudo account
				hex!["1260c29b59a365f07ac449e109cdf8f95905296af0707db9f3da0254e5db5741"].into(),
				// Initial authorities at genesis
				vec![
					// node 0
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["de6504921718b9b63c8318b4dda71f9d1678b0b16d4d44e7d835025b4985e64f"].into(),

						hex!["ccffb4cbe33cf1071bf8d235f707296dae01eb6636e214301268fac4c4361837"].unchecked_into(),
						hex!["879e699449e87508c43cbf95aa244baca54cb559367492d0ea27476fb1fc27c2"].unchecked_into(),
						hex!["244dc5109cdbae2e8b03f614ab3d43b49e4eec6bff9b895c0a9f80ee042b8557"].unchecked_into(),
					),
					// node 1
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["54e2b1342934a6cd71be5dd0ef9750281e445ab05732d2bc1d9513172995f266"].into(),

						hex!["1e265bd92a87f6a51d42277b09e7512868360aa922c582a2b75fbb992e2c2e05"].unchecked_into(),
						hex!["ad6454d9fdef5eb07ffa349298689e102ec355f1a4976f77c4506907d912ca99"].unchecked_into(),
						hex!["86f67d7dc73d0642bc558c95a7d4e1cf11519492163c41c81f7702ec073c445a"].unchecked_into(),
					),
					// node 2
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["465c937bdd92e75c55b80d4fb0aedf6908120ef008da684acd30b2f6cbdc1c2d"].into(),

						hex!["708d91506125a95b43cc692aca4c96bbe9760afd61fbb04f439c0efe8737cb2f"].unchecked_into(),
						hex!["61b0bc1acb9c6a53bbb5ac4e37033cb6d52ca63b5746888fb3c201ea96d49bdd"].unchecked_into(),
						hex!["b291c1da6ee11bd92e3342d6855f57b83d1e23577646eae2d0cd3a86d2d94879"].unchecked_into(),
					),
					// node 3
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["cea3095753273e30bd53eeec4c25e2b0a7418d9d55b94ec5e618d9ab45f7146e"].into(),

						hex!["26c9baaf0fd8ee768022a482ec06c36e24bb11b250f91d7d861526d6597a6f2d"].unchecked_into(),
						hex!["a7a59201cef948496438098942816e6ff6f53676d48d65bb2f1b992cf2d6fec3"].unchecked_into(),
						hex!["fce1ce2c0914f046dbe2a4dea2b0d784f9a6d8bd8090a8731f93d8cb3116265f"].unchecked_into(),
					),
					// node 4
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["b25c51bf9dd08a7348be3deb180e9bd1341a8dbaa53fc36b7375ab8fcac8dc5a"].into(),

						hex!["a48c9cd02b71690436d2485bd730158cabf5cc5b7c59c4368aebfa8d7cc4f76d"].unchecked_into(),
						hex!["0fb6ad09831794bbc64e8cef209463d7159aa81a02a3d78dbdd5fe91c749a46e"].unchecked_into(),
						hex!["d0e166e09b8cdd7a6d40c71b8073284ac06164145b5495d0f6f30b82a02bb31d"].unchecked_into(),
					),
					// node 5
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["7a9be08e61d66d8fb6c6e817bbdbf30d0fdb2dbfa05e6fcd42fd49e17a4dd601"].into(),

						hex!["e42790a8a48ea9039ba889ff7b5e1a24d91723daf8d38e48d0b3c696bd4a6408"].unchecked_into(),
						hex!["b24cb2cde027aec16ebc632bdb0884d68111fa95684a7ce371e8df2f0ad7938f"].unchecked_into(),
						hex!["50b77fadc0610f6afd54226b7952bc531a47de35dec3ec421de304551c37234a"].unchecked_into(),
					),
					// node 6
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["ec7b17f51793ba2f2fc649478d9c23cd2bf188343d9244c549b940c95a51073d"].into(),

						hex!["722b0f80be44f733710665f025700e0b0a7e42e493a18ea050488a43facb3119"].unchecked_into(),
						hex!["3443557788e9bdcfeeed48d8db79660015ca08783cada641f70e04c6e56029f7"].unchecked_into(),
						hex!["ded4aee3aa695886f7b016614277ff52d6c6577e8a1b3f369d691882a5b1b657"].unchecked_into(),
					),
					// node 7
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["64134b37e02b8ffd13b770dde18486da8a6c9f70102c5231ea48bca69be4e106"].into(),

						hex!["b045ca47691a3ba41e7e21586044ba2baf14763e58e88c7265a66eb07c39d812"].unchecked_into(),
						hex!["139fb056264d1989dbcba7e0de8b833b83d8e9be4e0dd0095171fd2af6c8a0fb"].unchecked_into(),
						hex!["10db2109855881f6ec81146d31a41166f2aa7d5e0dd0947e3a372225ff5f600e"].unchecked_into(),
					),
					// node 8
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["5862ee9d938616cfcf2bd52b42b72ab448ffd2fbf05e29fa74999eddbf2c6b13"].into(),

						hex!["56f32e0b020aa96d47f1741dcee91d06c57bfd6aa5510862d96aec2fb144a009"].unchecked_into(),
						hex!["e6320be8bb3329ca1ded528b9ac88c2f1f50cae18b711de44e4c17007ac558c7"].unchecked_into(),
						hex!["4279544e6763c7d6cf7ca1393fc17f8f9317a186a2e0c3e9c076cc0963c7044a"].unchecked_into(),
					),
					// node 9
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["4e6f9e4f7270fa92b3874618489bc4dbb7e13368481e41233853b4317c9d9831"].into(),

						hex!["a0c2baf89725680c35da2a3bc9c21090bed136229405113c42744c3f049de85b"].unchecked_into(),
						hex!["5df818825438f2d039f89b8d00e8bcef18199e94ecaf91e5567653972b21fbcc"].unchecked_into(),
						hex!["e662001cee21b418a8b22975621137183e7d17536114d98c3c694cbbc1cd8950"].unchecked_into(),
					),
					// node 10
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["40c1ed1f8ecf374e3dc7fa034091684919f42d7df1fb0031da41bb52ad295c7f"].into(),

						hex!["187674e20a50dde97f2dcb079b17fd338f61aa4e5033e0123a49dfd44d01f957"].unchecked_into(),
						hex!["15c31b81c06635c035065217c94f3d9edc8fbe150ad0cbda459c38e082aeb9e8"].unchecked_into(),
						hex!["22c93ab4b7231bbc3347525cd397bd5c3c84270281aefcad689829fdde1ad912"].unchecked_into(),
					),
					// node 11
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"].into(),
						hex!["368a2abb02892c86d9b683826678da60449f8f94c2f3ef1e43f356865618dd20"].into(),

						hex!["40cba8652980e686b3108a49e76be791c68cbbf5dc6fd7ae0197c53cea4dae0e"].unchecked_into(),
						hex!["898e0d42621a21aca94b7786e1845d7d597481cff038de7f96d76c9a8844619e"].unchecked_into(),
						hex!["e2c37a223378bd5032923fb9a94c4738168dbbe8df647c4191021bed07e39b6a"].unchecked_into(),
					),
				],
				// Pre-funded accounts
				vec![
					// Controller stashes
					(
						hex!["b4a1be5fbb1a1be77cfc4825c2d362cacf998aae252f2952d282369a0ac37b79"]
							.into(),
						CONTROLLER_SUPPLY
					),

					// Validator stashes
					(
						hex!["de6504921718b9b63c8318b4dda71f9d1678b0b16d4d44e7d835025b4985e64f"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["54e2b1342934a6cd71be5dd0ef9750281e445ab05732d2bc1d9513172995f266"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["465c937bdd92e75c55b80d4fb0aedf6908120ef008da684acd30b2f6cbdc1c2d"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["cea3095753273e30bd53eeec4c25e2b0a7418d9d55b94ec5e618d9ab45f7146e"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["b25c51bf9dd08a7348be3deb180e9bd1341a8dbaa53fc36b7375ab8fcac8dc5a"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["7a9be08e61d66d8fb6c6e817bbdbf30d0fdb2dbfa05e6fcd42fd49e17a4dd601"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["ec7b17f51793ba2f2fc649478d9c23cd2bf188343d9244c549b940c95a51073d"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["64134b37e02b8ffd13b770dde18486da8a6c9f70102c5231ea48bca69be4e106"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["5862ee9d938616cfcf2bd52b42b72ab448ffd2fbf05e29fa74999eddbf2c6b13"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["4e6f9e4f7270fa92b3874618489bc4dbb7e13368481e41233853b4317c9d9831"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["40c1ed1f8ecf374e3dc7fa034091684919f42d7df1fb0031da41bb52ad295c7f"]
							.into(),
						PER_VALIDATOR_STASH
					),
					(
						hex!["368a2abb02892c86d9b683826678da60449f8f94c2f3ef1e43f356865618dd20"]
							.into(),
						PER_VALIDATOR_STASH
					),

					// Tokenomics and supply
					(
						hex!["0062466de473bc2686173eed44f49b282bf1615f4287ce8566aeaa5747a70855"]
							.into(),
						SEED_ROUND_SUPPLY,
					),
					(
						hex!["5e489fd2dfc7dceb07c2f767d3e81928378330c2cef4dd58eb184582cc56d649"]
							.into(),
						INITIAL_PRIVATE_SALE,
					),
					(
						hex!["1645738c66053277fdbcf04631805a7392ce23b043dc60862d8af09a329f0a79"]
							.into(),
						PRIVATE_SALE,
					),
					(
						hex!["588de6ea1b423e0fc41995525a1fd63f50ec1e0c0b9bcc8192eb766eb85fce2f"]
							.into(),
						PUBLIC_SALE,
					),
					(
						hex!["62e926d7df56786c766af140cdc9da839c50e60fa0d6722488a1ad235f1c5d1a"]
							.into(),
						TEAM_SUPPLY,
					),
					(
						hex!["ca6b881965b230aa52153c972ca0dc3dd0fa0a7453c00b62dec3532716fcd92d"]
							.into(),
						TREASURY_SUPPLY,
					),
					(
						hex!["f612a8386a524dc0159463e5b2d01624d1730603fac6a5a1191aa32569138c4c"]
							.into(),
						COMMUNITY_SUPPLY,
					),
					(
						hex!["66c8567013bfc1ca7e4568b2d7ed07f775614ac6a68824a8a309425b2dcd930b"]
							.into(),
						VALIDATOR_SUPPLY,
					),
				],
			)
		},
		// Bootnodes
		vec![
			"/dns/bootnode-1.internal.analog.one/tcp/30333/p2p/12D3KooWHRZcA2GHQYpbqwPsvk3ZEPDnu35w7cfEBxYPFfTR2bHX".parse().unwrap(),
			"/dns/bootnode-2.internal.analog.one/tcp/30333/p2p/12D3KooWAHTG5KqRPKyerDVXAVrGEXd3g1XDK9JajbTCZP2K7xVN".parse().unwrap(),
		],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

/// Generate a chain spec for Analog staging environment.
pub fn analog_staging_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS_58_FORMAT.into());
	Ok(ChainSpec::from_genesis(
		// Name
		"Analog Staging",
		// ID
		"analog_staging",
		ChainType::Development,
		move || {
			generate_analog_genesis(
				wasm_binary,
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Pre-funded accounts
				vec![
					(get_account_id_from_seed::<sr25519::Public>("Alice"), ANLOG * 2000000),
					(get_account_id_from_seed::<sr25519::Public>("Bob"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Alice//stash"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Bob//stash"), ANLOG * 10000000),
					(
						hex!["88fd77d706e168d78713a6a927c1ddfae367b081fb2829b119bbcc6db9af401d"]
							.into(),
						SEED_ROUND_SUPPLY,
					),
					(
						hex!["04063fc1cbba917ced6c45091bf631de6a4db584dd55c1d67431661a5d57a575"]
							.into(),
						INITIAL_PRIVATE_SALE,
					),
					(
						hex!["cc5245e57dcf6c8f051e012beceaa1683578ae873223d3ef4f8cbd85a62e1536"]
							.into(),
						PRIVATE_SALE,
					),
					(
						hex!["2af7c08133177cc462171389578174b89758ca09c5f93235409594f15f65ac63"]
							.into(),
						PUBLIC_SALE,
					),
					(
						hex!["f6855b0ec40cc91c49025d75aa65a1965861cde56451da99170bd4dae13dab35"]
							.into(),
						TEAM_SUPPLY,
					),
					(
						hex!["e0dc12faf7e650b910638e934b4ef9aea1410707312bd8d80ec91123acb02747"]
							.into(),
						TREASURY_SUPPLY,
					),
					(
						hex!["685a09abdd4c4fe57730fb4eb5fbe6e18e9cca90a2124c5e60ad927278cfd36c"]
							.into(),
						COMMUNITY_SUPPLY,
					),
					(
						hex!["088f0e5d722a420339685a4e6ab358a4df4e39206bfad00e30617abf1633d37a"]
							.into(),
						VALIDATOR_SUPPLY,
					),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

/// Generate a chain spec for local developement and testing.
pub fn analog_dev_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS_58_FORMAT.into());

	Ok(ChainSpec::from_genesis(
		// Name
		"Analog Local",
		// ID
		"analog_local",
		ChainType::Local,
		move || {
			generate_analog_genesis(
				wasm_binary,
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
					authority_keys_from_seed("Charlie"),
					authority_keys_from_seed("Dave"),
					authority_keys_from_seed("Eve"),
					authority_keys_from_seed("Ferdie"),
					// Just enable six accounts for local testing
					// Reserve following accounts for quickly create shard
					// authority_keys_from_seed("Henry"),
					// authority_keys_from_seed("Ivan"),
					// authority_keys_from_seed("Jack"),
					// authority_keys_from_seed("Lisa"),
					// authority_keys_from_seed("Mona"),
					// authority_keys_from_seed("Nash"),
				],
				// Pre-funded accounts
				vec![
					// TODO remove the 1_000_000_000 after tokenomics issue fixed
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						ANLOG * 2000000 * 1_000_000_000,
					),
					(get_account_id_from_seed::<sr25519::Public>("Bob"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Charlie"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Dave"), ANLOG * 10000000),
					(get_account_id_from_seed::<sr25519::Public>("Eve"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Ferdie"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Henry"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Ivan"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Jack"), ANLOG * 10000000),
					(get_account_id_from_seed::<sr25519::Public>("Lisa"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Mona"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Nash"), ANLOG * 1000000),
					// TODO remove the 1_000_000_000 after tokenomics issue fixed
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						ANLOG * 1000000 * 1_000_000_000,
					),
					(
						// TODO remove the 1_000_000_000 after tokenomics issue fixed
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						ANLOG * 1000000 * 1_000_000_000,
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						ANLOG * 1000000,
					),
					(get_account_id_from_seed::<sr25519::Public>("Dave//stash"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Eve//stash"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Henry//stash"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Ivan//stash"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Jack//stash"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Lisa//stash"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Mona//stash"), ANLOG * 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Nash//stash"), ANLOG * 1000000),
				],
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		Some(properties),
		// Extensions
		None,
	))
}

/// Helper to generate genesis storage state.
fn generate_analog_genesis(
	wasm_binary: &[u8],
	root_key: AccountId,
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	endowed_accounts: Vec<(AccountId, Balance)>,
) -> GenesisConfig {
	type BlockNumer = u32;
	type NoOfVest = u32;

	// 	3 months in terms of 6s blocks is 1,296,000 blocks, i.e. period = 1,296,000
	// 	THREE_MONTHS: u32 = 1_296_000; // We are approximating a month to 30 days.
	// 	ONE_MONTH: u32 = 432_000; // 30 days from block 0, implies 432_000 blocks
	let vesting_accounts_json = &include_bytes!("../../resources/anlog_vesting.json")[..];
	// configure not valid for these vesting accounts.
	let vesting_accounts: Vec<(AccountId, BlockNumer, BlockNumer, NoOfVest, Balance)> =
		serde_json::from_slice(vesting_accounts_json)
			.expect("The file vesting_test.json is not exist or not having valid data.");
	let initial_nominators: Vec<AccountId> = vec![];
	let stash = ANLOG * 500000;
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.1.clone(), x.0.clone(), stash, StakerStatus::<AccountId>::Validator))
		.chain(initial_nominators.iter().map(|x| {
			let nominations = initial_authorities
				.as_slice()
				.iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), stash, StakerStatus::<AccountId>::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure pool accounts with its initial supply.
			balances: endowed_accounts,
		},
		babe: timechain_runtime::BabeConfig {
			authorities: vec![],
			epoch_config: Some(timechain_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: GrandpaConfig { authorities: vec![] },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		im_online: ImOnlineConfig { keys: vec![] },
		session: timechain_runtime::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						timechain_runtime::opaque::SessionKeys {
							babe: x.2.clone(),
							grandpa: x.3.clone(),
							im_online: x.4.clone(),
						},
					)
				})
				.collect::<Vec<_>>(),
		},

		// staking: Default::default(),
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			// TODO: ForceEra::ForceNone
			..Default::default()
		},
		vesting: VestingConfig { vesting: vesting_accounts },
		treasury: Default::default(),
		council: CouncilConfig::default(),
	}
}

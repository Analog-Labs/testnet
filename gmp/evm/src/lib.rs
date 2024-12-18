use alloy_primitives::{B256, U256};
use alloy_sol_types::{SolCall, SolConstructor, SolEvent, SolValue};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::Stream;
use rosetta_client::{
	query::GetLogs, types::AccountIdentifier, AtBlock, CallResult, FilterBlockOption,
	GetTransactionCount, Signer, SubmitResult, TransactionReceipt, Wallet,
};
use rosetta_crypto::{bip44::ChildNumber, SecretKey};
use rosetta_ethereum_backend::{jsonrpsee::Adapter, EthereumRpc};
use rosetta_server::ws::{default_client, DefaultClient};
use rosetta_server_ethereum::utils::{
	DefaultFeeEstimatorConfig, EthereumRpcExt, PolygonFeeEstimatorConfig,
};
use serde::Deserialize;
use sha3::{Digest, Keccak256};
use sol::{u256, Network, TssKey};
use std::ops::Range;
use std::pin::Pin;
use std::sync::Arc;
use time_primitives::{
	Address, BatchId, ConnectorParams, Gateway, GatewayMessage, GmpEvent, GmpEvent, GmpMessage,
	IChain, IConnector, IConnectorAdmin, IConnectorBuilder, MessageId, NetworkId, Route,
	TssPublicKey, TssSignature, H160,
};

use crate::sol::{ProxyContext, ProxyDigest};

type AlloyAddress = alloy_primitives::Address;

pub(crate) mod sol;

fn a_addr(address: Address) -> AlloyAddress {
	let address: [u8; 20] = address[12..32].try_into().unwrap();
	AlloyAddress::from(address)
}

fn t_addr(address: alloy_primitives::Address) -> Address {
	let mut addr = [0; 32];
	addr[12..32].copy_from_slice(&address.0[..]);
	addr
}

#[derive(Clone)]
pub struct Connector {
	network_id: NetworkId,
	wallet: Arc<Wallet>,
	backend: Adapter<DefaultClient>,
}

impl Connector {
	#[allow(dead_code)]
	async fn nonce(&self, address: Address) -> Result<u64> {
		let address: [u8; 20] = address[12..32].try_into().unwrap();
		self.wallet
			.query(GetTransactionCount {
				address: address.into(),
				block: AtBlock::Latest,
			})
			.await
	}

	async fn evm_call<T: SolCall>(
		&self,
		contract: Address,
		call: T,
		amount: u128,
		nonce: Option<u64>,
	) -> Result<(T::Return, TransactionReceipt, [u8; 32])> {
		let contract: [u8; 20] = contract[12..32].try_into().unwrap();
		let result = self
			.wallet
			.eth_send_call(contract, call.abi_encode(), amount, nonce, None)
			.await?;
		let SubmitResult::Executed {
			result: CallResult::Success(result),
			receipt,
			tx_hash,
		} = result
		else {
			anyhow::bail!("{:?}", result)
		};
		tracing::info!("evm_call success: {:?}", tx_hash);
		Ok((T::abi_decode_returns(&result, true)?, receipt, tx_hash.into()))
	}

	async fn evm_view<T: SolCall>(
		&self,
		contract: Address,
		call: T,
		block: Option<u64>,
	) -> Result<T::Return> {
		let contract: [u8; 20] = contract[12..32].try_into().unwrap();
		let block: AtBlock = if let Some(block) = block { block.into() } else { AtBlock::Latest };
		let result = self.wallet.eth_view_call(contract, call.abi_encode(), block).await?;
		let CallResult::Success(result) = result else { anyhow::bail!("{:?}", result) };
		Ok(T::abi_decode_returns(&result, true)?)
	}

	async fn deploy_contract(
		&self,
		abi: &[u8],
		constructor: impl SolConstructor,
	) -> Result<(Address, u64)> {
		let mut contract = get_contract_from_slice(abi)?;
		contract.extend(constructor.abi_encode());
		let tx_hash = self.wallet.eth_deploy_contract(contract).await?.tx_hash().0;
		let tx_receipt = self.wallet.eth_transaction_receipt(tx_hash).await?.unwrap();
		let address = tx_receipt.contract_address.unwrap();
		let block_number = tx_receipt.block_number.unwrap();
		Ok((t_addr(address.0.into()), block_number))
	}

	///
	/// init_code == contract_bytecode + contractor_code
	async fn deploy_contract_with_factory(
		&self,
		factory_address: [u8; 20],
		call: Vec<u8>,
	) -> Result<(AlloyAddress, u64)> {
		let result = self
			.wallet
			.eth_send_call(factory_address, call, 0, None, Some(20_000_000))
			.await?;
		let SubmitResult::Executed { result, receipt, tx_hash } = result else {
			anyhow::bail!("tx timed out");
		};
		match result {
			CallResult::Success(_) => {
				let log = receipt
					.logs
					.iter()
					.find(|log| log.address.as_bytes() == factory_address)
					.with_context(|| format!("Log with factory address not found: {}", tx_hash))?;
				let topic = log
					.topics
					.first()
					.with_context(|| "Unable to find topics in tx receipt")?
					.as_bytes();
				let contract_address = AlloyAddress::from_slice(&topic[12..]);
				Ok((contract_address, receipt.block_number.unwrap()))
			},
			CallResult::Revert(reason) => {
				anyhow::bail!("Deployment reverted: {tx_hash:?}: {:?}", hex::encode(reason))
			},
			CallResult::Error => anyhow::bail!("Failed to deploy contract"),
		}
	}

	async fn deploy_factory(&self, config: &DeploymentConfig) -> Result<()> {
		let deployer_address = self.parse_address(&config.factory_deployer)?;

		// Step1: fund 0x908064dE91a32edaC91393FEc3308E6624b85941
		self.faucet().await?;
		tracing::info!("wallet balance: {:?}", self.balance(self.address()).await);
		self.transfer(deployer_address, config.required_balance).await?;

		//Step2: load transaction from config
		let tx = hex::decode(config.raw_tx.strip_prefix("0x").unwrap_or(&config.raw_tx))?;

		//Step3: send eth_rawTransaction
		let tx_hash = self.backend.send_raw_transaction(tx.into()).await?;

		tracing::info!("Deployed factory with hash: {:?}", tx_hash);
		Ok(())
	}

	async fn deploy_gateway(
		&self,
		config: &DeploymentConfig,
		proxy_addr: [u8; 20],
		gateway: &[u8],
	) -> Result<AlloyAddress> {
		let gateway_bytecode = get_contract_from_slice(gateway)?;

		let gateway_constructor = sol::Gateway::constructorCall {
			network: self.network_id,
			proxy: proxy_addr.into(),
		};

		let gateway_init_code =
			extend_bytes_with_constructor(gateway_bytecode, gateway_constructor);

		// deploy using universal factory
		let gateway_create_call = sol::IUniversalFactory::create2_0Call {
			salt: config.deployment_salt.into(),
			creationCode: gateway_init_code.into(),
		}
		.abi_encode();

		tracing::info!("Deploying gateway ...");

		let factory_address = a_addr(self.parse_address(&config.factory_address)?).0 .0;
		let (gateway_address, _) =
			self.deploy_contract_with_factory(factory_address, gateway_create_call).await?;
		tracing::info!("gateway deployed at: {:?}", gateway_address);
		Ok(gateway_address)
	}

	fn compute_proxy_address(
		&self,
		config: &DeploymentConfig,
		admin: AlloyAddress,
		proxy: &[u8],
	) -> Result<[u8; 20]> {
		let factory_address = a_addr(self.parse_address(&config.factory_address)?).0 .0;
		let proxy_constructor = sol::GatewayProxy::constructorCall { admin };
		let proxy_bytecode = get_contract_from_slice(proxy)?;

		// proxy CreationCode
		let proxy_bytecode =
			extend_bytes_with_constructor(proxy_bytecode.clone(), proxy_constructor);

		let computed_proxy_address = compute_create2_address(
			factory_address,
			config.deployment_salt,
			proxy_bytecode.clone(),
		)?;

		Ok(computed_proxy_address)
	}

	fn get_proxy_admin_creds(&self, config: &DeploymentConfig) -> Result<(SecretKey, [u8; 20])> {
		let signer = Signer::new(&config.proxy_admin_sk.parse()?, "")?;
		let proxy_admin_sk = signer
			.bip44_account(self.wallet.config().algorithm, self.wallet.config().coin, 0)?
			.derive(ChildNumber::non_hardened_from_u32(0))?;

		let proxy_admin_pk = proxy_admin_sk
			.public_key()
			.to_address(rosetta_crypto::address::AddressFormat::Eip55);

		let proxy_admin_address = a_addr(self.parse_address(proxy_admin_pk.address())?).0 .0;
		let proxy_admin_sk = proxy_admin_sk.secret_key();
		Ok((proxy_admin_sk.clone(), proxy_admin_address))
	}

	async fn deploy_proxy(
		&self,
		config: &DeploymentConfig,
		proxy_addr: AlloyAddress,
		gateway_address: AlloyAddress,
		proxy_bytes: &[u8],
		admin_sk: SecretKey,
		admin: AlloyAddress,
	) -> Result<(AlloyAddress, u64)> {
		let factory_address = a_addr(self.parse_address(&config.factory_address)?).0 .0;
		let deployment_salt = config.deployment_salt;

		// constructor params
		let proxy_constructor = sol::GatewayProxy::constructorCall { admin };
		let proxy_bytecode = get_contract_from_slice(proxy_bytes)?;
		let proxy_bytecode =
			extend_bytes_with_constructor(proxy_bytecode.clone(), proxy_constructor);

		// computing signature for security purpose
		let digest = ProxyDigest {
			proxy: proxy_addr,
			implementation: gateway_address,
		}
		.abi_encode();
		let payload: [u8; 32] = Keccak256::digest(digest).into();
		let signature = admin_sk.sign_prehashed(&payload)?;
		let (v, r, s) = extract_signature_bytes(signature.to_bytes())?;
		let arguments = ProxyContext {
			// Ethereum verification uses 27,28 instead of 0,1 for recovery id
			v: v + 27,
			r: r.into(),
			s: s.into(),
			implementation: gateway_address,
		}
		.abi_encode();

		let initializer = self.get_initializer(admin)?;

		// Proxy creation
		let proxy_init_call = sol::IUniversalFactory::create2_1Call {
			salt: deployment_salt.into(),
			creationCode: proxy_bytecode.into(),
			arguments: arguments.into(),
			callback: initializer.into(),
		}
		.abi_encode();

		tracing::info!("Deploying proxy ...");
		let (proxy_address, block) =
			self.deploy_contract_with_factory(factory_address, proxy_init_call).await?;

		if proxy_address != proxy_addr {
			anyhow::bail!(
				"Unable to compute proxy address: expected: {:?}, got {:?}",
				proxy_addr,
				proxy_address
			);
		}
		tracing::info!("Proxy deployed at: {}", proxy_address);
		Ok((proxy_address, block))
	}

	fn get_initializer(&self, admin: AlloyAddress) -> Result<Vec<u8>> {
		let keys: Vec<TssKey> = vec![];
		let networks: Vec<Network> = vec![];
		let initializer = sol::Gateway::initializeCall { admin, keys, networks }.abi_encode();
		Ok(initializer)
	}
}

#[async_trait]
impl IConnectorBuilder for Connector {
	/// Creates a new connector.
	async fn new(params: ConnectorParams) -> Result<Self>
	where
		Self: Sized,
	{
		let wallet = Arc::new(
			Wallet::new(
				params.blockchain.parse()?,
				&params.network,
				&params.url,
				&params.mnemonic,
				None,
			)
			.await?,
		);
		let client = default_client(&params.url, None)
			.await
			.with_context(|| "Cannot get ws client for url: {url}")?;
		let adapter = Adapter(client);
		let connector = Self {
			network_id: params.network_id,
			wallet,
			backend: adapter,
		};
		// TODO Discuss keyfile issue
		if connector.wallet.config().coin == 1 {
			//Local run fund wallet
			connector.faucet().await?;
		};
		Ok(connector)
	}
}

#[async_trait]
impl IChain for Connector {
	/// Formats an address into a string.
	fn format_address(&self, address: Address) -> String {
		a_addr(address).to_string()
	}
	/// Parses an address from a string.
	fn parse_address(&self, address: &str) -> Result<Address> {
		let address: AlloyAddress = address.parse()?;
		Ok(t_addr(address))
	}
	/// Network identifier.
	fn network_id(&self) -> NetworkId {
		self.network_id
	}
	/// Human readable connector account identifier.
	fn address(&self) -> Address {
		self.parse_address(&self.wallet.account().address).unwrap()
	}
	fn currency(&self) -> (u32, &str) {
		(18, "ETH")
	}
	/// Uses a faucet to fund the account when possible.
	async fn faucet(&self) -> Result<()> {
		let balance = match self.network_id() {
			6 => 10u128.pow(25), // astar
			3 => 10u128.pow(29), // ethereum
			network_id => {
				tracing::info!("network {network_id} doesn't support faucet");
				return Ok(());
			},
		};
		self.wallet.faucet(balance, None).await?;
		Ok(())
	}
	/// Transfers an amount to an account.
	async fn transfer(&self, address: Address, amount: u128) -> Result<()> {
		let address = self.format_address(address);
		self.wallet
			.transfer(&AccountIdentifier::new(address), amount, None, None)
			.await?;
		Ok(())
	}
	/// Queries the account balance.
	async fn balance(&self, address: Address) -> Result<u128> {
		self.wallet.balance(a_addr(address).to_string()).await
	}
	/// Stream of finalized block indexes.
	fn block_stream(&self) -> Pin<Box<dyn Stream<Item = u64> + Send>> {
		self.wallet.block_stream()
	}
}

#[async_trait]
impl IConnector for Connector {
	/// Reads gmp messages from the target chain.
	async fn read_events(&self, gateway: Gateway, blocks: Range<u64>) -> Result<Vec<GmpEvent>> {
		let contract: [u8; 20] = a_addr(gateway).0.into();
		let logs = self
			.wallet
			.query(GetLogs {
				contracts: vec![contract.into()],
				topics: vec![],
				block: FilterBlockOption::Range {
					from_block: Some(blocks.start.into()),
					to_block: Some(blocks.end.into()),
				},
			})
			.await?;
		let mut events = vec![];
		tracing::info!("gmp executed hash: {:?}", sol::Gateway::GmpExecuted::SIGNATURE_HASH);
		for log in logs {
			let topics = log.topics.iter().map(|topic| B256::from(topic.0)).collect::<Vec<_>>();
			let log =
				alloy_primitives::Log::new(a_addr(gateway), topics, log.data.0.to_vec().into())
					.ok_or_else(|| anyhow::format_err!("failed to decode log"))?;
			for topic in log.topics() {
				match *topic {
					sol::Gateway::ShardRegistered::SIGNATURE_HASH => {
						let log = sol::Gateway::ShardRegistered::decode_log(&log, true)?;
						events.push(GmpEvent::ShardRegistered(log.key.clone().into()));
						break;
					},
					sol::Gateway::ShardUnregistered::SIGNATURE_HASH => {
						let log = sol::Gateway::ShardUnregistered::decode_log(&log, true)?;
						events.push(GmpEvent::ShardUnregistered(log.key.clone().into()));
						break;
					},
					sol::Gateway::MessageReceived::SIGNATURE_HASH => {
						let log = sol::Gateway::MessageReceived::decode_log(&log, true)?;
						events.push(GmpEvent::MessageReceived(log.msg.clone().into()));
						break;
					},
					sol::Gateway::GmpCreated::SIGNATURE_HASH => {
						let log = sol::Gateway::GmpCreated::decode_log(&log, true)?;
						//TODO remove when fixed in gateway
						let mut source = log.source;
						source[11] = 0;
						let gmp_message = GmpMessage {
							src_network: self.network_id,
							dest_network: log.destinationNetwork,
							src: source.into(),
							dest: t_addr(log.destinationAddress),
							nonce: u64::try_from(log.salt)?,
							gas_limit: u128::try_from(log.executionGasLimit)?,
							// TODO compute gas cost here
							gas_cost: 200,
							bytes: log.data.data.into(),
						};
						tracing::info!(
							"read events msg: {:?}/{:?}",
							gmp_message,
							gmp_message.message_id()
						);
						events.push(GmpEvent::MessageReceived(gmp_message));
						break;
					},
					sol::Gateway::GmpExecuted::SIGNATURE_HASH => {
						let log = sol::Gateway::GmpExecuted::decode_log(&log, true)?;
						events.push(GmpEvent::MessageExecuted(log.id.into()));
						break;
					},
					sol::Gateway::BatchExecuted::SIGNATURE_HASH => {
						let log = sol::Gateway::BatchExecuted::decode_log(&log, true)?;
						events.push(GmpEvent::BatchExecuted(log.batch));
						break;
					},
					_ => {},
				}
				tracing::info!("logsiter 3");
			}
		}
		Ok(events)
	}
	/// Submits a gmp message to the target chain.
	async fn submit_commands(
		&self,
		gateway: Gateway,
		_batch: BatchId,
		msg: GatewayMessage,
		signer: TssPublicKey,
		sig: TssSignature,
	) -> Result<(), String> {
		// TODO this implementation is not what was designed but due to limitation of gateway we are sending single msg.
		let msg = msg.ops.get(0).ok_or_else(|| String::from("Invalid msg ops length"))?;
		let GatewayOp::SendMessage(msg) = msg else {
			return Err(String::from("Not valid type of GatewayOp"));
		};
		let signature = sol::Signature {
			xCoord: u256(&signer[1..33]),
			e: u256(&sig[..32]),
			s: u256(&sig[32..]),
		};
		let call = sol::Gateway::execute_1Call {
			signature,
			message: msg.clone().into(),
		};
		self.evm_call(gateway, call, 0, None).await.map_err(|err| err.to_string())?;
		Ok(())
	}
}

#[async_trait]
impl IConnectorAdmin for Connector {
	/// Deploys the gateway contract.
	async fn deploy_gateway(
		&self,
		additional_params: &[u8],
		proxy: &[u8],
		gateway: &[u8],
	) -> Result<(Address, u64)> {
		// check if uf already deployed
		let config: DeploymentConfig = serde_json::from_slice(additional_params)?;
		let factory_address = a_addr(self.parse_address(&config.factory_address)?).0 .0;
		let is_factory_deployed = self
			.backend
			.get_code(factory_address.into(), rosetta_ethereum_types::AtBlock::Latest)
			.await?;

		if is_factory_deployed.is_empty() {
			tracing::info!("Factory is not deployed");
			self.deploy_factory(&config).await?;
		} else {
			tracing::info!("Factory is deployed: {:?}", is_factory_deployed.len());
		}

		// proxy address computation
		let (admin_sk, admin) = self.get_proxy_admin_creds(&config)?;
		let proxy_addr = self.compute_proxy_address(&config, admin.into(), proxy)?;

		// check if proxy is deployed
		let is_proxy_deployed = self
			.backend
			.get_code(proxy_addr.into(), rosetta_ethereum_types::AtBlock::Latest)
			.await?;

		if !is_proxy_deployed.is_empty() {
			tracing::debug!("Proxy already deployed, Please upgrade the gateway contract");
			return Ok((t_addr(proxy_addr.into()), 0));
		}

		// gateway deployment
		let gateway_address = self.deploy_gateway(&config, proxy_addr, gateway).await?;

		// compute proxy arguments
		let (proxy_address, block) = self
			.deploy_proxy(
				&config,
				proxy_addr.into(),
				gateway_address,
				proxy,
				admin_sk,
				admin.into(),
			)
			.await?;

		Ok((t_addr(proxy_address), block))
	}

	/// Redeploys the gateway contract.
	async fn redeploy_gateway(
		&self,
		additional_params: &[u8],
		proxy: Address,
		gateway: &[u8],
	) -> Result<()> {
		let config: DeploymentConfig = serde_json::from_slice(additional_params)?;
		let gateway_addr = self.deploy_gateway(&config, a_addr(proxy).into(), gateway).await?;
		let call = sol::Gateway::upgradeCall {
			newImplementation: gateway_addr,
		};
		self.evm_call(proxy, call, 0, None).await?;
		Ok(())
	}
	/// Returns the gateway admin.
	async fn admin(&self, gateway: Address) -> Result<Address> {
		let result = self.evm_view(gateway, sol::Gateway::adminCall {}, None).await?;
		Ok(t_addr(result._0))
	}
	/// Sets the gateway admin.
	async fn set_admin(&self, gateway: Address, admin: Address) -> Result<()> {
		let call = sol::Gateway::setAdminCall { admin: a_addr(admin) };
		self.evm_call(gateway, call, 0, None).await?;
		Ok(())
	}
	/// Returns the registered shard keys.
	async fn shards(&self, gateway: Address) -> Result<Vec<TssPublicKey>> {
		let result = self.evm_view(gateway, sol::Gateway::shardsCall {}, None).await?;
		let keys = result._0.into_iter().map(Into::into).collect();
		Ok(keys)
	}
	/// Sets the registered shard keys. Overwrites any other keys.
	async fn set_shards(&self, gateway: Address, keys: &[TssPublicKey]) -> Result<()> {
		let mut shards = keys.iter().copied().map(Into::into).collect::<Vec<TssKey>>();
		shards.sort_by(|a, b| a.xCoord.cmp(&b.xCoord));
		let call = sol::Gateway::setShardsCall { publicKeys: shards };
		self.evm_call(gateway, call, 0, None).await?;
		Ok(())
	}
	/// Returns the gateway routing table.
	async fn routes(&self, gateway: Address) -> Result<Vec<Route>> {
		let result = self.evm_view(gateway, sol::Gateway::routesCall {}, None).await?;
		let networks = result._0.into_iter().map(Into::into).collect();
		Ok(networks)
	}
	/// Updates an entry in the gateway routing table.
	async fn set_route(&self, gateway: Address, route: Route) -> Result<()> {
		let call = sol::Gateway::setRouteCall { info: route.into() };
		self.evm_call(gateway, call, 0, None).await?;
		Ok(())
	}
	/// Estimates the message cost.
	async fn estimate_message_cost(
		&self,
		gateway: Address,
		dest: NetworkId,
		msg_size: usize,
		gas_limit: u128,
	) -> Result<u128> {
		let msg_size = U256::from_str_radix(&msg_size.to_string(), 16).unwrap();
		let call = sol::Gateway::estimateMessageCostCall {
			networkid: dest,
			messageSize: msg_size,
			gasLimit: U256::from(gas_limit),
		};
		let result = self.evm_view(gateway, call, None).await?;
		let msg_cost: u128 = result._0.try_into().unwrap();
		Ok(msg_cost)
	}
	/// Deploys a test contract.
	async fn deploy_test(&self, gateway: Address, tester: &[u8]) -> Result<(Address, u64)> {
		self.deploy_contract(tester, sol::GmpTester::constructorCall { gateway: a_addr(gateway) })
			.await
	}
	/// Sends a message using the test contract.
	async fn send_message(&self, contract: Address, msg: GmpMessage) -> Result<MessageId> {
		// EVM specific logic
		let mut modified_msg = msg.clone();
		modified_msg.gas_limit = 300_000;
		let sol_msg: sol::GmpMessage = modified_msg.clone().into();
		modified_msg.bytes = sol_msg.abi_encode();

		let cost_call = sol::GmpTester::estimateMessageCostCall {
			messageSize: U256::from(modified_msg.bytes.len()),
			gasLimit: U256::from(modified_msg.gas_limit),
		};

		let msg_cost = self.evm_view(contract, cost_call, None).await?;
		let msg_cost = u128::try_from(msg_cost._0)?;

		let call = sol::GmpTester::sendMessageCall {
			msg: modified_msg.clone().into(),
		};

		self.evm_call(contract, call, msg_cost, None).await?;
		let msg_id = modified_msg.message_id();
		Ok(msg_id)
	}
	/// Receives messages from test contract.
	async fn recv_messages(
		&self,
		contract: Address,
		blocks: Range<u64>,
	) -> Result<Vec<GmpMessage>> {
		let contract: [u8; 20] = a_addr(contract).0.into();
		let logs = self
			.wallet
			.query(GetLogs {
				contracts: vec![contract.into()],
				topics: vec![],
				block: FilterBlockOption::Range {
					from_block: Some(blocks.start.into()),
					to_block: Some(blocks.end.into()),
				},
			})
			.await?;
		let mut msgs = vec![];
		for log in logs {
			let topics = log.topics.iter().map(|topic| B256::from(topic.0)).collect::<Vec<_>>();
			let log =
				alloy_primitives::Log::new(contract.into(), topics, log.data.0.to_vec().into())
					.ok_or_else(|| anyhow::format_err!("failed to decode log"))?;
			for topic in log.topics() {
				let sol::GmpTester::MessageReceived::SIGNATURE_HASH = *topic else {
					continue;
				};
				let log = sol::GmpTester::MessageReceived::decode_log(&log, true)?;
				msgs.push(log.msg.clone().into());
			}
		}
		Ok(msgs)
	}
	/// Calculate transaction base fee for a chain.
	async fn transaction_base_fee(&self) -> Result<u128> {
		let fee_estimator = if self.wallet.config().blockchain == "polygon" {
			self.backend.estimate_eip1559_fees::<PolygonFeeEstimatorConfig>().await?
		} else {
			self.backend.estimate_eip1559_fees::<DefaultFeeEstimatorConfig>().await?
		};
		Ok(u128::try_from(fee_estimator.0)
			.map_err(|_| anyhow::anyhow!("Failed to convert value from U256 to u128"))?)
	}

	/// Returns gas limit of latest block.
	async fn block_gas_limit(&self) -> Result<u64> {
		let block = self
			.backend
			.block(AtBlock::Latest)
			.await?
			.with_context(|| "Cannot find latest block")?;
		Ok(block.header.gas_limit)
	}
	/// Withdraw gateway funds.
	async fn withdraw_funds(
		&self,
		gateway: Address,
		amount: u128,
		receipient: Address,
	) -> Result<()> {
		let call = sol::Gateway::withdrawCall {
			amount: U256::from(amount),
			recipient: a_addr(receipient),
			data: vec![].into(),
		};
		self.evm_call(gateway, call, 0, None).await?;
		Ok(())
	}
	/// Deposit gateway funds.
	async fn deposit_funds(&self, gateway: Address, amount: u128) -> Result<()> {
		let call = sol::Gateway::depositCall {};
		self.evm_call(gateway, call, amount, None).await?;
		Ok(())
	}
}

fn compute_create2_address(
	factory_address: [u8; 20],
	salt: [u8; 32],
	init_code: Vec<u8>,
) -> Result<[u8; 20]> {
	// solidity
	// bytes32 create2hash = keccak256(abi.encodePacked(uint8(0xff), address(factory), salt, initcodeHash));
	// return address(uint160(uint256(create2hash)));
	//
	use sha3::Digest;
	let init_code_hash: [u8; 32] = Keccak256::digest(init_code).into();
	let mut hasher = Keccak256::new();
	hasher.update([0xff]);
	hasher.update(factory_address);
	hasher.update(salt);
	hasher.update(init_code_hash);

	let proxy_hashed = hasher.finalize();

	Ok(AlloyAddress::from_slice(&proxy_hashed[12..]).0 .0)
}

fn get_contract_from_slice(slice: &[u8]) -> Result<Vec<u8>> {
	let contract_abi: Contract = serde_json::from_slice(slice)?;
	hex::decode(contract_abi.bytecode.object.replace("0x", ""))
		.with_context(|| "Failed to get contract bytecode")
}

fn extend_bytes_with_constructor(bytecode: Vec<u8>, constructor: impl SolConstructor) -> Vec<u8> {
	let mut bytecode = bytecode.clone();
	bytecode.extend(constructor.abi_encode());
	bytecode
}

pub fn extract_signature_bytes(sig: Vec<u8>) -> Result<(u8, [u8; 32], [u8; 32])> {
	if sig.len() != 65 {
		anyhow::bail!("Invalid signature length");
	}

	let r: [u8; 32] = sig[0..32].try_into()?;
	let s: [u8; 32] = sig[32..64].try_into()?;
	let v = sig[64];
	Ok((v, r, s))
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeploymentConfig {
	pub factory_deployer: String,
	pub required_balance: u128,
	pub raw_tx: String,
	pub factory_address: String,
	pub deployment_salt: [u8; 32],
	pub proxy_admin_sk: String,
}

#[derive(Deserialize)]
struct Contract {
	bytecode: Bytecode,
}

#[derive(Deserialize)]
struct Bytecode {
	object: String,
}

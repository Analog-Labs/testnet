use serde::{Deserialize, Serialize};
use serde_big_array::Array;
use time_primitives::{
	Address, BatchId, Gateway, GatewayMessage, GmpEvent, GmpMessage, MessageId, NetworkId, Route,
	TssPublicKey, TssSignature,
};

#[derive(Serialize, Deserialize)]
pub struct FaucetRequest {
	pub balance: u128,
}

#[derive(Serialize, Deserialize)]
pub struct FaucetResponse {}

#[derive(Serialize, Deserialize)]
pub struct TransferRequest {
	pub address: Address,
	pub amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct TransferResponse {}

#[derive(Serialize, Deserialize)]
pub struct BalanceRequest {
	pub address: Address,
}

#[derive(Serialize, Deserialize)]
pub struct BalanceResponse {
	pub balance: u128,
}

#[derive(Serialize, Deserialize)]
pub struct FinalizedBlockRequest {}

#[derive(Serialize, Deserialize)]
pub struct FinalizedBlockResponse {
	pub finalized_block: u64,
}

#[derive(Serialize, Deserialize)]
pub struct BlockStreamRequest {}

#[derive(Serialize, Deserialize)]
pub struct BlockStreamResponse {
	pub block: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ReadEventsRequest {
	pub gateway: Gateway,
	pub start_block: u64,
	pub end_block: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ReadEventsResponse {
	pub events: Vec<GmpEvent>,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitCommandsRequest {
	pub gateway: Gateway,
	pub batch: BatchId,
	pub msg: GatewayMessage,
	#[serde(with = "time_primitives::serde_tss_public_key")]
	pub signer: TssPublicKey,
	#[serde(with = "time_primitives::serde_tss_signature")]
	pub sig: TssSignature,
}

#[derive(Serialize, Deserialize)]
pub struct SubmitCommandsResponse {}

#[derive(Serialize, Deserialize)]
pub struct DeployGatewayRequest {
	pub proxy: Vec<u8>,
	pub gateway: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct DeployGatewayResponse {
	pub address: Address,
	pub block: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RedeployGatewayRequest {
	pub proxy: Address,
	pub gateway: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct RedeployGatewayResponse {}

#[derive(Serialize, Deserialize)]
pub struct AdminRequest {
	pub gateway: Gateway,
}

#[derive(Serialize, Deserialize)]
pub struct AdminResponse {
	pub address: Address,
}

#[derive(Serialize, Deserialize)]
pub struct SetAdminRequest {
	pub gateway: Gateway,
	pub admin: Address,
}

#[derive(Serialize, Deserialize)]
pub struct SetAdminResponse {}

#[derive(Serialize, Deserialize)]
pub struct ShardsRequest {
	pub gateway: Gateway,
}

#[derive(Serialize, Deserialize)]
pub struct ShardsResponse {
	pub shards: Vec<Array<u8, 33>>,
}

#[derive(Serialize, Deserialize)]
pub struct SetShardsRequest {
	pub gateway: Gateway,
	pub shards: Vec<Array<u8, 33>>,
}

#[derive(Serialize, Deserialize)]
pub struct SetShardsResponse {}

#[derive(Serialize, Deserialize)]
pub struct RoutesRequest {
	pub gateway: Gateway,
}

#[derive(Serialize, Deserialize)]
pub struct RoutesResponse {
	pub routes: Vec<Route>,
}

#[derive(Serialize, Deserialize)]
pub struct SetRouteRequest {
	pub gateway: Gateway,
	pub route: Route,
}

#[derive(Serialize, Deserialize)]
pub struct SetRouteResponse {}

#[derive(Serialize, Deserialize)]
pub struct DeployTestRequest {
	pub gateway: Address,
	pub tester: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct DeployTestResponse {
	pub address: Address,
	pub block: u64,
}

#[derive(Serialize, Deserialize)]
pub struct EstimateMessageGasLimitRequest {
	pub contract: Address,
	pub src_network: NetworkId,
	pub src: Address,
	pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct EstimateMessageGasLimitResponse {
	pub gas_limit: u128,
}

#[derive(Serialize, Deserialize)]
pub struct EstimateMessageCostRequest {
	pub gateway: Address,
	pub dest_network: NetworkId,
	pub gas_limit: u128,
	pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct EstimateMessageCostResponse {
	pub cost: u128,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageRequest {
	pub src: Address,
	pub dest_network: NetworkId,
	pub dest: Address,
	pub gas_limit: u128,
	pub gas_cost: u128,
	pub payload: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageResponse {
	pub message_id: MessageId,
}

#[derive(Serialize, Deserialize)]
pub struct RecvMessagesRequest {
	pub contract: Address,
	pub start_block: u64,
	pub end_block: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RecvMessagesResponse {
	pub messages: Vec<GmpMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionBaseFeeRequest {}

#[derive(Serialize, Deserialize)]
pub struct TransactionBaseFeeResponse {
	pub base_fee: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BlockGasLimitRequest {}

#[derive(Serialize, Deserialize)]
pub struct BlockGasLimitResponse {
	pub gas_limit: u64,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawFundsRequest {
	pub gateway: Address,
	pub amount: u128,
	pub address: Address,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawFundsResponse {}

use borsh::{BorshDeserialize, BorshSerialize};
use frost_dalek::{
	precomputation::PublicCommitmentShareList,
	signature::{PartialThresholdSignature, Signer, ThresholdSignature},
	IndividualPublicKey, Parameters, Participant,
};

//tss network request types
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TSSEventType {
	ReceivePeerIDForIndex(u64),
	ReceiveParams(u64),
	ReceivePeersWithColParticipant(u64),
	ReceiveParticipant(u64),
	ReceiveSecretShare(u64),
	ReceiveCommitment(u64),
	PartialSignatureGenerateReq(u64),
	PartialSignatureReceived(u64),
	VerifyThresholdSignature(u64),
	ResetTSSState(u64),
}

//genetic call for tss events
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TSSData {
	pub peer_id: String,
	pub tss_event_type: TSSEventType,
	pub tss_data: Vec<u8>,
}

//call for publishing peer id i.e. tss init call
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PublishPeerIDCall {
	pub peer_id: String,
	pub random: String,
}

//call for resetting tss state
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ResetTSSCall {
	pub reason: String,
	pub random: String,
}

//call for publishing peer id
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ReceiveParamsWithPeerCall {
	pub peer_id: String,
	pub random: String,
	pub params: Parameters,
}

//call to receive others peers participant
#[derive(Debug, BorshSerialize, BorshDeserialize, Clone, PartialEq)]
pub struct OthersCommitmentShares {
	pub public_key: IndividualPublicKey,
	pub public_commitment_share_list: PublicCommitmentShareList,
}

//call to sign message with participant signature
#[derive(Debug, BorshSerialize, BorshDeserialize, Clone)]
pub struct PartialMessageSign {
	pub msg_hash: [u8; 64],
	pub signers: Vec<Signer>,
}

//call to receive partial signature from other peers
#[derive(Debug, BorshSerialize, BorshDeserialize, Clone)]
pub struct ReceivePartialSignatureReq {
	pub msg_hash: [u8; 64],
	pub partial_sign: PartialThresholdSignature,
}

//Call for verifying threshold signature generated by aggregator
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct VerifyThresholdSignatureReq {
	pub msg_hash: [u8; 64],
	pub threshold_sign: ThresholdSignature,
}

//network call for receiveing peers and collector participant
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct FilterAndPublishParticipant {
	pub total_peer_list: Vec<String>,
	pub col_participant: Participant,
}

//cli params struct
pub struct TSSCliParams {
	pub total_nodes: u8,
	pub threshold: u8,
}

//defines tss local state for current node
#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
pub enum TSSLocalStateType {
	NotParticipating,
	Empty,
	ReceivedPeers,
	ReceivedParams,
	DkgGeneratedR1,
	DkgGeneratedR2,
	StateFinished,
	CommitmentsReceived,
}

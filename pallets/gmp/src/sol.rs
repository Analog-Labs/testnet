use sp_std::vec::Vec;
alloy_sol_macro::sol! {
	#[derive(Default, Debug, PartialEq, Eq)]
	struct Signature {
		uint8 parity;
		uint256 px;
		uint256 e;
		uint256 s;
	}

	#[derive(Debug, PartialEq, Eq)]
	struct GMPMessage {
		uint128 nonce;
		uint128 networkId; // source network id
		bytes32 sender;    // sender public key
		address dest;      // dest contract
		bytes payload;     // message payload
	}

	#[derive(Debug, PartialEq, Eq)]
	struct TssKey {
		uint8 parity;    // public key y-coord parity (27 or 28)
		bytes32 coordX;  // public key x-coord
	}

	#[derive(Debug, PartialEq, Eq)]
	interface IGateway {
		function registerTSSKeys(Signature memory signature, TssKey[] memory tssKeys) external;
		function revokeTSSKeys(Signature memory signature, TssKey[] memory tssKeys) external;
		function execute(Signature memory signature, GMPMessage memory message) external;
	}
}

// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use super::mock::*;
use super::{Assets, Error, EthBridge};
use crate::common::AssetId;
use crate::contract::{functions, FUNCTIONS, RECEIVE_BY_ETHEREUM_ASSET_ADDRESS_ID};
use crate::offchain::SignatureParams;
use crate::requests::{
	encode_outgoing_request_eth_call, AssetKind, IncomingChangePeers, IncomingMetaRequestKind,
	IncomingMigrate, IncomingPrepareForMigration, IncomingRequest, IncomingTransfer,
	OffchainRequest, OutgoingAddPeer, OutgoingMigrate, OutgoingPrepareForMigration,
	OutgoingRemovePeer, OutgoingRequest, OutgoingTransfer,
};
use crate::tests::mock::{get_account_id_from_seed, ExtBuilder};
use crate::tests::{
	approve_last_request, assert_incoming_request_done,
	assert_incoming_request_registration_failed, last_outgoing_request, request_incoming,
	ETH_NETWORK_ID,
};
use crate::{AssetConfig, EthAddress};
use frame_support::assert_ok;
use frame_support::sp_runtime::{DispatchResult, TransactionOutcome};
use frame_support::traits::Currency;
use hex_literal::hex;
use polkadot_sdk::*;
use sp_core::crypto::AccountId32;
use sp_core::sr25519;
use sp_core::{H160, H256};
use std::str::FromStr;

#[test]
#[ignore]
fn should_cancel_ready_outgoing_request() {
	let (mut ext, state) = ExtBuilder::default().build();
	let _ = FUNCTIONS.get_or_init(functions);
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		// Sending request part
		Assets::mint_to(&AssetId::Balances, &alice, &alice, 100u32.into()).unwrap();
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 100u32.into());
		assert_ok!(EthBridge::transfer_to_sidechain(
			RuntimeOrigin::signed(alice.clone()),
			AssetId::Balances,
			EthAddress::from_str("19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A").unwrap(),
			100_u32.into(),
			net_id,
		));
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 0);
		let (outgoing_req, outgoing_req_hash) =
			approve_last_request(&state, net_id).expect("request wasn't approved");

		// Cancelling request part
		let tx_hash = H256::from_slice(&[1u8; 32]);
		let request_hash = request_incoming(
			alice.clone(),
			tx_hash,
			IncomingMetaRequestKind::CancelOutgoingRequest.into(),
			net_id,
		)
		.unwrap();
		let tx_input = encode_outgoing_request_eth_call::<Runtime>(
			*RECEIVE_BY_ETHEREUM_ASSET_ADDRESS_ID.get().unwrap(),
			&outgoing_req,
			outgoing_req_hash,
		)
		.unwrap();
		let incoming_transfer =
			IncomingRequest::CancelOutgoingRequest(crate::IncomingCancelOutgoingRequest {
				outgoing_request: outgoing_req.clone(),
				outgoing_request_hash: outgoing_req_hash,
				initial_request_hash: request_hash,
				tx_input: tx_input.clone(),
				author: alice.clone(),
				tx_hash,
				at_height: 1,
				timepoint: Default::default(),
				network_id: ETH_NETWORK_ID,
			});

		assert_incoming_request_done(&state, incoming_transfer.clone()).unwrap();
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 100u32.into());
	});
}

#[test]
#[ignore]
fn should_fail_cancel_ready_outgoing_request_with_wrong_approvals() {
	let (mut ext, state) = ExtBuilder::default().build();
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		// Sending request part
		Assets::mint_to(&AssetId::Balances, &alice, &alice, 100u32.into()).unwrap();
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 100u32.into());
		assert_ok!(EthBridge::transfer_to_sidechain(
			RuntimeOrigin::signed(alice.clone()),
			AssetId::Balances,
			EthAddress::from_str("19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A").unwrap(),
			100_u32.into(),
			net_id,
		));
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 0);
		let (outgoing_req, outgoing_req_hash) =
			approve_last_request(&state, net_id).expect("request wasn't approved");

		// Cancelling request part
		let tx_hash = H256::from_slice(&[1u8; 32]);
		let request_hash = request_incoming(
			alice.clone(),
			tx_hash,
			IncomingMetaRequestKind::CancelOutgoingRequest.into(),
			net_id,
		)
		.unwrap();
		let tx_input = encode_outgoing_request_eth_call::<Runtime>(
			*RECEIVE_BY_ETHEREUM_ASSET_ADDRESS_ID.get().unwrap(),
			&outgoing_req,
			outgoing_req_hash,
		)
		.unwrap();
		let incoming_transfer =
			IncomingRequest::CancelOutgoingRequest(crate::IncomingCancelOutgoingRequest {
				outgoing_request: outgoing_req.clone(),
				outgoing_request_hash: outgoing_req_hash,
				initial_request_hash: request_hash,
				tx_input: tx_input.clone(),
				author: alice.clone(),
				tx_hash,
				at_height: 1,
				timepoint: Default::default(),
				network_id: ETH_NETWORK_ID,
			});

		// Insert some signature
		crate::RequestApprovals::<Runtime>::mutate(net_id, outgoing_req_hash, |v| {
			v.insert(SignatureParams { r: [1; 32], s: [1; 32], v: 0 })
		});
		assert_incoming_request_registration_failed(
			&state,
			incoming_transfer.clone(),
			Error::InvalidContractInput,
		)
		.unwrap();
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 0);
	});
}

#[test]
#[ignore]
fn should_fail_cancel_unfinished_outgoing_request() {
	let (mut ext, state) = ExtBuilder::default().build();
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		// Sending request part
		Assets::mint_to(&AssetId::Balances, &alice, &alice, 100u32.into()).unwrap();
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 100u32.into());
		assert_ok!(EthBridge::transfer_to_sidechain(
			RuntimeOrigin::signed(alice.clone()),
			AssetId::Balances,
			EthAddress::from_str("19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A").unwrap(),
			100_u32.into(),
			net_id,
		));
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 0);
		let (outgoing_req, outgoing_req_hash) =
			last_outgoing_request(net_id).expect("request wasn't found");

		// Cancelling request part
		let tx_hash = H256::from_slice(&[1u8; 32]);
		let request_hash = request_incoming(
			alice.clone(),
			tx_hash,
			IncomingMetaRequestKind::CancelOutgoingRequest.into(),
			net_id,
		)
		.unwrap();
		let tx_input = encode_outgoing_request_eth_call::<Runtime>(
			*RECEIVE_BY_ETHEREUM_ASSET_ADDRESS_ID.get().unwrap(),
			&outgoing_req,
			outgoing_req_hash,
		)
		.unwrap();
		let incoming_transfer =
			IncomingRequest::CancelOutgoingRequest(crate::IncomingCancelOutgoingRequest {
				outgoing_request: outgoing_req,
				outgoing_request_hash: outgoing_req_hash,
				initial_request_hash: request_hash,
				tx_input,
				author: alice.clone(),
				tx_hash,
				at_height: 1,
				timepoint: Default::default(),
				network_id: ETH_NETWORK_ID,
			});
		assert_incoming_request_registration_failed(
			&state,
			incoming_transfer.clone(),
			Error::RequestIsNotReady,
		)
		.unwrap();
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 0);
	});
}

#[test]
fn should_cancel_outgoing_prepared_requests() {
	let net_id = ETH_NETWORK_ID;
	let mut builder = ExtBuilder::default();
	builder.add_network(
		vec![AssetConfig::Reservable {
			id: AssetId::Balances,
			sidechain_id: sp_core::H160::from_str("40fd72257597aa14c7231a7b1aaa29fce868f677")
				.unwrap(),
		}],
		Some(vec![(AssetId::Balances, 350000), (AssetId::Balances, 33900000)]),
		Some(5),
		Default::default(),
	);
	let (mut ext, state) = builder.build();
	ext.execute_with(|| {
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let bridge_acc = &state.networks[&net_id].config.bridge_account_id;
		Assets::mint_to(&AssetId::Balances, &alice, &alice, 100u32.into()).unwrap();
		Assets::mint_to(&AssetId::Balances, &alice, bridge_acc, 100u32.into()).unwrap();
		let ocw0_account_id = &state.networks[&net_id].ocw_keypairs[0].1;
		// Paris (preparation requests, testable request).
		let test_acc = AccountId32::new([10u8; 32]);
		let _ = pallet_balances::Pallet::<Runtime>::deposit_creating(&test_acc, 1u32.into());
		let requests: Vec<(Vec<OffchainRequest<Runtime>>, OffchainRequest<Runtime>)> = vec![
			(
				vec![],
				OutgoingTransfer {
					from: alice.clone(),
					to: EthAddress::from_str("19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A").unwrap(),
					asset_id: AssetId::Balances,
					amount: 1_u32.into(),
					nonce: 0,
					network_id: net_id,
					timepoint: Default::default(),
				}
				.into(),
			),
			(
				vec![],
				OutgoingAddPeer {
					author: alice.clone(),
					peer_address: EthAddress::from([10u8; 20]),
					nonce: 0,
					network_id: net_id,
					peer_account_id: test_acc.clone(),
					timepoint: Default::default(),
				}
				.into(),
			),
			(
				vec![],
				OutgoingAddPeer {
					author: alice.clone(),
					peer_address: EthAddress::from([10u8; 20]),
					nonce: 0,
					network_id: net_id + 1,
					peer_account_id: test_acc.clone(),
					timepoint: Default::default(),
				}
				.into(),
			),
			(
				vec![],
				OutgoingAddPeer {
					author: alice.clone(),
					peer_address: EthAddress::from([10u8; 20]),
					nonce: 0,
					network_id: net_id,
					peer_account_id: test_acc.clone(),
					timepoint: Default::default(),
				}
				.into(),
			),
			(
				vec![
					OutgoingAddPeer {
						author: alice.clone(),
						peer_address: EthAddress::from([10u8; 20]),
						nonce: 0,
						network_id: net_id + 1,
						peer_account_id: test_acc.clone(),
						timepoint: Default::default(),
					}
					.into(),
					IncomingChangePeers {
						peer_account_id: Some(test_acc.clone()),
						peer_address: EthAddress::from([10u8; 20]),
						removed: false,
						author: alice.clone(),
						tx_hash: H256([11; 32]),
						at_height: 0,
						timepoint: Default::default(),
						network_id: net_id + 1,
					}
					.into(),
				],
				OutgoingRemovePeer {
					author: alice.clone(),
					peer_address: EthAddress::from([10u8; 20]),
					nonce: 0,
					network_id: net_id + 1,
					peer_account_id: test_acc.clone(),
					timepoint: Default::default(),
				}
				.into(),
			),
			(
				vec![],
				OutgoingRemovePeer {
					author: alice.clone(),
					peer_address: crate::PeerAddress::<Runtime>::get(&net_id, &ocw0_account_id),
					nonce: 0,
					network_id: net_id,
					peer_account_id: ocw0_account_id.clone(),
					timepoint: Default::default(),
				}
				.into(),
			),
			(
				vec![],
				OutgoingPrepareForMigration {
					author: alice.clone(),
					nonce: 0,
					network_id: net_id,
					timepoint: Default::default(),
				}
				.into(),
			),
			(
				vec![
					OutgoingPrepareForMigration {
						author: alice.clone(),
						nonce: 0,
						network_id: net_id,
						timepoint: Default::default(),
					}
					.into(),
					IncomingPrepareForMigration {
						author: alice.clone(),
						tx_hash: [1u8; 32].into(),
						at_height: 0,
						timepoint: Default::default(),
						network_id: net_id,
					}
					.into(),
				],
				OutgoingMigrate {
					author: alice.clone(),
					new_contract_address: Default::default(),
					erc20_native_tokens: vec![],
					nonce: 0,
					network_id: net_id,
					timepoint: Default::default(),
					new_signature_version: crate::BridgeSignatureVersion::V3,
				}
				.into(),
			),
		];
		for (preparations, request) in requests {
			frame_support::storage::with_transaction(|| {
				for preparation_request in &preparations {
					preparation_request.validate().unwrap();
					preparation_request.prepare().unwrap();
					match preparation_request {
						// Do not finalize add/remove peer requests for ethereum network,
						// because of XOR and VAL contracts (see `OutgoingAddPeerCompat`).
						OffchainRequest::Outgoing(OutgoingRequest::AddPeer(req), ..)
							if req.network_id == ETH_NETWORK_ID => {},
						OffchainRequest::Outgoing(OutgoingRequest::RemovePeer(req), ..)
							if req.network_id == ETH_NETWORK_ID => {},
						_ => preparation_request.finalize().unwrap(),
					}
				}
				// Save the current storage root hash, apply transaction preparation,
				// cancel it and compare with the final root hash.
				frame_system::Pallet::<Runtime>::reset_events();
				let state_hash_before = sp_io::storage::root(sp_runtime::StateVersion::V1);
				request.validate().unwrap();
				request.prepare().unwrap();
				request.cancel().unwrap();
				frame_system::Pallet::<Runtime>::reset_events();
				let state_hash_after = sp_io::storage::root(sp_runtime::StateVersion::V1);
				assert_eq!(state_hash_before, state_hash_after);
				TransactionOutcome::Rollback(DispatchResult::Ok(()))
			})
			.unwrap();
		}
	});
}

#[test]
fn should_cancel_incoming_prepared_requests() {
	let net_id = ETH_NETWORK_ID;
	let mut builder = ExtBuilder::default();
	builder.add_currency(
		net_id,
		AssetConfig::Reservable {
			id: AssetId::Balances,
			sidechain_id: H160(hex!("dAC17F958D2ee523a2206206994597C13D831ec7")),
		},
	);
	let (mut ext, state) = builder.build();
	ext.execute_with(|| {
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let bridge_acc = &state.networks[&net_id].config.bridge_account_id;
		Assets::mint_to(&AssetId::Balances, &alice, &alice, 100u32.into()).unwrap();
		Assets::mint_to(&AssetId::Balances, &alice, bridge_acc, 100u32.into()).unwrap();
		Assets::mint_to(&AssetId::Balances, &alice, bridge_acc, 100u32.into()).unwrap();
		// Paris (preparation requests, testable request).
		let requests: Vec<(Vec<OffchainRequest<Runtime>>, OffchainRequest<Runtime>)> = vec![
			(
				vec![],
				IncomingTransfer {
					from: EthAddress::from_str("19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A").unwrap(),
					to: alice.clone(),
					asset_id: AssetId::Balances,
					asset_kind: AssetKind::Reservable,
					amount: 1_u32.into(),
					author: alice.clone(),
					tx_hash: Default::default(),
					network_id: net_id,
					timepoint: Default::default(),
					at_height: 0,
					should_take_fee: false,
				}
				.into(),
			),
			(
				vec![],
				IncomingTransfer {
					from: EthAddress::from_str("19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A").unwrap(),
					to: alice.clone(),
					asset_id: AssetId::Balances,
					asset_kind: AssetKind::Reservable,
					amount: 1_u32.into(),
					author: alice.clone(),
					tx_hash: Default::default(),
					network_id: net_id,
					timepoint: Default::default(),
					at_height: 0,
					should_take_fee: false,
				}
				.into(),
			),
			(
				vec![],
				IncomingTransfer {
					from: EthAddress::from_str("19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A").unwrap(),
					to: alice.clone(),
					asset_id: AssetId::Balances,
					asset_kind: AssetKind::Reservable,
					amount: 1_u32.into(),
					author: alice.clone(),
					tx_hash: Default::default(),
					network_id: net_id,
					timepoint: Default::default(),
					at_height: 0,
					should_take_fee: false,
				}
				.into(),
			),
			(
				vec![],
				IncomingPrepareForMigration {
					author: alice.clone(),
					tx_hash: Default::default(),
					network_id: net_id,
					timepoint: Default::default(),
					at_height: 0,
				}
				.into(),
			),
			(
				vec![
					IncomingPrepareForMigration {
						author: alice.clone(),
						tx_hash: Default::default(),
						network_id: net_id,
						timepoint: Default::default(),
						at_height: 0,
					}
					.into(),
					OutgoingMigrate {
						author: alice.clone(),
						new_contract_address: Default::default(),
						erc20_native_tokens: vec![],
						nonce: Default::default(),
						network_id: net_id,
						timepoint: Default::default(),
						new_signature_version: crate::BridgeSignatureVersion::V3,
					}
					.into(),
				],
				IncomingMigrate {
					new_contract_address: Default::default(),
					author: alice.clone(),
					tx_hash: Default::default(),
					network_id: net_id,
					timepoint: Default::default(),
					at_height: 0,
				}
				.into(),
			),
			// TODO: test incoming 'cancel outgoing request'
		];
		for (preparations, request) in requests {
			frame_support::storage::with_transaction(|| {
				for preparation_request in preparations {
					preparation_request.prepare().unwrap();
					preparation_request.finalize().unwrap();
				}
				// Save the current storage root hash, apply transaction preparation,
				// cancel it and compare with the final root hash.
				frame_system::Pallet::<Runtime>::reset_events();
				let state_hash_before = sp_io::storage::root(sp_runtime::StateVersion::V1);
				request.prepare().unwrap();
				request.cancel().unwrap();
				frame_system::Pallet::<Runtime>::reset_events();
				let state_hash_after = sp_io::storage::root(sp_runtime::StateVersion::V1);
				assert_eq!(state_hash_before, state_hash_after);
				TransactionOutcome::Rollback(DispatchResult::Ok(()))
			})
			.unwrap();
		}
	});
}

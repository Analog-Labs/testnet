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

use super::assert_last_event;
use super::mock::*;
use super::Error;
use crate::common::{AssetId, Balance};
use crate::contract::{ContractEvent, DepositEvent};
use crate::requests::{
	AssetKind, IncomingRequest, IncomingRequestKind, IncomingTransactionRequestKind,
	LoadIncomingRequest, LoadIncomingTransactionRequest, RequestStatus,
};
use crate::tests::mock::{get_account_id_from_seed, ExtBuilder};
use crate::tests::{assert_incoming_request_done, request_incoming, ETH_NETWORK_ID};
use crate::types::{Log, TransactionReceipt};
use crate::{types, AssetConfig, EthAddress, CONFIRMATION_INTERVAL};
use frame_support::assert_noop;
use frame_support::dispatch::{DispatchErrorWithPostInfo, Pays, PostDispatchInfo};
use frame_support::{assert_err, assert_ok};
use hex_literal::hex;
use polkadot_sdk::*;
use scale_codec::Encode;
use sp_core::H160;
use sp_core::{sr25519, H256};
use std::str::FromStr;

#[test]
fn should_not_accept_duplicated_incoming_transfer() {
	let (mut ext, _state) = ExtBuilder::default().build();

	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		assert_ok!(EthBridge::request_from_sidechain(
			RuntimeOrigin::signed(alice.clone()),
			H256::from_slice(&[1u8; 32]),
			IncomingTransactionRequestKind::Transfer.into(),
			net_id,
		));
		assert_err!(
			EthBridge::request_from_sidechain(
				RuntimeOrigin::signed(alice.clone()),
				H256::from_slice(&[1u8; 32]),
				IncomingTransactionRequestKind::Transfer.into(),
				net_id,
			),
			Error::DuplicatedRequest
		);
	});
}

#[test]
fn should_not_accept_approved_incoming_transfer() {
	let (mut ext, state) = ExtBuilder::default().build();

	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let tx_hash = request_incoming(
			alice.clone(),
			H256::from_slice(&[1u8; 32]),
			IncomingTransactionRequestKind::Transfer.into(),
			net_id,
		)
		.unwrap();
		let incoming_transfer = IncomingRequest::Transfer(crate::IncomingTransfer {
			from: EthAddress::from([1; 20]),
			to: alice.clone(),
			asset_id: AssetId::Balances,
			asset_kind: AssetKind::Reservable,
			amount: 100u32.into(),
			author: alice.clone(),
			tx_hash,
			at_height: 1,
			timepoint: Default::default(),
			network_id: ETH_NETWORK_ID,
			should_take_fee: false,
		});
		assert_incoming_request_done(&state, incoming_transfer.clone()).unwrap();
		assert_err!(
			EthBridge::request_from_sidechain(
				RuntimeOrigin::signed(alice.clone()),
				H256::from_slice(&[1u8; 32]),
				IncomingTransactionRequestKind::Transfer.into(),
				net_id,
			),
			Error::DuplicatedRequest
		);
	});
}

#[test]
fn should_success_incoming_transfer() {
	let (mut ext, state) = ExtBuilder::default().build();
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let tx_hash = request_incoming(
			alice.clone(),
			H256::from_slice(&[1u8; 32]),
			IncomingTransactionRequestKind::Transfer.into(),
			net_id,
		)
		.unwrap();
		let incoming_transfer = IncomingRequest::Transfer(crate::IncomingTransfer {
			from: EthAddress::from([1; 20]),
			to: alice.clone(),
			asset_id: AssetId::Balances,
			asset_kind: AssetKind::Reservable,
			amount: 99u32.into(),
			author: alice.clone(),
			tx_hash,
			at_height: 1,
			timepoint: Default::default(),
			network_id: ETH_NETWORK_ID,
			should_take_fee: false,
		});
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 1);
		assert_incoming_request_done(&state, incoming_transfer.clone()).unwrap();
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 100u32.into());
	});
}

#[test]
fn should_cancel_incoming_transfer() {
	let mut builder = ExtBuilder::new();
	let net_id = builder.add_network(
		vec![AssetConfig::Reservable {
			id: AssetId::Balances,
			sidechain_id: sp_core::H160::from_str("40fd72257597aa14c7231a7b1aaa29fce868f677")
				.unwrap(),
		}],
		Some(vec![(AssetId::Balances, Balance::from(100u32))]),
		None,
		Default::default(),
	);
	let (mut ext, state) = builder.build();
	ext.execute_with(|| {
		let bridge_acc_id = state.networks[&net_id].config.bridge_account_id.clone();
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		Assets::mint_to(&AssetId::Balances, &alice, &alice, 99999u32.into()).unwrap();
		let bob = get_account_id_from_seed::<sr25519::Public>("Bob");
		let tx_hash = request_incoming(
			alice.clone(),
			H256::from_slice(&[1u8; 32]),
			IncomingTransactionRequestKind::Transfer.into(),
			net_id,
		)
		.unwrap();
		let incoming_transfer = IncomingRequest::Transfer(crate::IncomingTransfer {
			from: EthAddress::from([1; 20]),
			to: alice.clone(),
			asset_id: AssetId::Balances,
			asset_kind: AssetKind::Reservable,
			amount: 100u32.into(),
			author: alice.clone(),
			tx_hash,
			at_height: 1,
			timepoint: Default::default(),
			network_id: ETH_NETWORK_ID,
			should_take_fee: false,
		});
		assert_ok!(EthBridge::register_incoming_request(
			RuntimeOrigin::signed(bridge_acc_id.clone()),
			incoming_transfer.clone(),
		));
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 100000u32.into());
		Assets::unreserve(&AssetId::Balances, &bridge_acc_id, 100u32.into()).unwrap();
		Assets::transfer_from(&AssetId::Balances, &bridge_acc_id, &bob, 100u32.into()).unwrap();
		let req_hash = crate::LoadToIncomingRequestHash::<Runtime>::get(net_id, tx_hash);
		assert_ok!(
			EthBridge::finalize_incoming_request(
				RuntimeOrigin::signed(bridge_acc_id.clone()),
				req_hash,
				net_id,
			),
			PostDispatchInfo {
				pays_fee: Pays::No.into(),
				actual_weight: None
			}
		);
		assert_last_event::<Runtime>(crate::Event::CancellationFailed(req_hash).into());
		assert!(matches!(
			crate::RequestStatuses::<Runtime>::get(net_id, req_hash).unwrap(),
			RequestStatus::Broken(_, _)
		));
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 100000u32.into());
	});
}

#[test]
fn should_fail_incoming_transfer() {
	let (mut ext, state) = ExtBuilder::default().build();
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let bridge_acc_id = state.networks[&net_id].config.bridge_account_id.clone();
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		Assets::mint_to(&AssetId::Balances, &alice, &alice, 99999u32.into()).unwrap();
		let tx_hash = request_incoming(
			alice.clone(),
			H256::from_slice(&[1u8; 32]),
			IncomingTransactionRequestKind::Transfer.into(),
			net_id,
		)
		.unwrap();
		let incoming_transfer = IncomingRequest::Transfer(crate::IncomingTransfer {
			from: EthAddress::from([1; 20]),
			to: alice.clone(),
			asset_id: AssetId::Balances,
			asset_kind: AssetKind::Reservable,
			amount: 100u32.into(),
			author: alice.clone(),
			tx_hash,
			at_height: 1,
			timepoint: Default::default(),
			network_id: ETH_NETWORK_ID,
			should_take_fee: false,
		});
		assert_ok!(EthBridge::register_incoming_request(
			RuntimeOrigin::signed(bridge_acc_id.clone()),
			incoming_transfer.clone(),
		));
		let req_hash = crate::LoadToIncomingRequestHash::<Runtime>::get(net_id, tx_hash);
		assert!(crate::RequestsQueue::<Runtime>::get(net_id).contains(&req_hash));
		assert_eq!(
			*crate::Requests::get(net_id, &req_hash).unwrap().as_incoming().unwrap().0,
			incoming_transfer
		);
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 100000u32.into());
		assert_ok!(EthBridge::abort_request(
			RuntimeOrigin::signed(bridge_acc_id),
			req_hash,
			Error::Other.into(),
			net_id,
		));
		assert!(matches!(
			crate::RequestStatuses::<Runtime>::get(net_id, &req_hash).unwrap(),
			RequestStatus::Failed(_)
		));
		assert!(!crate::RequestsQueue::<Runtime>::get(net_id).contains(&req_hash));
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 100000u32.into());
	});
}

#[test]
fn should_take_fee_in_incoming_transfer() {
	let (mut ext, state) = ExtBuilder::default().build();
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let tx_hash = request_incoming(
			alice.clone(),
			H256::from_slice(&[1u8; 32]),
			IncomingTransactionRequestKind::Transfer.into(),
			net_id,
		)
		.unwrap();
		let incoming_transfer = IncomingRequest::Transfer(crate::IncomingTransfer {
			from: EthAddress::from([1; 20]),
			to: alice.clone(),
			asset_id: AssetId::Balances,
			asset_kind: AssetKind::Reservable,
			amount: 100,
			author: alice.clone(),
			tx_hash,
			at_height: 1,
			timepoint: Default::default(),
			network_id: ETH_NETWORK_ID,
			should_take_fee: true,
		});
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 1);
		assert_incoming_request_done(&state, incoming_transfer.clone()).unwrap();
		assert_eq!(Assets::total_balance(&AssetId::Balances, &alice).unwrap(), 101);
	});
}

#[test]
fn should_fail_registering_incoming_request_if_preparation_failed() {
	let net_id = ETH_NETWORK_ID;
	let mut builder = ExtBuilder::default();
	builder.add_currency(
		net_id,
		AssetConfig::Reservable {
			id: AssetId::Balances,
			sidechain_id: H160::repeat_byte(2),
		},
	);
	let (mut ext, state) = builder.build();

	ext.execute_with(|| {
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let tx_hash = request_incoming(
			alice.clone(),
			H256::from_slice(&[1u8; 32]),
			IncomingTransactionRequestKind::Transfer.into(),
			net_id,
		)
		.unwrap();
		let incoming_transfer = IncomingRequest::Transfer(crate::IncomingTransfer {
			from: EthAddress::from([1; 20]),
			to: alice.clone(),
			asset_id: AssetId::Balances,
			asset_kind: AssetKind::Reservable,
			amount: 1000_000_000u32.into(),
			author: alice.clone(),
			tx_hash,
			at_height: 1,
			timepoint: Default::default(),
			network_id: net_id,
			should_take_fee: false,
		});
		let bridge_acc_id = state.networks[&net_id].config.bridge_account_id.clone();
		assert_ok!(
			EthBridge::register_incoming_request(
				RuntimeOrigin::signed(bridge_acc_id.clone()),
				incoming_transfer.clone(),
			),
			PostDispatchInfo {
				pays_fee: Pays::No.into(),
				actual_weight: None
			}
		);
		let req_hash = crate::LoadToIncomingRequestHash::<Runtime>::get(net_id, tx_hash);
		assert_last_event::<Runtime>(
			crate::Event::RegisterRequestFailed(
				req_hash,
				pallet_balances::Error::<Runtime>::InsufficientBalance.into(),
			)
			.into(),
		);
		assert!(!crate::RequestsQueue::<Runtime>::get(net_id).contains(&tx_hash));
		assert!(!crate::RequestsQueue::<Runtime>::get(net_id).contains(&req_hash));
		assert!(crate::Requests::<Runtime>::get(net_id, &req_hash).is_none());
		assert!(matches!(
			crate::RequestStatuses::<Runtime>::get(net_id, &req_hash).unwrap(),
			RequestStatus::Failed(_)
		));
	});
}

#[test]
fn should_import_incoming_request() {
	let (mut ext, state) = ExtBuilder::default().build();

	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let load_incoming_transaction_request = LoadIncomingTransactionRequest::new(
			alice.clone(),
			H256([1; 32]),
			Default::default(),
			IncomingTransactionRequestKind::Transfer,
			net_id,
		);
		let incoming_transfer_result = IncomingRequest::try_from_contract_event(
			ContractEvent::Deposit(DepositEvent::new(
				alice.clone(),
				1,
				crate::RegisteredSidechainToken::<Runtime>::get(net_id, AssetId::Balances).unwrap(),
				H256::zero(),
			)),
			load_incoming_transaction_request.clone(),
			1,
		)
		.map_err(|e| e.into());
		assert!(incoming_transfer_result.is_ok());
		let bridge_account_id = &state.networks[&net_id].config.bridge_account_id;
		assert_ok!(EthBridge::import_incoming_request(
			RuntimeOrigin::signed(bridge_account_id.clone()),
			LoadIncomingRequest::Transaction(load_incoming_transaction_request),
			incoming_transfer_result
		));
	});
}

#[test]
fn should_not_import_incoming_request_twice() {
	let (mut ext, state) = ExtBuilder::default().build();

	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let hash = H256([1; 32]);
		let load_incoming_transaction_request = LoadIncomingTransactionRequest::new(
			alice.clone(),
			hash,
			Default::default(),
			IncomingTransactionRequestKind::Transfer,
			net_id,
		);
		let incoming_transfer_result = IncomingRequest::try_from_contract_event(
			ContractEvent::Deposit(DepositEvent::new(
				alice.clone(),
				1,
				crate::RegisteredSidechainToken::<Runtime>::get(net_id, AssetId::Balances).unwrap(),
				H256::zero(),
			)),
			load_incoming_transaction_request.clone(),
			1,
		)
		.map_err(|e| e.into());
		assert!(incoming_transfer_result.is_ok());
		let bridge_account_id = &state.networks[&net_id].config.bridge_account_id;
		assert_ok!(EthBridge::import_incoming_request(
			RuntimeOrigin::signed(bridge_account_id.clone()),
			LoadIncomingRequest::Transaction(load_incoming_transaction_request),
			incoming_transfer_result
		));
		assert_noop!(
			EthBridge::request_from_sidechain(
				RuntimeOrigin::signed(alice),
				hash,
				IncomingRequestKind::Transaction(IncomingTransactionRequestKind::Transfer),
				net_id
			),
			Error::DuplicatedRequest
		);
	});
}

#[test]
fn ocw_should_handle_incoming_request() {
	let mut builder = ExtBuilder::new();
	builder.add_network(
		vec![AssetConfig::Reservable {
			id: AssetId::Balances,
			sidechain_id: sp_core::H160::from_str("40fd72257597aa14c7231a7b1aaa29fce868f677")
				.unwrap(),
		}],
		Some(vec![(AssetId::Balances, 350000)]),
		Some(1),
		Default::default(),
	);
	let (mut ext, mut state) = builder.build();
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let tx_hash = H256([1; 32]);
		assert_ok!(EthBridge::request_from_sidechain(
			RuntimeOrigin::signed(alice.clone()),
			tx_hash,
			IncomingRequestKind::Transaction(IncomingTransactionRequestKind::Transfer),
			net_id
		));
		let mut log = Log::default();
		log.topics = vec![types::H256(hex!(
			"85c0fa492ded927d3acca961da52b0dda1debb06d8c27fe189315f06bb6e26c8"
		))];
		let data = ethabi::encode(&[
			ethabi::Token::FixedBytes(alice.encode()),
			ethabi::Token::Uint(types::U256::from(100)),
			ethabi::Token::Address(types::EthAddress::from(
				crate::RegisteredSidechainToken::<Runtime>::get(net_id, AssetId::Balances)
					.unwrap()
					.0,
			)),
			ethabi::Token::FixedBytes(H256::zero().0.to_vec()),
		]);
		log.data = data.into();
		log.removed = Some(false);
		let receipt = TransactionReceipt {
			transaction_hash: types::H256(tx_hash.0),
			block_number: Some(0u64.into()),
			to: Some(types::H160(crate::BridgeContractAddress::<Runtime>::get(net_id).0)),
			logs: vec![log],
			status: Some(1u64.into()),
			..Default::default()
		};
		state.push_response(receipt.clone());
		state.run_next_offchain_and_dispatch_txs();
		assert_eq!(
			crate::RequestStatuses::<Runtime>::get(net_id, tx_hash).unwrap(),
			RequestStatus::Done
		);
		let hash = crate::LoadToIncomingRequestHash::<Runtime>::get(net_id, &tx_hash);
		assert_eq!(
			crate::RequestStatuses::<Runtime>::get(net_id, hash).unwrap(),
			RequestStatus::Pending
		);
		state.push_response(receipt);
		state.run_next_offchain_and_dispatch_txs();
		// assert_eq!(
		//     crate::RequestStatuses::<Runtime>::get(net_id, hash).unwrap(),
		//     RequestStatus::Done
		// );
	});
}

#[test]
fn ocw_should_not_register_pending_incoming_request() {
	let mut builder = ExtBuilder::new();
	builder.add_network(
		vec![AssetConfig::Reservable {
			id: AssetId::Balances,
			sidechain_id: sp_core::H160::from_str("40fd72257597aa14c7231a7b1aaa29fce868f677")
				.unwrap(),
		}],
		Some(vec![(AssetId::Balances, 350000)]),
		Some(1),
		Default::default(),
	);
	let (mut ext, mut state) = builder.build();
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let tx_hash = H256([1; 32]);
		assert_ok!(EthBridge::request_from_sidechain(
			RuntimeOrigin::signed(alice.clone()),
			tx_hash,
			IncomingRequestKind::Transaction(IncomingTransactionRequestKind::Transfer),
			net_id
		));
		let mut log = Log::default();
		log.topics = vec![types::H256(hex!(
			"85c0fa492ded927d3acca961da52b0dda1debb06d8c27fe189315f06bb6e26c8"
		))];
		let data = ethabi::encode(&[
			ethabi::Token::FixedBytes(alice.encode()),
			ethabi::Token::Uint(types::U256::from(100)),
			ethabi::Token::Address(types::EthAddress::from(
				crate::RegisteredSidechainToken::<Runtime>::get(net_id, AssetId::Balances)
					.unwrap()
					.0,
			)),
			ethabi::Token::FixedBytes(H256::zero().0.to_vec()),
		]);
		log.data = data.into();
		log.removed = Some(false);
		let block_number = CONFIRMATION_INTERVAL.into();
		log.block_number = Some(block_number);
		let receipt = TransactionReceipt {
			transaction_hash: types::H256(tx_hash.0),
			block_number: Some(block_number),
			to: Some(types::H160(crate::BridgeContractAddress::<Runtime>::get(net_id).0)),
			logs: vec![log],
			status: Some(1u64.into()),
			..Default::default()
		};
		state.push_response(receipt.clone());
		state.run_next_offchain_and_dispatch_txs();
		assert_eq!(
			crate::RequestStatuses::<Runtime>::get(net_id, tx_hash).unwrap(),
			RequestStatus::Done
		);
		state.push_response(receipt.clone());
		state.run_next_offchain_and_dispatch_txs();
		let hash = crate::LoadToIncomingRequestHash::<Runtime>::get(net_id, &tx_hash);
		assert_eq!(
			crate::RequestStatuses::<Runtime>::get(net_id, hash).unwrap(),
			RequestStatus::Pending
		);
	});
}

#[test]
fn ocw_should_import_incoming_request() {
	let mut builder = ExtBuilder::new();
	builder.add_network(
		vec![AssetConfig::Reservable {
			id: AssetId::Balances,
			sidechain_id: sp_core::H160::from_str("40fd72257597aa14c7231a7b1aaa29fce868f677")
				.unwrap(),
		}],
		Some(vec![(AssetId::Balances, 350000)]),
		Some(1),
		Default::default(),
	);
	let (mut ext, mut state) = builder.build();
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let mut log = Log::default();
		log.topics = vec![types::H256(hex!(
			"85c0fa492ded927d3acca961da52b0dda1debb06d8c27fe189315f06bb6e26c8"
		))];
		let data = ethabi::encode(&[
			ethabi::Token::FixedBytes(alice.encode()),
			ethabi::Token::Uint(types::U256::from(100)),
			ethabi::Token::Address(types::EthAddress::from(
				crate::RegisteredSidechainToken::<Runtime>::get(net_id, AssetId::Balances)
					.unwrap()
					.0,
			)),
			ethabi::Token::FixedBytes(H256::zero().0.to_vec()),
		]);
		let tx_hash = H256([1; 32]);
		log.data = data.into();
		log.removed = Some(false);
		log.transaction_hash = Some(types::H256(tx_hash.0));
		log.block_number = Some(0u64.into());
		log.transaction_index = Some(0u64.into());
		state.run_next_offchain_with_params(
			0,
			frame_system::Pallet::<Runtime>::block_number() + 1,
			true,
		);
		state.push_response([log]);
		// "Wait" `CONFIRMATION_INTERVAL` blocks on sidechain.
		state.run_next_offchain_with_params(
			CONFIRMATION_INTERVAL,
			frame_system::Pallet::<Runtime>::block_number() + 1,
			true,
		);
		assert_eq!(
			crate::RequestStatuses::<Runtime>::get(net_id, tx_hash).unwrap(),
			RequestStatus::Done
		);
		let hash = crate::LoadToIncomingRequestHash::<Runtime>::get(net_id, &tx_hash);
		assert_eq!(
			crate::RequestStatuses::<Runtime>::get(net_id, hash).unwrap(),
			RequestStatus::Done
		);
	});
}

#[test]
fn ocw_should_import_incoming_request_raw_response() {
	let mut builder = ExtBuilder::new();
	builder.add_network(
		vec![AssetConfig::Reservable {
			id: AssetId::Balances,
			sidechain_id: sp_core::H160::from_str("0x725c6b8cd3621eba4e0ccc40d532e7025b925a65")
				.unwrap(),
		}],
		Some(vec![(AssetId::Balances, 350_000_000_000_000_000_000_000)]),
		Some(1),
		hex!("077c2ec37d28709ce01ae740209bfbe185bd1eaa").into(),
	);
	let (mut ext, mut state) = builder.build();
	ext.execute_with(|| {
        let net_id = ETH_NETWORK_ID;
        let block_num = 8416395;
        state.run_next_offchain_with_params(block_num, frame_system::Pallet::<Runtime>::block_number() + 1,true);
        let raw_response = r#"{
"jsonrpc": "2.0",
  "id": 0,
  "result": [
    {
      "address": "0x077c2ec37d28709ce01ae740209bfbe185bd1eaa",
      "topics": [
        "0x85c0fa492ded927d3acca961da52b0dda1debb06d8c27fe189315f06bb6e26c8"
      ],
      "data": "0x3ec517d6e13491e575b6ab58bfb3c110a7782b4eb065d280a0d1725c4a850f440000000000000000000000000000000000000000000000000de0b6b3a7640000000000000000000000000000725c6b8cd3621eba4e0ccc40d532e7025b925a650000000000000000000000000000000000000000000000000000000000000000",
      "blockNumber": "0x808b3b",
      "transactionHash": "0xfb5ad3cc57f66d9903e70d23fb878634d7119bcff17d25944d21466500ce7238",
      "transactionIndex": "0x5",
      "blockHash": "0x3869ca9d4ad1871291ec2d736ad647e1a5dd2af8d5bf370fa32ec1c54b0502e5",
      "logIndex": "0x3",
      "removed": false
    }
  ]
}"#;
        state.push_response_raw(raw_response.as_bytes().to_owned());
        // "Wait" `CONFIRMATION_INTERVAL` blocks on sidechain.
        state.run_next_offchain_with_params(
            block_num + CONFIRMATION_INTERVAL,
            frame_system::Pallet::<Runtime>::block_number() + 1,true,
        );
        let tx_hash = H256(hex!("fb5ad3cc57f66d9903e70d23fb878634d7119bcff17d25944d21466500ce7238"));
        assert_eq!(
            crate::RequestStatuses::<Runtime>::get(net_id, tx_hash).unwrap(),
            RequestStatus::Done
        );
        let hash = crate::LoadToIncomingRequestHash::<Runtime>::get(net_id, &tx_hash);
        assert_eq!(
            crate::RequestStatuses::<Runtime>::get(net_id, hash).unwrap(),
            RequestStatus::Done
        );
    });
}

#[test]
fn ocw_should_not_import_pending_incoming_request() {
	let mut builder = ExtBuilder::new();
	builder.add_network(
		vec![AssetConfig::Reservable {
			id: AssetId::Balances,
			sidechain_id: sp_core::H160::from_str("40fd72257597aa14c7231a7b1aaa29fce868f677")
				.unwrap(),
		}],
		Some(vec![(AssetId::Balances, 350000)]),
		Some(2),
		Default::default(),
	);
	let (mut ext, mut state) = builder.build();
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let mut log = Log::default();
		log.topics = vec![types::H256(hex!(
			"85c0fa492ded927d3acca961da52b0dda1debb06d8c27fe189315f06bb6e26c8"
		))];
		let data = ethabi::encode(&[
			ethabi::Token::FixedBytes(alice.encode()),
			ethabi::Token::Uint(types::U256::from(100)),
			ethabi::Token::Address(types::EthAddress::from(
				crate::RegisteredSidechainToken::<Runtime>::get(net_id, AssetId::Balances)
					.unwrap()
					.0,
			)),
			ethabi::Token::FixedBytes(H256::zero().0.to_vec()),
		]);
		let tx_hash = H256([1; 32]);
		log.data = data.into();
		log.removed = Some(false);
		log.transaction_hash = Some(types::H256(tx_hash.0));
		log.block_number = Some(0u64.into());
		state.run_next_offchain_with_params(
			0,
			frame_system::Pallet::<Runtime>::block_number() + 1,
			true,
		);
		state.push_response([log]);
		// "Wait" `CONFIRMATION_INTERVAL` blocks on sidechain.
		state.run_next_offchain_with_params(
			CONFIRMATION_INTERVAL - 1,
			frame_system::Pallet::<Runtime>::block_number() + 1,
			true,
		);
		assert!(crate::RequestStatuses::<Runtime>::get(net_id, tx_hash).is_none(),);
	});
}

#[test]
fn should_not_register_and_finalize_incoming_request_twice() {
	let (mut ext, state) = ExtBuilder::default().build();
	ext.execute_with(|| {
		let net_id = ETH_NETWORK_ID;
		let bridge_acc_id = state.networks[&net_id].config.bridge_account_id.clone();
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		Assets::mint_to(&AssetId::Balances, &alice, &alice, 100000u32.into()).unwrap();
		let tx_hash = request_incoming(
			alice.clone(),
			H256::from_slice(&[1u8; 32]),
			IncomingTransactionRequestKind::Transfer.into(),
			net_id,
		)
		.unwrap();
		let incoming_transfer = IncomingRequest::Transfer(crate::IncomingTransfer {
			from: EthAddress::from([1; 20]),
			to: alice.clone(),
			asset_id: AssetId::Balances,
			asset_kind: AssetKind::Reservable,
			amount: 100u32.into(),
			author: alice.clone(),
			tx_hash,
			at_height: 1,
			timepoint: Default::default(),
			network_id: ETH_NETWORK_ID,
			should_take_fee: false,
		});
		assert_ok!(EthBridge::register_incoming_request(
			RuntimeOrigin::signed(bridge_acc_id.clone()),
			incoming_transfer.clone(),
		));
		assert_noop!(
			EthBridge::register_incoming_request(
				RuntimeOrigin::signed(bridge_acc_id.clone()),
				incoming_transfer.clone(),
			),
			DispatchErrorWithPostInfo {
				post_info: Pays::No.into(),
				error: Error::RequestIsAlreadyRegistered.into()
			}
		);
		let req_hash = crate::LoadToIncomingRequestHash::<Runtime>::get(net_id, tx_hash);
		assert_ok!(EthBridge::finalize_incoming_request(
			RuntimeOrigin::signed(bridge_acc_id.clone()),
			req_hash,
			net_id,
		));
	});
}

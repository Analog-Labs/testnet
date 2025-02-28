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

use sp_core::H160;

use super::mock::{ExtBuilder, Runtime};
use crate::common::AssetId;
use crate::requests::*;
use crate::{
	BridgeSignatureVersion, BridgeSignatureVersions, RegisteredAsset, RegisteredSidechainToken,
};
use bridge_multisig::{BridgeTimepoint, MultiChainHeight};
use polkadot_sdk::*;
use rustc_hex::ToHex;

fn assert_hex(bytes: &[u8], expected: &str) {
	assert_eq!(bytes.to_hex::<String>(), expected);
}

#[test]
fn should_encode_thischain_transfer() {
	let (mut ext, _state) = ExtBuilder::default().build();

	ext.execute_with(|| {
        for (version, expected) in [
            (BridgeSignatureVersion::V3, "0000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040fd72257597aa14c7231a7b1aaa29fce868f677000000000000000000000000000000000000000000000000000000000001d97d000000000000000000000000020202020202020202020202020202020202020200000000000000000000000001010101010101010101010101010101010101010303030303030303030303030303030303030303030303030303030303030303000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000087472616e73666572000000000000000000000000000000000000000000000000"),
        ] {
            BridgeSignatureVersions::<Runtime>::insert(0, version);
            RegisteredAsset::<Runtime>::insert(0, AssetId::Balances, AssetKind::Reservable);
            let request = OutgoingTransfer::<Runtime> {
                from: [1u8; 32].into(),
                to: [2u8; 20].into(),
                asset_id: AssetId::Balances,
                amount: 121213,
                nonce: 12,
                network_id: 0,
                timepoint: BridgeTimepoint {
                    height: MultiChainHeight::Thischain(12),
                    index: 13,
                },
            };
            let encoded = request.to_eth_abi([3u8; 32].into()).unwrap();
            assert_hex(&encoded.raw, expected);
        }
    });
}

#[test]
fn should_encode_sidechain_transfer() {
	let (mut ext, _state) = ExtBuilder::default().build();

	ext.execute_with(|| {
        for (version, expected) in [
            (BridgeSignatureVersion::V3, "000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000404040404040404040404040404040404040404000000000000000000000000000000000000000000000000000000000001d97d000000000000000000000000020202020202020202020202020202020202020200000000000000000000000001010101010101010101010101010101010101010505050505050505050505050505050505050505050505050505050505050505000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000087472616e73666572000000000000000000000000000000000000000000000000"),
        ] {
            BridgeSignatureVersions::<Runtime>::insert(0, version);
            RegisteredAsset::<Runtime>::insert(0, AssetId::Balances, AssetKind::Reservable);
            RegisteredSidechainToken::<Runtime>::insert(0, AssetId::Balances, H160::from([4u8; 20]));
            let request = OutgoingTransfer::<Runtime> {
                from: [1u8; 32].into(),
                to: [2u8; 20].into(),
                asset_id: AssetId::Balances,
                amount: 121213,
                nonce: 12,
                network_id: 0,
                timepoint: BridgeTimepoint {
                    height: MultiChainHeight::Thischain(12),
                    index: 13,
                },
            };
            let encoded = request.to_eth_abi([5u8; 32].into()).unwrap();
            assert_hex(&encoded.raw, expected);
        }
    });
}

#[test]
fn should_encode_add_peer() {
	let (mut ext, _state) = ExtBuilder::default().build();

	ext.execute_with(|| {
        for (version, expected) in [
            (BridgeSignatureVersion::V3, "00000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002020202020202020202020202020202020202020404040404040404040404040404040404040404040404040404040404040404000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000076164645065657200000000000000000000000000000000000000000000000000"),
        ] {
            BridgeSignatureVersions::<Runtime>::insert(0, version);
            let request = OutgoingAddPeer::<Runtime> {
                author: [1u8; 32].into(),
                peer_address: [2u8; 20].into(),
                peer_account_id: [3u8; 32].into(),
                nonce: 1213,
                network_id: 0,
                timepoint: BridgeTimepoint {
                    height: MultiChainHeight::Thischain(12),
                    index: 13,
                },
            };
            let encoded = request.to_eth_abi([4u8; 32].into()).unwrap();
            assert_hex(&encoded.raw, expected);
        }
    });
}

#[test]
fn should_encode_remove_peer() {
	let (mut ext, _state) = ExtBuilder::default().build();

	ext.execute_with(|| {
        for (version, expected) in [
            (BridgeSignatureVersion::V3, "00000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020202020202020202020202020202020202020204040404040404040404040404040404040404040404040404040404040404040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a72656d6f76655065657200000000000000000000000000000000000000000000"),
        ] {
            BridgeSignatureVersions::<Runtime>::insert(0, version);
            let request = OutgoingRemovePeer::<Runtime> {
                author: [1u8; 32].into(),
                peer_address: [2u8; 20].into(),
                peer_account_id: [3u8; 32].into(),
                nonce: 1213,
                network_id: 0,
                timepoint: BridgeTimepoint {
                    height: MultiChainHeight::Thischain(12),
                    index: 13,
                },
            };
            let encoded = request.to_eth_abi([4u8; 32].into()).unwrap();
            assert_hex(&encoded.raw, expected);
        }
    });
}

#[test]
fn should_encode_prepare_for_migration() {
	let (mut ext, _state) = ExtBuilder::default().build();

	ext.execute_with(|| {
        for (version, expected) in [
            (BridgeSignatureVersion::V3, "00000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000030303030303030303030303030303030303030303030303030303030303030300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010707265706172654d6967726174696f6e00000000000000000000000000000000"),
        ] {
            BridgeSignatureVersions::<Runtime>::insert(0, version);
            let request = OutgoingPrepareForMigration::<Runtime> {
                author: [1u8; 32].into(),
                nonce: 1213,
                network_id: 0,
                timepoint: BridgeTimepoint {
                    height: MultiChainHeight::Thischain(12),
                    index: 13,
                },
            };
            let encoded = request.to_eth_abi([3u8; 32].into()).unwrap();
            assert_hex(&encoded.raw, expected);
        }
    });
}

#[test]
fn should_encode_migrate() {
	let (mut ext, _state) = ExtBuilder::default().build();

	ext.execute_with(|| {
        for (version, expected) in [
            (BridgeSignatureVersion::V3, "00000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000060606060606060606060606060606060606060605050505050505050505050505050505050505050505050505050505050505050000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000076d696772617465000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003000000000000000000000000020202020202020202020202020202020202020200000000000000000000000003030303030303030303030303030303030303030000000000000000000000000404040404040404040404040404040404040404"),
        ] {
            BridgeSignatureVersions::<Runtime>::insert(0, version);
            let request = OutgoingMigrate::<Runtime> {
                author: [1u8; 32].into(),
                new_contract_address: [6u8; 20].into(),
                erc20_native_tokens: vec![[2u8; 20].into(), [3u8; 20].into(), [4u8; 20].into()],
                nonce: 1213,
                network_id: 0,
                timepoint: BridgeTimepoint {
                    height: MultiChainHeight::Thischain(12),
                    index: 13,
                },
                new_signature_version: BridgeSignatureVersion::V3,
            };
            let encoded = request.to_eth_abi([5u8; 32].into()).unwrap();
            assert_hex(&encoded.raw, expected);
        }
    });
}

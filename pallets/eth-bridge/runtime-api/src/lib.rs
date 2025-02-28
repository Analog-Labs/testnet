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

#![cfg_attr(not(feature = "std"), no_std)]
// TODO #167: fix clippy warnings
#![allow(clippy::all)]

use polkadot_sdk::*;
use scale_codec::Codec;
use sp_runtime::DispatchError;
use sp_std::prelude::*;

sp_api::decl_runtime_apis! {
	pub trait EthBridgeRuntimeApi<
		Hash,
		Approval,
		AccountId,
		AssetKind,
		AssetId,
		EthAddress,
		OffchainRequest,
		RequestStatus,
		OutgoingRequestEncoded,
		NetworkId,
		BalancePrecision,
> where
		Hash: Codec,
		Approval: Codec,
		AccountId: Codec,
		AssetKind: Codec,
		AssetId: Codec,
		EthAddress: Codec,
		OffchainRequest: Codec,
		RequestStatus: Codec,
		OutgoingRequestEncoded: Codec,
		NetworkId: Codec,
		BalancePrecision: Codec,
	{
		fn get_requests(hashes: Vec<Hash>, network_id: Option<NetworkId>, redirect_finished_load_requests: bool) -> Result<Vec<(OffchainRequest, RequestStatus)>, DispatchError>;
		fn get_approved_requests(hashes: Vec<Hash>, network_id: Option<NetworkId>) -> Result<Vec<(OutgoingRequestEncoded, Vec<Approval>)>, DispatchError>;
		fn get_approvals(hashes: Vec<Hash>, network_id: Option<NetworkId>) -> Result<Vec<Vec<Approval>>, DispatchError>;
		fn get_account_requests(account_id: AccountId, status_filter: Option<RequestStatus>) -> Result<Vec<(NetworkId, Hash)>, DispatchError>;
		fn get_registered_assets(network_id: Option<NetworkId>) -> Result<Vec<(AssetKind, (AssetId, BalancePrecision), Option<(EthAddress, BalancePrecision)>)>, DispatchError>;
	}
}

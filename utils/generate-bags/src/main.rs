// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Make the set of bag thresholds to be used with pallet-bags-list.

use clap::Parser;
use generate_bags::generate_thresholds;
use std::path::PathBuf;

#[derive(Debug, Parser)]
// #[clap(author, version, about)]
struct Opt {
	/// How many bags to generate.
	#[arg(long, default_value_t = 200)]
	n_bags: usize,

	/// Where to write the output.
	output: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
	let Opt { n_bags, output } = Opt::parse();
	generate_thresholds::<timechain_runtime::Runtime>(
		n_bags,
		&output,
		90_570_710 * timechain_runtime::ANLOG,
		timechain_runtime::EXISTENTIAL_DEPOSIT,
	)
}

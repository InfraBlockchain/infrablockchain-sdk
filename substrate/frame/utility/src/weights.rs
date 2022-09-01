// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-03, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/utility/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_utility.
pub trait WeightInfo {
	fn batch(c: u32, ) -> Weight;
	fn as_derivative() -> Weight;
	fn batch_all(c: u32, ) -> Weight;
	fn dispatch_as() -> Weight;
	fn force_batch(c: u32, ) -> Weight;
}

/// Weights for pallet_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		Weight::from_ref_time(23_113_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(2_701_000 as RefTimeWeight).saturating_mul(c as RefTimeWeight))
	}
	fn as_derivative() -> Weight {
		Weight::from_ref_time(4_182_000 as RefTimeWeight)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		Weight::from_ref_time(18_682_000 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(2_794_000 as RefTimeWeight).saturating_mul(c as RefTimeWeight))
	}
	fn dispatch_as() -> Weight {
		Weight::from_ref_time(12_049_000 as RefTimeWeight)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		Weight::from_ref_time(19_136_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(2_697_000 as RefTimeWeight).saturating_mul(c as RefTimeWeight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		Weight::from_ref_time(23_113_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(2_701_000 as RefTimeWeight).saturating_mul(c as RefTimeWeight))
	}
	fn as_derivative() -> Weight {
		Weight::from_ref_time(4_182_000 as RefTimeWeight)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		Weight::from_ref_time(18_682_000 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(2_794_000 as RefTimeWeight).saturating_mul(c as RefTimeWeight))
	}
	fn dispatch_as() -> Weight {
		Weight::from_ref_time(12_049_000 as RefTimeWeight)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		Weight::from_ref_time(19_136_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(2_697_000 as RefTimeWeight).saturating_mul(c as RefTimeWeight))
	}
}

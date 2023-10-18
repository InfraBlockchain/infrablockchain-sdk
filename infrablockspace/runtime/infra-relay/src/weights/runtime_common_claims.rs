// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_common::claims`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("infrablockspace-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/infrablockspace
// benchmark
// pallet
// --chain=infrablockspace-dev
// --steps=50
// --repeat=20
// --pallet=runtime_common::claims
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/infrablockspace/src/weights/runtime_common_claims.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_common::claims`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::claims::WeightInfo for WeightInfo<T> {
	/// Storage: Claims Claims (r:1 w:1)
	/// Proof Skipped: Claims Claims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Signing (r:1 w:1)
	/// Proof Skipped: Claims Signing (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Total (r:1 w:1)
	/// Proof Skipped: Claims Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Claims Vesting (r:1 w:1)
	/// Proof Skipped: Claims Vesting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `20301`
		// Minimum execution time: 143_551 nanoseconds.
		Weight::from_parts(147_538_000, 0)
			.saturating_add(Weight::from_parts(0, 20301))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Claims Total (r:1 w:1)
	/// Proof Skipped: Claims Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Claims Vesting (r:0 w:1)
	/// Proof Skipped: Claims Vesting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Claims (r:0 w:1)
	/// Proof Skipped: Claims Claims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Signing (r:0 w:1)
	/// Proof Skipped: Claims Signing (max_values: None, max_size: None, mode: Measured)
	fn mint_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `1223`
		// Minimum execution time: 10_986 nanoseconds.
		Weight::from_parts(11_519_000, 0)
			.saturating_add(Weight::from_parts(0, 1223))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Claims Claims (r:1 w:1)
	/// Proof Skipped: Claims Claims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Signing (r:1 w:1)
	/// Proof Skipped: Claims Signing (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Total (r:1 w:1)
	/// Proof Skipped: Claims Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Claims Vesting (r:1 w:1)
	/// Proof Skipped: Claims Vesting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn claim_attest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `618`
		//  Estimated: `20301`
		// Minimum execution time: 146_443 nanoseconds.
		Weight::from_parts(150_592_000, 0)
			.saturating_add(Weight::from_parts(0, 20301))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Claims Preclaims (r:1 w:1)
	/// Proof Skipped: Claims Preclaims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Signing (r:1 w:1)
	/// Proof Skipped: Claims Signing (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Claims (r:1 w:1)
	/// Proof Skipped: Claims Claims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Total (r:1 w:1)
	/// Proof Skipped: Claims Total (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Claims Vesting (r:1 w:1)
	/// Proof Skipped: Claims Vesting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn attest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `692`
		//  Estimated: `23764`
		// Minimum execution time: 69_307 nanoseconds.
		Weight::from_parts(84_145_000, 0)
			.saturating_add(Weight::from_parts(0, 23764))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Claims Claims (r:1 w:2)
	/// Proof Skipped: Claims Claims (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Vesting (r:1 w:2)
	/// Proof Skipped: Claims Vesting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Signing (r:1 w:2)
	/// Proof Skipped: Claims Signing (max_values: None, max_size: None, mode: Measured)
	/// Storage: Claims Preclaims (r:1 w:1)
	/// Proof Skipped: Claims Preclaims (max_values: None, max_size: None, mode: Measured)
	fn move_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438`
		//  Estimated: `11652`
		// Minimum execution time: 22_152 nanoseconds.
		Weight::from_parts(22_926_000, 0)
			.saturating_add(Weight::from_parts(0, 11652))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}

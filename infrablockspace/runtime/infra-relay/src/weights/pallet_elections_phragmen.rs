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
//! Autogenerated weights for `pallet_elections_phragmen`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/infrablockspace/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `465 + v * (80 ±0)`
		//  Estimated: `9590 + v * (320 ±0)`
		// Minimum execution time: 27_938 nanoseconds.
		Weight::from_parts(29_100_110, 0)
			.saturating_add(Weight::from_parts(0, 9590))
			// Standard Error: 6_600
			.saturating_add(Weight::from_parts(157_595, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 320).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `433 + v * (80 ±0)`
		//  Estimated: `9462 + v * (320 ±0)`
		// Minimum execution time: 38_534 nanoseconds.
		Weight::from_parts(39_643_343, 0)
			.saturating_add(Weight::from_parts(0, 9462))
			// Standard Error: 9_959
			.saturating_add(Weight::from_parts(126_648, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 320).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `465 + v * (80 ±0)`
		//  Estimated: `9590 + v * (320 ±0)`
		// Minimum execution time: 38_040 nanoseconds.
		Weight::from_parts(39_536_618, 0)
			.saturating_add(Weight::from_parts(0, 9590))
			// Standard Error: 11_329
			.saturating_add(Weight::from_parts(149_545, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 320).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Voting (r:1 w:1)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `955`
		//  Estimated: `7204`
		// Minimum execution time: 33_834 nanoseconds.
		Weight::from_parts(34_319_000, 0)
			.saturating_add(Weight::from_parts(0, 7204))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2485 + c * (48 ±0)`
		//  Estimated: `8934 + c * (144 ±0)`
		// Minimum execution time: 29_549 nanoseconds.
		Weight::from_parts(22_842_922, 0)
			.saturating_add(Weight::from_parts(0, 8934))
			// Standard Error: 850
			.saturating_add(Weight::from_parts(86_966, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 144).saturating_mul(c.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314 + c * (48 ±0)`
		//  Estimated: `796 + c * (48 ±0)`
		// Minimum execution time: 24_951 nanoseconds.
		Weight::from_parts(18_393_117, 0)
			.saturating_add(Weight::from_parts(0, 796))
			// Standard Error: 887
			.saturating_add(Weight::from_parts(58_381, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2692`
		//  Estimated: `15440`
		// Minimum execution time: 42_947 nanoseconds.
		Weight::from_parts(43_654_000, 0)
			.saturating_add(Weight::from_parts(0, 15440))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1806`
		//  Estimated: `2301`
		// Minimum execution time: 27_864 nanoseconds.
		Weight::from_parts(28_247_000, 0)
			.saturating_add(Weight::from_parts(0, 2301))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000_000 nanoseconds.
		Weight::from_parts(2_000_000_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2692`
		//  Estimated: `18043`
		// Minimum execution time: 58_914 nanoseconds.
		Weight::from_parts(60_596_000, 0)
			.saturating_add(Weight::from_parts(0, 18043))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: PhragmenElection Voting (r:10001 w:10000)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:0)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Candidates (r:1 w:0)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:10000 w:10000)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:10000 w:10000)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `36028 + v * (872 ±0)`
		//  Estimated: `149172 + v * (12340 ±0)`
		// Minimum execution time: 339_754_864 nanoseconds.
		Weight::from_parts(340_507_844_000, 0)
			.saturating_add(Weight::from_parts(0, 149172))
			// Standard Error: 288_237
			.saturating_add(Weight::from_parts(41_595_186, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_parts(0, 12340).saturating_mul(v.into()))
	}
	/// Storage: PhragmenElection Candidates (r:1 w:1)
	/// Proof Skipped: PhragmenElection Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Members (r:1 w:1)
	/// Proof Skipped: PhragmenElection Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection RunnersUp (r:1 w:1)
	/// Proof Skipped: PhragmenElection RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PhragmenElection Voting (r:10001 w:0)
	/// Proof Skipped: PhragmenElection Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:967 w:967)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PhragmenElection ElectionRounds (r:1 w:1)
	/// Proof Skipped: PhragmenElection ElectionRounds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + v * (639 ±0) + e * (28 ±0)`
		//  Estimated: `5352462 + c * (2560 ±0) + v * (5714 ±4) + e * (123 ±0)`
		// Minimum execution time: 35_362_437 nanoseconds.
		Weight::from_parts(35_569_288_000, 0)
			.saturating_add(Weight::from_parts(0, 5352462))
			// Standard Error: 456_165
			.saturating_add(Weight::from_parts(40_481_430, 0).saturating_mul(v.into()))
			// Standard Error: 29_273
			.saturating_add(Weight::from_parts(2_019_337, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(269))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2560).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 5714).saturating_mul(v.into()))
			.saturating_add(Weight::from_parts(0, 123).saturating_mul(e.into()))
	}
}

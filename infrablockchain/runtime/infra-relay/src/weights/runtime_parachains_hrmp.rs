// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `runtime_parachains::hrmp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::hrmp
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/runtime_parachains_hrmp.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_parachains::hrmp`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::hrmp::WeightInfo for WeightInfo<T> {
	/// Storage: Paras ParaLifecycles (r:2 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannels (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpEgressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	fn hrmp_init_open_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `704`
		//  Estimated: `6644`
		// Minimum execution time: 41_564_000 picoseconds.
		Weight::from_parts(42_048_000, 0)
			.saturating_add(Weight::from_parts(0, 6644))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpIngressChannelsIndex (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpIngressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpAcceptedChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	fn hrmp_accept_open_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `936`
		//  Estimated: `4401`
		// Minimum execution time: 43_570_000 picoseconds.
		Weight::from_parts(44_089_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Hrmp HrmpChannels (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpCloseChannelRequests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpCloseChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpCloseChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	fn hrmp_close_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `807`
		//  Estimated: `4272`
		// Minimum execution time: 36_594_000 picoseconds.
		Weight::from_parts(37_090_000, 0)
			.saturating_add(Weight::from_parts(0, 4272))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Hrmp HrmpIngressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpIngressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpEgressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpEgressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannels (r:254 w:254)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:0 w:1)
	/// Proof Skipped: Hrmp HrmpAcceptedChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannelContents (r:0 w:254)
	/// Proof Skipped: Hrmp HrmpChannelContents (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestCount (r:0 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[0, 127]`.
	/// The range of component `e` is `[0, 127]`.
	fn force_clean_hrmp(i: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264 + e * (100 ±0) + i * (100 ±0)`
		//  Estimated: `3726 + e * (2575 ±0) + i * (2575 ±0)`
		// Minimum execution time: 1_085_140_000 picoseconds.
		Weight::from_parts(1_100_901_000, 0)
			.saturating_add(Weight::from_parts(0, 3726))
			// Standard Error: 98_982
			.saturating_add(Weight::from_parts(3_229_112, 0).saturating_mul(i.into()))
			// Standard Error: 98_982
			.saturating_add(Weight::from_parts(3_210_944, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 2575).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 2575).saturating_mul(i.into()))
	}
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequests (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:256 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpIngressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpIngressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpEgressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpEgressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestCount (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpAcceptedChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannels (r:0 w:128)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_open(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `779 + c * (136 ±0)`
		//  Estimated: `2234 + c * (5086 ±0)`
		// Minimum execution time: 10_497_000 picoseconds.
		Weight::from_parts(6_987_455, 0)
			.saturating_add(Weight::from_parts(0, 2234))
			// Standard Error: 18_540
			.saturating_add(Weight::from_parts(18_788_534, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 5086).saturating_mul(c.into()))
	}
	/// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpCloseChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannels (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpEgressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpEgressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpIngressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpIngressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpCloseChannelRequests (r:0 w:128)
	/// Proof Skipped: Hrmp HrmpCloseChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannelContents (r:0 w:128)
	/// Proof Skipped: Hrmp HrmpChannelContents (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_close(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `335 + c * (124 ±0)`
		//  Estimated: `1795 + c * (2600 ±0)`
		// Minimum execution time: 6_575_000 picoseconds.
		Weight::from_parts(1_228_642, 0)
			.saturating_add(Weight::from_parts(0, 1795))
			// Standard Error: 14_826
			.saturating_add(Weight::from_parts(11_604_038, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2600).saturating_mul(c.into()))
	}
	/// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[0, 128]`.
	fn hrmp_cancel_open_request(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1026 + c * (13 ±0)`
		//  Estimated: `4295 + c * (15 ±0)`
		// Minimum execution time: 22_301_000 picoseconds.
		Weight::from_parts(26_131_473, 0)
			.saturating_add(Weight::from_parts(0, 4295))
			// Standard Error: 830
			.saturating_add(Weight::from_parts(49_448, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 15).saturating_mul(c.into()))
	}
	/// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequests (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[0, 128]`.
	fn clean_open_channel_requests(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243 + c * (63 ±0)`
		//  Estimated: `1722 + c * (2538 ±0)`
		// Minimum execution time: 5_234_000 picoseconds.
		Weight::from_parts(7_350_270, 0)
			.saturating_add(Weight::from_parts(0, 1722))
			// Standard Error: 3_105
			.saturating_add(Weight::from_parts(2_981_935, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2538).saturating_mul(c.into()))
	}
	/// Storage: Paras ParaLifecycles (r:2 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannels (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpEgressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:2 w:2)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:2 w:2)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpIngressChannelsIndex (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpIngressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpAcceptedChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	fn force_open_hrmp_channel(_c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `704`
		//  Estimated: `6644`
		// Minimum execution time: 55_611_000 picoseconds.
		Weight::from_parts(56_488_000, 0)
			.saturating_add(Weight::from_parts(0, 6644))
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:1 w:0)
	/// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
	/// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:2 w:2)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:2 w:2)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:1 w:0)
	/// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:1 w:1)
	/// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn establish_system_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `417`
		//  Estimated: `6357`
		// Minimum execution time: 629_674_000 picoseconds.
		Weight::from_parts(640_174_000, 0)
			.saturating_add(Weight::from_parts(0, 6357))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Hrmp::HrmpChannels` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn poke_channel_deposits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `263`
		//  Estimated: `3728`
		// Minimum execution time: 173_371_000 picoseconds.
		Weight::from_parts(175_860_000, 0)
			.saturating_add(Weight::from_parts(0, 3728))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
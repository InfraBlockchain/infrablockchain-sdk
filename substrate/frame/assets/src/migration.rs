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

use super::*;
use frame_support::traits::OnRuntimeUpgrade;
use log;

#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;

pub mod next_asset_id {
	use super::*;
	use sp_core::Get;

	/// Set [`NextAssetId`] to the value of `ID` if [`NextAssetId`] does not exist yet.
	pub struct SetNextAssetId<ID, T: Config<I>, I: 'static = ()>(
		core::marker::PhantomData<(ID, T, I)>,
	);
	impl<ID, T: Config<I>, I: 'static> OnRuntimeUpgrade for SetNextAssetId<ID, T, I>
	where
		T::AssetId: Incrementable,
		ID: Get<T::AssetId>,
	{
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			if !NextAssetId::<T, I>::exists() {
				NextAssetId::<T, I>::put(ID::get());
				T::DbWeight::get().reads_writes(1, 1)
			} else {
				T::DbWeight::get().reads(1)
			}
		}
	}
}

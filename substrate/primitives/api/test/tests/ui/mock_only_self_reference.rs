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

use substrate_test_runtime_client::runtime::Block;

sp_api::decl_runtime_apis! {
	pub trait Api {
		fn test(data: u64);
		fn test2(data: u64);
	}
}

struct MockApi;

sp_api::mock_impl_runtime_apis! {
	impl Api<Block> for MockApi {
		fn test(self, data: u64) {}

		fn test2(&mut self, data: u64) {}
	}
}

fn main() {}

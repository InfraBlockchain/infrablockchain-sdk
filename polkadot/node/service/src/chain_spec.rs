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

//! Polkadot chain configurations.

use polkadot_primitives::{AccountId, AccountPublic, AssignmentId, ValidatorId};
use sc_consensus_grandpa::AuthorityId as GrandpaId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;

use sc_chain_spec::ChainSpecExtension;
#[cfg(any(feature = "yosemite-native"))]
use sc_chain_spec::ChainType;
#[cfg(any(feature = "yosemite-native"))]
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::IdentifyAccount;
#[cfg(feature = "yosemite-native")]
use yosemite_runtime as yosemite;

#[cfg(feature = "yosemite-native")]
const YOSEMITE_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
#[cfg(any(feature = "yosemite-native"))]
const DEFAULT_PROTOCOL_ID: &str = "yosemite";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<polkadot_primitives::Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<polkadot_primitives::Block>,
	/// The light sync state.
	///
	/// This value will be set by the `sync-state rpc` implementation.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

// Generic chain spec, in case when we don't have the native runtime.
pub type GenericChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The `ChainSpec` parameterized for the rococo runtime.
#[cfg(feature = "yosemite-native")]
pub type YosemiteChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The `ChainSpec` parameterized for the rococo runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "yosemite-native"))]
pub type YosemiteChainspec = GenericChainSpec;

pub fn yosemite_config() -> Result<YosemiteChainSpec, String> {
	YosemiteChainSpec::from_json_bytes(&include_bytes!("../chain-specs/rococo.json")[..])
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
	BeefyId,
) {
	let keys = get_authority_keys_from_seed_no_beefy(seed);
	(keys.0, keys.1, keys.2, keys.3, keys.4, keys.5, keys.6, get_from_seed::<BeefyId>(seed))
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
	seed: &str,
) -> (AccountId, AccountId, BabeId, GrandpaId, ValidatorId, AssignmentId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ValidatorId>(seed),
		get_from_seed::<AssignmentId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Yosemite development config (single validator Alice)
#[cfg(feature = "yosemite-native")]
pub fn yosemite_development_config() -> Result<YosemiteChainSpec, String> {
	Ok(YosemiteChainSpec::builder(
		yosemite::WASM_BINARY.ok_or("Yosemite wasm not available")?,
		Default::default(),
	)
	.with_name("Yosemite Development")
	.with_id("yosemite_development")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_preset_name("dev")
	.with_protocol_id(DEFAULT_PROTOCOL_ID)
	.build())
}

/// Yosemite local testnet config.
#[cfg(feature = "yosemite-native")]
pub fn yosemite_local_testnet_config() -> Result<YosemiteChainSpec, String> {
	Ok(YosemiteChainSpec::builder(
		yosemite::fast_runtime_binary::WASM_BINARY.ok_or("Yosemite wasm not available")?,
		Default::default(),
	)
	.with_name("Yosemite Local Testnet")
	.with_id("yosemite_local_testnet")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name("local_testnet")
	.with_protocol_id(DEFAULT_PROTOCOL_ID)
	.build())
}

/// Yosemite staging testnet config.
#[cfg(feature = "yosemite-native")]
pub fn yosemite_staging_testnet_config() -> Result<YosemiteChainSpec, String> {
	Ok(YosemiteChainSpec::builder(
		yosemite::WASM_BINARY.ok_or("Yosemite wasm not available")?,
		Default::default(),
	)
	.with_name("Yosemite Staging Testnet")
	.with_id("yosemite_staging_testnet")
	.with_chain_type(ChainType::Live)
	.with_genesis_config_preset_name("staging_testnet")
	.with_telemetry_endpoints(
		TelemetryEndpoints::new(vec![(YOSEMITE_STAGING_TELEMETRY_URL.to_string(), 0)])
			.expect("Yosemite Staging telemetry url is valid; qed"),
	)
	.with_protocol_id(DEFAULT_PROTOCOL_ID)
	.build())
}

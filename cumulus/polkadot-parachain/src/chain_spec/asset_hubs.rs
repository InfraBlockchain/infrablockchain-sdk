// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use crate::chain_spec::{
	get_account_id_from_seed, get_collator_keys_from_seed, Extensions, GenericChainSpec,
	SAFE_XCM_VERSION,
};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachains_common::{AccountId, AuraId, Balance as AssetHubBalance};
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};

#[cfg(feature = "infra-parachain")]
const ASSET_HUB_YOSMITE_ED: AssetHubBalance = asset_hub_yosemite_runtime::ExistentialDeposit::get();

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
#[cfg(feature = "infra-parachain")]
pub fn asset_hub_yosemite_session_keys(keys: AuraId) -> asset_hub_yosemite_runtime::SessionKeys {
	asset_hub_yosemite_runtime::SessionKeys { aura: keys }
}

#[cfg(feature = "infra-parachain")]
pub fn asset_hub_yosemite_development_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenSymbol".into(), "BCL".into());
	properties.insert("tokenDecimals".into(), 12.into());
	asset_hub_yosemite_like_development_config(
		properties,
		"Asset Hub Yosemite Development",
		"asset-hub-yosemite-dev",
		1000,
	)
}

#[cfg(feature = "infra-parachain")]
fn asset_hub_yosemite_like_development_config(
	properties: sc_chain_spec::Properties,
	name: &str,
	chain_id: &str,
	para_id: u32,
) -> GenericChainSpec {
	GenericChainSpec::builder(
		asset_hub_yosemite_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "yosemite-dev".into(), para_id },
	)
	.with_name(name)
	.with_id(chain_id)
	.with_chain_type(ChainType::Development)
	.with_genesis_config_patch(asset_hub_yosemite_genesis(
		// initial collators.
		vec![(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_collator_keys_from_seed::<AuraId>("Alice"),
		)],
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		],
		testnet_parachains_constants::yosemite::currency::UNITS * 1_000_000,
		Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
		para_id.into(),
	))
	.with_properties(properties)
	.build()
}

#[cfg(feature = "infra-parachain")]
pub fn asset_hub_yosemite_local_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenSymbol".into(), "BCL".into());
	properties.insert("tokenDecimals".into(), 12.into());
	asset_hub_yosemite_like_local_config(
		properties,
		"Asset Hub Yosemite Testnet",
		"asset-hub-yosemite-testnet",
		1000,
	)
}

#[cfg(feature = "infra-parachain")]
fn asset_hub_yosemite_like_local_config(
	properties: sc_chain_spec::Properties,
	name: &str,
	chain_id: &str,
	para_id: u32,
) -> GenericChainSpec {
	GenericChainSpec::builder(
		asset_hub_yosemite_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "yosemite-local".into(), para_id },
	)
	.with_name(name)
	.with_id(chain_id)
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(asset_hub_yosemite_genesis(
		// initial collators.
		vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_collator_keys_from_seed::<AuraId>("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_collator_keys_from_seed::<AuraId>("Bob"),
			),
		],
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		],
		testnet_parachains_constants::yosemite::currency::UNITS * 1_000_000,
		Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
		para_id.into(),
	))
	.with_properties(properties)
	.build()
}

#[cfg(feature = "infra-parachain")]
pub fn asset_hub_yosemite_genesis_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "BCL".into());
	properties.insert("tokenDecimals".into(), 12.into());
	let para_id = 1000;
	GenericChainSpec::builder(
		asset_hub_yosemite_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "yosemite".into(), para_id },
	)
	.with_name("Asset Hub Yosemite")
	.with_id("asset-hub-yosemite")
	.with_chain_type(ChainType::Live)
	.with_genesis_config_patch(asset_hub_yosemite_genesis(
		// initial collators.
		vec![
			// E8XC6rTJRsioKCp6KMy6zd24ykj4gWsusZ3AkSeyavpVBAG
			(
				hex!("44cb62d1d6cdd2fff2a5ef3bb7ef827be5b3e117a394ecaa634d8dd9809d5608").into(),
				hex!("44cb62d1d6cdd2fff2a5ef3bb7ef827be5b3e117a394ecaa634d8dd9809d5608")
					.unchecked_into(),
			),
			// G28iWEybndgGRbhfx83t7Q42YhMPByHpyqWDUgeyoGF94ri
			(
				hex!("9864b85e23aa4506643db9879c3dbbeabaa94d269693a4447f537dd6b5893944").into(),
				hex!("9864b85e23aa4506643db9879c3dbbeabaa94d269693a4447f537dd6b5893944")
					.unchecked_into(),
			),
			// G839e2eMiq7UXbConsY6DS1XDAYG2XnQxAmLuRLGGQ3Px9c
			(
				hex!("9ce5741ee2f1ac3bdedbde9f3339048f4da2cb88ddf33a0977fa0b4cf86e2948").into(),
				hex!("9ce5741ee2f1ac3bdedbde9f3339048f4da2cb88ddf33a0977fa0b4cf86e2948")
					.unchecked_into(),
			),
			// GLao4ukFUW6qhexuZowdFrKa2NLCfnEjZMftSXXfvGv1vvt
			(
				hex!("a676ed15f5a325eab49ed8d5f8c00f3f814b19bb58cda14ad10894c078dd337f").into(),
				hex!("a676ed15f5a325eab49ed8d5f8c00f3f814b19bb58cda14ad10894c078dd337f")
					.unchecked_into(),
			),
		],
		Vec::new(),
		ASSET_HUB_YOSMITE_ED * 524_288,
		None,
		para_id.into(),
	))
	.with_properties(properties)
	.build()
}

#[cfg(feature = "infra-parachain")]
fn asset_hub_yosemite_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	endowment: AssetHubBalance,
	root_key: Option<AccountId>,
	id: ParaId,
) -> serde_json::Value {
	serde_json::json!({
		"balances": asset_hub_yosemite_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, endowment))
				.collect(),
		},
		"parachainInfo": asset_hub_yosemite_runtime::ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		"collatorSelection": asset_hub_yosemite_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: ASSET_HUB_YOSMITE_ED * 16,
			..Default::default()
		},
		"session": asset_hub_yosemite_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                         // account id
						acc,                                 // validator id
						asset_hub_yosemite_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		"infraXcm": asset_hub_yosemite_runtime::InfraXcmConfig {
			  safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		"sudo": asset_hub_yosemite_runtime::SudoConfig { key: root_key }
	})
}

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

//! Genesis configs presets for the Rococo runtime

use crate::{SessionKeys, BABE_GENESIS_EPOCH_CONFIG};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use polkadot_primitives::{
	vstaging::SchedulerParams, AccountId, AccountPublic, AssignmentId,
	SystemConfig as InfraSystemConfig, ValidatorId,
};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{
	infra::{BaseSystemTokenDetail, Fiat},
	traits::IdentifyAccount,
};
#[cfg(not(feature = "std"))]
use sp_std::alloc::format;
use sp_std::vec::Vec;
use yosemite_runtime_constants::currency::DOLLARS;

/// Helper function to generate a crypto pair from seed
fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
	BeefyId,
) {
	let keys = get_authority_keys_from_seed_no_beefy(seed);
	(keys.0, keys.1, keys.2, keys.3, keys.4, keys.5, keys.6, keys.7, get_from_seed::<BeefyId>(seed))
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<ValidatorId>(seed),
		get_from_seed::<AssignmentId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
	Vec::from([
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
	])
}

fn rococo_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, para_validator, para_assignment, authority_discovery }
}

fn default_parachains_host_configuration(
) -> polkadot_runtime_parachains::configuration::HostConfiguration<polkadot_primitives::BlockNumber>
{
	use polkadot_primitives::{
		node_features::FeatureIndex, AsyncBackingParams, MAX_CODE_SIZE, MAX_POV_SIZE,
	};

	polkadot_runtime_parachains::configuration::HostConfiguration {
		validation_upgrade_cooldown: 2u32,
		validation_upgrade_delay: 2,
		code_retention_period: 1200,
		max_code_size: MAX_CODE_SIZE,
		max_pov_size: MAX_POV_SIZE,
		max_head_data_size: 32 * 1024,
		max_upward_queue_count: 8,
		max_upward_queue_size: 1024 * 1024,
		max_downward_message_size: 1024 * 1024,
		max_upward_message_size: 50 * 1024,
		max_upward_message_num_per_candidate: 5,
		hrmp_sender_deposit: 0,
		hrmp_recipient_deposit: 0,
		hrmp_channel_max_capacity: 8,
		hrmp_channel_max_total_size: 8 * 1024,
		hrmp_max_parachain_inbound_channels: 4,
		hrmp_channel_max_message_size: 1024 * 1024,
		hrmp_max_parachain_outbound_channels: 4,
		hrmp_max_message_num_per_candidate: 5,
		dispute_period: 6,
		no_show_slots: 2,
		n_delay_tranches: 25,
		needed_approvals: 2,
		relay_vrf_modulo_samples: 2,
		zeroth_delay_tranche_width: 0,
		minimum_validation_upgrade_delay: 5,
		async_backing_params: AsyncBackingParams {
			max_candidate_depth: 3,
			allowed_ancestry_len: 2,
		},
		node_features: bitvec::vec::BitVec::from_element(
			1u8 << (FeatureIndex::ElasticScalingMVP as usize),
		),
		scheduler_params: SchedulerParams {
			lookahead: 2,
			group_rotation_frequency: 20,
			paras_availability_period: 4,
			..Default::default()
		},
		..Default::default()
	}
}

fn default_infra_relay_system_configuration() -> InfraSystemConfig {
	InfraSystemConfig {
		base_system_token_detail: BaseSystemTokenDetail {
			base_currency: Fiat::USD,
			base_weight: 1_000_000,
			base_decimals: 4,
		},
		weight_scale: 25,
		base_para_fee_rate: 1_000_000,
	}
}

#[test]
fn default_parachains_host_configuration_is_consistent() {
	default_parachains_host_configuration().panic_if_not_consistent();
}

fn yosemite_testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> serde_json::Value {
	use pallet_validator_management::{Forcing, Pool};
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * DOLLARS;

	serde_json::json!({
	   "balances": {
			"balances": endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect::<Vec<_>>()
		},
		"session": {
		  "keys": initial_authorities
					.iter()
					.map(|x| {
						(
							x.0.clone(),
							x.0.clone(),
							rococo_session_keys(
								x.2.clone(),
								x.3.clone(),
								x.4.clone(),
								x.5.clone(),
								x.6.clone(),
								x.7.clone(),
							),
						)
					})
					.collect::<Vec<_>>()
		},
		"sudo": {
		  "key": Some(root_key)
		},
		"babe": {
			"epochConfig": Some(BABE_GENESIS_EPOCH_CONFIG),
		},
		"configuration": {
			"config": default_parachains_host_configuration(),
			"systemConfig": default_infra_relay_system_configuration(),
		},
		"validatorManagement": {
		  "seedTrustValidators": initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>(),
			"totalValidatorSlots": 6,
			"seedTrustSlots": 6,
			"forceEra": Forcing::NotForcing,
			"poolStatus": Pool::SeedTrust,
			"isPotEnableAtGenesis": false,
		}
	})
}

//development
fn yosemite_development_config_genesis() -> serde_json::Value {
	yosemite_testnet_genesis(
		Vec::from([
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		]),
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

//local_testnet
fn yosemite_local_testnet_genesis() -> serde_json::Value {
	yosemite_testnet_genesis(
		Vec::from([
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
			get_authority_keys_from_seed_no_beefy("Charlie"),
			get_authority_keys_from_seed_no_beefy("Dave"),
			get_authority_keys_from_seed_no_beefy("Eve"),
			get_authority_keys_from_seed_no_beefy("Ferdie"),
		]),
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &sp_genesis_builder::PresetId) -> Option<sp_std::vec::Vec<u8>> {
	let patch = match id.try_into() {
		Ok("dev") => yosemite_development_config_genesis(),
		Ok("local_testnet") => yosemite_local_testnet_genesis(),
		Ok("staging_testnet") => yosemite_local_testnet_genesis(),
		Ok("mainnet") => yosemite_local_testnet_genesis(),
 		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

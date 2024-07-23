use super::*;
use frame_support::traits::fungibles::Mutate;
use codec::{Decode, Encode};
use xcm::{latest::prelude::*, opaque::lts::Junctions::X1};

#[derive(Encode, Decode)]
pub enum ParachainRuntimePallets {
	#[codec(index = 2)]
	InfraParaCore(InfraParaCoreCalls),
	#[codec(index = 55)]
	Oracle(OracleCalls),
}

#[derive(Encode, Decode)]
pub enum InfraParaCoreCalls {
	#[codec(index = 0)]
	SetAdmin(AccountId),
	#[codec(index = 1)]
	UpdateFeeTable(Vec<u8>, Vec<u8>, SystemTokenBalance),
	#[codec(index = 2)]
	UpdateParaFeeRate(SystemTokenBalance),
	#[codec(index = 3)]
	UpdateRuntimeState,
	#[codec(index = 4)]
	RegisterSystemToken(Location, SystemTokenWeight),
	#[codec(index = 5)]
	CreateWrapped(AccountId, Location, Fiat, Balance, Vec<u8>, Vec<u8>, u8, SystemTokenWeight),
	#[codec(index = 6)]
	DeregisterSystemToken(Location),
	#[codec(index = 7)]
	SuspendSystemToken(Location),
	#[codec(index = 8)]
	UnsuspendSystemToken(Location),
	#[codec(index = 9)]
	DistriubteReward(AccountId, Location, SystemTokenWeight),
}

#[derive(Encode, Decode)]
pub enum OracleCalls {
	#[codec(index = 1)]
	RequestFiat(Vec<Fiat>),
}

/// Main actor for handling policy of paracahain configuration
pub struct ParaConfigHandler;

impl ParaConfigInterface for ParaConfigHandler {
	type AccountId = AccountId;
	type DestId = u32;
	type Balance = SystemTokenBalance;

	fn set_admin(dest_id: Self::DestId, who: Self::AccountId) {
		let set_admin_call =
			ParachainRuntimePallets::InfraParaCore(InfraParaCoreCalls::SetAdmin(who));
		send_xcm_for(true, set_admin_call.encode(), dest_id);
	}

	fn update_fee_table(
		dest_id: Self::DestId,
		pallet_name: Vec<u8>,
		call_name: Vec<u8>,
		fee: Self::Balance,
	) {
		let set_fee_table_call = ParachainRuntimePallets::InfraParaCore(
			InfraParaCoreCalls::UpdateFeeTable(pallet_name, call_name, fee),
		);
		send_xcm_for(true, set_fee_table_call.encode(), dest_id);
	}

	fn update_para_fee_rate(dest_id: Self::DestId, fee_rate: Self::Balance) {
		let set_fee_rate_call =
			ParachainRuntimePallets::InfraParaCore(InfraParaCoreCalls::UpdateParaFeeRate(fee_rate));
		send_xcm_for(true, set_fee_rate_call.encode(), dest_id);
	}

	fn update_runtime_state(dest_id: Self::DestId) {
		let set_runtime_state_call =
			ParachainRuntimePallets::InfraParaCore(InfraParaCoreCalls::UpdateRuntimeState);
		send_xcm_for(true, set_runtime_state_call.encode(), dest_id);
	}
}

pub struct OracleManager;
impl OracleInterface for OracleManager {
	type DestId = u32;

	fn request_fiat(dest_id: Self::DestId, fiat: Vec<Fiat>) {
		if dest_id != AssetHubId::get() {
			return;
		}
		let request_fiat_call = ParachainRuntimePallets::Oracle(OracleCalls::RequestFiat(fiat));
		send_xcm_for(false, request_fiat_call.encode(), dest_id);
	}
}

/// Main actor for handling System Token related calls
pub struct SystemTokenHandler;
impl SystemTokenInterface for SystemTokenHandler {
	type AccountId = AccountId;
	type Location = Location;
	type Balance = SystemTokenBalance;
	type SystemTokenWeight = SystemTokenWeight;
	type DestId = u32;

	fn register_system_token(
		dest_id: Self::DestId,
		system_token_id: Self::Location,
		system_token_weight: Self::SystemTokenWeight,
	) {
		let register_call = ParachainRuntimePallets::InfraParaCore(
			InfraParaCoreCalls::RegisterSystemToken(system_token_id, system_token_weight),
		);
		send_xcm_for(true, register_call.encode(), dest_id);
	}

	fn deregister_system_token(dest_id: Self::DestId, system_token_id: Self::Location) {
		let deregister_call = ParachainRuntimePallets::InfraParaCore(
			InfraParaCoreCalls::DeregisterSystemToken(system_token_id),
		);
		send_xcm_for(true, deregister_call.encode(), dest_id);
	}

	fn create_wrapped(
		dest_id: Self::DestId,
		owner: Self::AccountId,
		original: Self::Location,
		currency_type: Fiat,
		min_balance: Self::Balance,
		name: Vec<u8>,
		symbol: Vec<u8>,
		decimals: u8,
		system_token_weight: Self::SystemTokenWeight,
	) {
		let create_call =
			ParachainRuntimePallets::InfraParaCore(InfraParaCoreCalls::CreateWrapped(
				owner,
				original,
				currency_type,
				min_balance,
				name,
				symbol,
				decimals,
				system_token_weight,
			));
		send_xcm_for(true, create_call.encode(), dest_id);
	}

	fn suspend_system_token(dest_id: Self::DestId, asset_id: Self::Location) {
		let suspend_call = ParachainRuntimePallets::InfraParaCore(
			InfraParaCoreCalls::SuspendSystemToken(asset_id),
		);
		send_xcm_for(true, suspend_call.encode(), dest_id);
	}

	fn unsuspend_system_token(dest_id: Self::DestId, asset_id: Self::Location) {
		let unsuspend_call = ParachainRuntimePallets::InfraParaCore(
			InfraParaCoreCalls::UnsuspendSystemToken(asset_id),
		);
		send_xcm_for(true, unsuspend_call.encode(), dest_id);
	}
}

pub struct RewardHandler;
impl RewardInterface for RewardHandler {
	type DestId = u32;
	type AccountId = AccountId;
	type AssetKind = Location;
	type Balance = SystemTokenBalance;
	type Fungibles = NativeAndForeignAssets;

	fn distribute_reward(
		who: Self::AccountId,
		reward: Reward<Self::DestId, Self::AssetKind, Self::Balance>,
	) {
		let Reward { origin, asset, amount } = reward;
		let origin: Option<Self::DestId> = origin.into();
		if let Some(dest_id) = origin {
			// Handle remote asset
			let target = Location::new(0, [Parachain(dest_id)]);
			let context = UniversalLocation::get();
			let mut reanchored = asset.clone();
			if let Err(_) = reanchored.reanchor(&target, &context) {
				// Something went wrong
				log::error!("Failed to reanchor asset remotely.");
				return;
			};
			let distribute_reward_call = ParachainRuntimePallets::InfraParaCore(
				InfraParaCoreCalls::DistriubteReward(who, reanchored, amount),
			);
			send_xcm_for(true, distribute_reward_call.encode(), dest_id);
		} else {
			// Handle local asset
			let source: AccountId = FeeTreasuryId::get().into_account_truncating();
			if let Err(_) = <Self::Fungibles as Mutate<AccountId>>::transfer(
				asset, &source, &who, amount, Preserve,
			) {
				log::error!("❌❌ Error on transfering reward from {:?} to {:?} ❌❌", source, who);
			};
		}
	}
}

pub(super) fn send_xcm_for(is_native: bool, call: Vec<u8>, dest_id: u32) {
	let message = Xcm(vec![
		Instruction::UnpaidExecution { weight_limit: WeightLimit::Unlimited, check_origin: None },
		Instruction::Transact {
			origin_kind: if is_native { OriginKind::Native } else { OriginKind::Superuser },
			require_weight_at_most: Weight::from_parts(1_000_000_000, 200000),
			call: call.into(),
		},
	]);

	match XcmPallet::send_xcm(Here, Location::new(0, [Parachain(dest_id)]), message.clone()) {
		Ok(_) => log::info!(
			target: "runtime::parachain-config",
			"Instruction sent successfully."
		),
		Err(e) => log::error!(
			target: "runtime::parachain-config",
			"Error on sending XCM to parachain {:?} => {:?}",
			dest_id, e
		),
	}
}

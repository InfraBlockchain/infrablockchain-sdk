
use super::*;
use xcm::latest::prelude::*;
use parity_scale_codec::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum ParachainRuntimePallets {
	#[codec(index = 2)]
	InfraParaCore(ParachainCalls),
}

#[derive(Encode, Decode)]
pub enum ParachainCalls {
    #[codec(index = 1)]
	UpdateFeeTable(Vec<u8>, Vec<u8>, SystemTokenBalance),
	#[codec(index = 2)]
	UpdateParaFeeRate(SystemTokenBalance),
	#[codec(index = 3)]
	UpdateRuntimeState,
	#[codec(index = 4)]
	RegisterSystemToken(MultiLocation, SystemTokenWeight),
	#[codec(index = 5)]
	CreateWrappedLocal(MultiLocation, Fiat, Balance, Vec<u8>, Vec<u8>, u8, SystemTokenWeight),
	#[codec(index = 6)]
	DeregisterSystemToken(MultiLocation),
}

/// Main actor for handling policy of paracahain configuration
pub struct ParaConfigHandler;

impl ParaConfigInterface for ParaConfigHandler {
    type DestId = u32;
    type Balance = SystemTokenBalance;

    fn update_fee_table(dest_id: Self::DestId, pallet_name: Vec<u8>, call_name: Vec<u8>, fee: Self::Balance) {
        let set_fee_table_call = ParachainRuntimePallets::InfraParaCore(
            ParachainCalls::UpdateFeeTable(pallet_name, call_name, fee),
        );
        send_xcm_for(set_fee_table_call.encode(), dest_id);
    }
    
    fn update_para_fee_rate(dest_id: Self::DestId, fee_rate: Self::Balance) {
        let set_fee_rate_call = ParachainRuntimePallets::InfraParaCore(
            ParachainCalls::UpdateParaFeeRate(fee_rate),
        );
        send_xcm_for(set_fee_rate_call.encode(), dest_id);
    }
    
    fn update_runtime_state(dest_id: Self::DestId) {
        let set_runtime_state_call = ParachainRuntimePallets::InfraParaCore(
            ParachainCalls::UpdateRuntimeState,
        );
        send_xcm_for(set_runtime_state_call.encode(), dest_id);
    }
}

/// Main actor for handling System Token related calls
pub struct SystemTokenHandler;
impl SystemTokenInterface for SystemTokenHandler {
    type Location = MultiLocation;
    type Balance = SystemTokenBalance;
    type SystemTokenWeight = SystemTokenWeight;
    type DestId = u32;

    fn register_system_token(
        dest_id: Self::DestId,
        system_token_id: Self::Location,
        system_token_weight: Self::SystemTokenWeight,
    ) {
        let register_call = ParachainRuntimePallets::InfraParaCore(
            ParachainCalls::RegisterSystemToken(system_token_id, system_token_weight),
        );
        send_xcm_for(register_call.encode(), dest_id);
    }

    fn deregister_system_token(dest_id: Self::DestId, system_token_id: Self::Location) {
        let deregister_call = ParachainRuntimePallets::InfraParaCore(
            ParachainCalls::DeregisterSystemToken(system_token_id),
        );
        send_xcm_for(deregister_call.encode(), dest_id);
    }

    fn create_wrapped(
        dest_id: Self::DestId,
        original: Self::Location,
        currency_type: Fiat,
        min_balance: Self::Balance,
        name: Vec<u8>,
        symbol: Vec<u8>,
        decimals: u8,
        system_token_weight: Self::SystemTokenWeight,
    ) {
        let create_call = ParachainRuntimePallets::InfraParaCore(
            ParachainCalls::CreateWrappedLocal(
                original,
                currency_type,
                min_balance,
                name,
                symbol,
                decimals,
                system_token_weight,
            ));
        send_xcm_for(create_call.encode(), dest_id);
    }
}

pub(super) fn send_xcm_for(call: Vec<u8>, dest_id: u32) {
		let message = Xcm(vec![
			Instruction::UnpaidExecution {
				weight_limit: WeightLimit::Unlimited,
				check_origin: None,
			},
			Instruction::Transact {
				origin_kind: OriginKind::Native,
				require_weight_at_most: Weight::from_parts(1_000_000_000, 200000),
				call: call.into(),
			},
		]);

		match XcmPallet::send_xcm(
            Here,
			MultiLocation::new(0, X1(Parachain(dest_id))),
			message.clone(),
		) {
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
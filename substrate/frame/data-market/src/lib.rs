#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;
pub use types::*;

use frame_support::{
	pallet_prelude::*,
	traits::{
		fungibles::{Inspect, Mutate},
		tokens::Preservation,
		ConstU32, Get,
	},
	BoundedVec, PalletId,
};
use frame_system::{pallet_prelude::*, Config as SystemConfig};
pub use pallet::*;
use sp_runtime::{traits::AccountIdConversion, BoundedBTreeMap};
use sp_std::{prelude::*, vec, vec::Vec};

#[frame_support::pallet(dev_mode)]
pub mod pallet {

	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Origin for admin-level operations.
		type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// The type used to tokenize the asset balance.
		type Assets: Inspect<Self::AccountId> + Mutate<Self::AccountId>;

		// The maximum quantity of data that can be purchased
		#[pallet::constant]
		type MaxPurchaseQuantity: Get<Quantity>;
	}

	// The Next value of contract id
	#[pallet::storage]
	#[pallet::getter(fn get_next_contract_id)]
	pub(super) type NextContractId<T: Config> = StorageValue<_, ContractId, ValueQuery>;

	// The Data Delegate Contracts
	#[pallet::storage]
	#[pallet::getter(fn get_data_delegate_contracts)]
	pub(super) type DataDelegateContracts<T: Config> = StorageMap<
		_,
		Twox64Concat,
		ContractId,
		DataDelegateContractDetail<T::AccountId, BlockNumberFor<T>>,
		OptionQuery,
	>;

	// The Data Purchase Contracts
	#[pallet::storage]
	#[pallet::getter(fn get_data_purchase_contracts)]
	pub(super) type DataPurchaseContracts<T: Config> = StorageMap<
		_,
		Twox64Concat,
		ContractId,
		DataPurchaseContractDetail<T::AccountId, BlockNumberFor<T>, AssetBalanceOf<T>>,
		OptionQuery,
	>;

	// The Data Delegate Contract List
	#[pallet::storage]
	#[pallet::getter(fn get_data_delegate_contract_list)]
	pub(super) type DataDelegateContractList<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Vec<ContractId>, ValueQuery>;

	// The Data Purchase Contract List
	#[pallet::storage]
	#[pallet::getter(fn get_data_purchase_contract_list)]
	pub(super) type DataPurchaseContractList<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Vec<ContractId>, ValueQuery>;

	// Agency list
	#[pallet::storage]
	#[pallet::getter(fn get_agencies)]
	pub(super) type Agencies<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	// The Config of the platform
	#[pallet::storage]
	#[pallet::getter(fn get_platform_config)]
	pub(super) type PlatformConfig<T: Config> = StorageValue<_, MarketConfiguration, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Register Agency
		RegisterAgency {
			agency: T::AccountId,
		},
		// Deregister Agency
		DeregisterAgency {
			agency: T::AccountId,
		},
		// Make Data Delegate Contract
		MakeDataDelegateContract {
			contract_id: ContractId,
			agency: T::AccountId,
		},
		// Sign Data Delegate Contract
		SignDateDelegateContract {
			contract_id: ContractId,
			data_owner: T::AccountId,
		},
		// Make Data Purchase Contract
		MakeDataPurchaseContract {
			contract_id: ContractId,
			data_buyer: T::AccountId,
		},
		// Sign Data Purchase Contract
		SignDataPurchaseContract {
			contract_id: ContractId,
			agency: T::AccountId,
			data_verifier: T::AccountId,
		},
		// Pending Contract Terminate
		PendingContractTerminate {
			contract_type: ContractType,
			contract_id: ContractId,
		},
		// Contract Terminated
		ContractTerminated {
			contract_type: ContractType,
			contract_id: ContractId,
		},
		// Data Trade Executed
		DataTradeExecuted {
			contract_id: ContractId,
			data_owner: T::AccountId,
			data_issuer: Vec<(T::AccountId, IssuerWeight)>,
			data_owner_fee: u128,
			data_issuer_fee: u128,
			platform_fee: u128,
			data_verification_proof: VerificationProof<AnyText>,
		},
		// Set Platform Config
		SetPlatformConfig {
			config: MarketConfiguration,
		},
		// Add More Balance in Deposit
		DepositBalance {
			contract_id: ContractId,
			amount: AssetBalanceOf<T>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Overflow for NextPurchaseId.
		Overflow,
		/// Error that the total trade limit has been reached.
		TradeLimitReached,
		/// Contract period is invalid.
		InvalidPeriod,
		/// Error failed to the existing contract.
		ContractNotExist,
		/// Error failed to the existing contract status.
		ContractStatusNotExist,
		/// Error failed to the existing contract signer.
		ContractSignerNotExist,
		/// Purchase has already been finished.
		ContractNotActive,
		/// Origin is different with data Owner.
		InvalidOwner,
		/// Origin is different with data buyer.
		InvalidBuyer,
		/// Verifier of the origin is invalid.
		InvalidVerifier,
		/// Agency of the origin is invalid.
		InvalidAgency,
		/// Cannot sign the contract.
		NotSigned,
		/// Issuer weight should be greater than zero
		IssuerWeightInvalid,
		/// Max verifier members exceed
		MaxVeirifierMembersExceed,
		/// Invalid calculation of fee ratio
		InvalidFeeRatio,
		/// Exceed Contract Signer
		ExceedContractSigner,
		/// Config is invalid
		InvalidConfig,
		/// The Data Owner has already sold data
		AlreadyPurchased,
	}

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub _config: sp_std::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			// Set as default configuration
			let default =
				MarketConfiguration { total_fee_ratio: 10000, min_platform_fee_ratio: 1000 };

			PlatformConfig::<T>::put(default);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		AssetBalanceOf<T>: From<u128> + Into<u128>,
		AssetIdOf<T>: From<u32>,
	{
		/// Make a delegate contract
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `detail`: The detail of the contract.
		#[pallet::call_index(0)]
		pub fn make_delegate_contract(
			origin: OriginFor<T>,
			params: DataDelegateContractParams<T::AccountId, BlockNumberFor<T>>,
		) -> DispatchResult {
			let maybe_agency = ensure_signed(origin)?;
			Self::do_make_delegate_contract(maybe_agency, params)?;
			Ok(())
		}

		/// Sign a delegate contract
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `contract_id`: The id of the contract.
		#[pallet::call_index(1)]
		pub fn sign_delegate_contract(
			origin: OriginFor<T>,
			contract_id: ContractId,
		) -> DispatchResult {
			let maybe_owner = ensure_signed(origin)?;
			Self::do_sign_delegate_contract(maybe_owner, contract_id)?;
			Ok(())
		}

		/// Make a purchase contract
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `detail`: The detail of the contract.
		#[pallet::call_index(2)]
		pub fn make_purchase_contract(
			origin: OriginFor<T>,
			params: DataPurchaseContractParams<T::AccountId, BlockNumberFor<T>, AssetBalanceOf<T>>,
			is_agency_exist: bool,
		) -> DispatchResult {
			let maybe_buyer = ensure_signed(origin)?;
			Self::do_make_purchase_contract(maybe_buyer, params, is_agency_exist)?;
			Ok(())
		}

		/// Sign a purchase contract
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `contract_id`: The id of the contract.
		/// - `data_verifier`: The verifier of the contract.
		#[pallet::call_index(3)]
		pub fn sign_purchase_contract(
			origin: OriginFor<T>,
			contract_id: ContractId,
			data_verifier: T::AccountId,
		) -> DispatchResult {
			let maybe_agency = ensure_signed(origin)?;
			Self::do_sign_purchase_contract(maybe_agency, contract_id, data_verifier)?;
			Ok(())
		}

		/// Terminate a delegate contract
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `contract_id`: The id of the contract.
		#[pallet::call_index(4)]
		pub fn terminate_delegate_contract(
			origin: OriginFor<T>,
			contract_id: ContractId,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			Self::do_terminate_delegate_contract(signer, contract_id)?;
			Ok(())
		}

		/// Terminate a purchase contract
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `contract_id`: The id of the contract.
		#[pallet::call_index(5)]
		pub fn terminate_purchase_contract(
			origin: OriginFor<T>,
			contract_id: ContractId,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			Self::do_terminate_purchase_contract(signer, contract_id)?;
			Ok(())
		}

		/// Execute a data trade
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `contract_id`: The id of the contract.
		/// - `data_owner`: The owner of the data.
		/// - `data_issuer`: The issuer of the data.
		/// - `data_owner_fee_ratio`: The fee ratio of the data owner.
		/// - `data_issuer_fee_ratio`: The fee ratio of the data issuer.
		/// - `agency`: The agency of the data.
		/// - `agency_fee_ratio`: The fee ratio of the agency.
		/// - `price_per_data`: The price per data.
		/// - `data_verification_proof`: The verification proof of the data.
		#[pallet::call_index(6)]
		pub fn execute_data_trade(
			origin: OriginFor<T>,
			contract_id: ContractId,
			data_owner: T::AccountId,
			data_issuer: Vec<(T::AccountId, IssuerWeight)>,
			data_owner_fee_ratio: u32,
			data_issuer_fee_ratio: u32,
			agency: Option<T::AccountId>,
			agency_fee_ratio: Option<u32>,
			data_verification_proof: VerificationProof<AnyText>,
		) -> DispatchResult {
			let maybe_verifier = ensure_signed(origin)?;
			Self::do_execute_data_trade(
				maybe_verifier,
				contract_id,
				data_owner,
				data_issuer,
				data_owner_fee_ratio,
				data_issuer_fee_ratio,
				agency,
				agency_fee_ratio,
				data_verification_proof,
			)?;
			Ok(())
		}

		/// Register an agency
		///
		/// The dispatch origin for this call must be _Signed_.
		#[pallet::call_index(7)]
		pub fn register_agency(origin: OriginFor<T>) -> DispatchResult {
			let agency = ensure_signed(origin)?;
			Agencies::<T>::try_mutate(|list| -> DispatchResult {
				if list.contains(&agency) {
					return Err(Error::<T>::InvalidAgency.into());
				}
				list.push(agency.clone());
				Ok(())
			})?;
			Self::deposit_event(Event::<T>::RegisterAgency { agency });
			Ok(())
		}

		/// Deregister an agency
		///
		/// The dispatch origin for this call must be _Signed_.
		#[pallet::call_index(8)]
		pub fn deregister_agency(origin: OriginFor<T>) -> DispatchResult {
			let agency = ensure_signed(origin)?;
			Agencies::<T>::try_mutate(|list| -> DispatchResult {
				if list.contains(&agency) {
					list.retain(|x| x != &agency);
				} else {
					return Err(Error::<T>::InvalidAgency.into());
				}
				Ok(())
			})?;
			Self::deposit_event(Event::<T>::DeregisterAgency { agency });
			Ok(())
		}

		/// Make a delegate contract by root
		///
		/// The dispatch origin for this call must be _Admin_.
		///
		/// - `agency`: The agency of the contract.
		/// - `params`: The detail of the contract.
		#[pallet::call_index(9)]
		pub fn make_delegate_contract_by_admin(
			origin: OriginFor<T>,
			agency: T::AccountId,
			params: DataDelegateContractParams<T::AccountId, BlockNumberFor<T>>,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_make_delegate_contract(agency, params)?;
			Ok(())
		}

		/// Sign a delegate contract by root
		///
		/// The dispatch origin for this call must be _Admin_.
		///
		/// - `contract_id`: The id of the contract.
		/// - `data_owner`: The owner of the data.
		#[pallet::call_index(10)]
		pub fn sign_delegate_contract_by_admin(
			origin: OriginFor<T>,
			data_owner: T::AccountId,
			contract_id: ContractId,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_sign_delegate_contract(data_owner, contract_id)?;
			Ok(())
		}

		/// Make a purchase contract by root
		///
		/// The dispatch origin for this call must be _Admin_.
		///
		/// - `data_buyer`: The buyer of the data.
		/// - `params`: The detail of the contract.
		/// - `is_agency_exist`: The flag of the agency.
		#[pallet::call_index(11)]
		pub fn make_purchase_contract_by_admin(
			origin: OriginFor<T>,
			data_buyer: T::AccountId,
			params: DataPurchaseContractParams<T::AccountId, BlockNumberFor<T>, AssetBalanceOf<T>>,
			is_agency_exist: bool,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_make_purchase_contract(data_buyer, params, is_agency_exist)?;
			Ok(())
		}

		/// Sign a purchase contract by root
		///
		/// The dispatch origin for this call must be _Admin_.
		///
		/// - `data_agency`: The agency of the contract.
		/// - `contract_id`: The id of the contract.
		/// - `data_verifier`: The verifier of the contract.
		#[pallet::call_index(12)]
		pub fn sign_purchase_contract_by_admin(
			origin: OriginFor<T>,
			agency: T::AccountId,
			contract_id: ContractId,
			data_verifier: T::AccountId,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_sign_purchase_contract(agency, contract_id, data_verifier)?;
			Ok(())
		}

		/// Terminate a delegate contract by root
		///
		/// The dispatch origin for this call must be _Admin_.
		///
		/// - `signer`: The signer of the contract.
		/// - `contract_id`: The id of the contract.
		#[pallet::call_index(13)]
		pub fn terminate_delegate_contract_by_admin(
			origin: OriginFor<T>,
			signer: T::AccountId,
			contract_id: ContractId,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_terminate_delegate_contract(signer, contract_id)?;
			Ok(())
		}

		/// Terminate a purchase contract by root
		///
		/// The dispatch origin for this call must be _Admin_.
		///
		/// - `signer`: The signer of the contract.
		/// - `contract_id`: The id of the contract.
		#[pallet::call_index(14)]
		pub fn terminate_purchase_contract_by_admin(
			origin: OriginFor<T>,
			signer: T::AccountId,
			contract_id: ContractId,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_terminate_purchase_contract(signer, contract_id)?;
			Ok(())
		}

		/// Register an agency by root
		///
		/// The dispatch origin for this call must be _Admin_.
		///
		/// - `agency`: The agency of the contract.
		#[pallet::call_index(15)]
		pub fn register_agency_by_admin(
			origin: OriginFor<T>,
			agency: T::AccountId,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Agencies::<T>::try_mutate(|list| -> DispatchResult {
				if list.contains(&agency) {
					return Err(Error::<T>::InvalidAgency.into());
				}
				list.push(agency.clone());
				Ok(())
			})?;
			Self::deposit_event(Event::<T>::RegisterAgency { agency });
			Ok(())
		}

		/// Deregister an agency by root
		///
		/// The dispatch origin for this call must be _Admin_.
		///
		/// - `agency`: The agency of the contract.
		#[pallet::call_index(16)]
		pub fn deregister_agency_by_admin(
			origin: OriginFor<T>,
			agency: T::AccountId,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Agencies::<T>::try_mutate(|list| -> DispatchResult {
				if list.contains(&agency) {
					list.retain(|x| x != &agency);
				} else {
					return Err(Error::<T>::InvalidAgency.into());
				}
				Ok(())
			})?;
			Self::deposit_event(Event::<T>::DeregisterAgency { agency });
			Ok(())
		}

		/// Set the platform configuration
		///
		/// The dispatch origin for this call must be _Admin_.
		///
		/// - `config`: The configuration of the platform.
		#[pallet::call_index(17)]
		pub fn set_platform_config(
			origin: OriginFor<T>,
			config: MarketConfiguration,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_set_platform_config(config)?;
			Ok(())
		}

		/// Deposit balance
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// - `contract_id`: The id of the contract.
		/// - `amount`: The amount of the deposit.
		#[pallet::weight(18)]
		pub fn deposit_balance(
			origin: OriginFor<T>,
			contract_id: ContractId,
			amount: AssetBalanceOf<T>,
		) -> DispatchResult {
			let maybe_buyer = ensure_signed(origin)?;
			Self::do_deposit_balance(maybe_buyer, contract_id, amount)?;
			Ok(())
		}

		/// Deposit balance by admin
		///
		/// The dispatch origin for this call must be _Admin_.
		///
		/// - `data_buyer`: The buyer of the data.
		/// - `contract_id`: The id of the contract.
		/// - `amount`: The amount of the deposit.
		#[pallet::weight(19)]
		pub fn deposit_balance_by_admin(
			origin: OriginFor<T>,
			data_buyer: T::AccountId,
			contract_id: ContractId,
			amount: AssetBalanceOf<T>,
		) -> DispatchResult {
			T::AdminOrigin::ensure_origin(origin)?;
			Self::do_deposit_balance(data_buyer, contract_id, amount)?;
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T>
where
	AssetBalanceOf<T>: From<u128> + Into<u128>,
	AssetIdOf<T>: From<u32>,
{
	pub fn get_escrow_account() -> T::AccountId {
		const ID: PalletId = PalletId(*b"marketid");
		AccountIdConversion::<T::AccountId>::into_account_truncating(&ID)
	}

	pub fn get_platform_account() -> T::AccountId {
		const ID: PalletId = PalletId(*b"platform");
		AccountIdConversion::<T::AccountId>::into_account_truncating(&ID)
	}

	pub fn check_agency_exist(agency: &T::AccountId) -> bool {
		Agencies::<T>::get().contains(agency)
	}

	pub fn do_deposit_balance(
		data_buyer: T::AccountId,
		contract_id: ContractId,
		amount: AssetBalanceOf<T>,
	) -> DispatchResult {
		let detail =
			DataPurchaseContracts::<T>::get(contract_id).ok_or(Error::<T>::ContractNotExist)?;

		ensure!(detail.data_buyer == data_buyer, Error::<T>::InvalidBuyer);

		let escrow_account = Self::get_escrow_account();

		Self::transfer_escrow(
			TransferFrom::Origin(data_buyer),
			escrow_account,
			detail.system_token_id,
			amount.into(),
		)?;

		Self::update_deposit_in_detail(contract_id, detail.deposit + amount);

		Self::deposit_event(Event::<T>::DepositBalance { contract_id, amount });

		Ok(())
	}

	pub fn update_deposit_in_detail(contract_id: ContractId, left_deposit: AssetBalanceOf<T>) {
		let mut detail = DataPurchaseContracts::<T>::get(contract_id).unwrap();
		detail.deposit = left_deposit;
		DataPurchaseContracts::<T>::insert(contract_id, detail);
	}

	pub fn settle_data_trade(
		data_owner: T::AccountId,
		data_owner_fee: u128,
		data_issuer: Vec<(T::AccountId, IssuerWeight)>,
		data_issuer_fee: u128,
		platform_fee: u128,
		maybe_agency: Option<T::AccountId>,
		agency_fee: u128,
		system_token_asset_id: u32,
	) -> DispatchResult {
		let platform_account = Self::get_platform_account();

		Self::transfer_escrow(
			TransferFrom::Escrow,
			data_owner,
			system_token_asset_id,
			data_owner_fee,
		)?;

		let total_weight: u32 = data_issuer.iter().map(|(_, weight)| weight).sum();
		ensure!(total_weight > 0u32, Error::<T>::IssuerWeightInvalid);

		for (issuer, weight) in data_issuer.iter() {
			let distributed_fee = data_issuer_fee
				.saturating_mul(*weight as u128)
				.saturating_div(total_weight as u128);
			Self::transfer_escrow(
				TransferFrom::Escrow,
				issuer.clone(),
				system_token_asset_id,
				distributed_fee,
			)?;
		}

		Self::transfer_escrow(
			TransferFrom::Escrow,
			platform_account,
			system_token_asset_id,
			platform_fee,
		)?;

		if let Some(agency) = maybe_agency {
			if agency_fee > 0 {
				Self::transfer_escrow(
					TransferFrom::Escrow,
					agency,
					system_token_asset_id,
					agency_fee,
				)?;
			}
		}

		Ok(())
	}

	/**
	 * Calculate the fee for the data trade
	 * @param price_per_data The price per data
	 * @param data_owner_fee_ratio The fee ratio of the data owner
	 * @param data_issuer_fee_ratio The fee ratio of the data issuer
	 * @param agency_fee_ratio The fee ratio of the agency
	 * @return The fee for the data trade
	 *
	 * The fee is calculated as follows:
	 * 1. The total fee ratio is 100%, from total_fee_ratio
	 * 2. The remainder after subtracting data_owner_fee_ratio, data_issuer_fee_ratio, and
	 *    agency_fee_ratio from TotalFeeRatio becomes platform_fee_ratio.
	 * 3. The total amount is calculated as price_per_data * quantity * ratio.
	 */
	fn calculate_data_fee(
		price_per_data: u128,
		data_owner_fee_ratio: u32,
		data_issuer_fee_ratio: u32,
		agency_fee_ratio: u32,
	) -> (u128, u128, u128, u128) {
		let market_config = PlatformConfig::<T>::get();
		let MarketConfiguration { total_fee_ratio, min_platform_fee_ratio: _ } = market_config;

		let platform_fee_ratio =
			total_fee_ratio - data_owner_fee_ratio - data_issuer_fee_ratio - agency_fee_ratio;
		let quantity = 1;
		let total_amount = price_per_data * quantity;

		let data_owner_fee = total_amount
			.saturating_mul(data_owner_fee_ratio as u128)
			.saturating_div(total_fee_ratio.into());
		let data_issuer_fee = total_amount
			.saturating_mul(data_issuer_fee_ratio as u128)
			.saturating_div(total_fee_ratio.into());
		let platform_fee = total_amount
			.saturating_mul(platform_fee_ratio as u128)
			.saturating_div(total_fee_ratio.into());
		let agency_fee = total_amount
			.saturating_mul(agency_fee_ratio as u128)
			.saturating_div(total_fee_ratio.into());

		(data_owner_fee, data_issuer_fee, platform_fee, agency_fee)
	}

	fn transfer_escrow(
		from: TransferFrom<T>,
		to: T::AccountId,
		system_token_asset_id: u32,
		amount: u128,
	) -> DispatchResult {
		let balance = AssetBalanceOf::<T>::from(amount);

		match from {
			TransferFrom::Origin(origin) => {
				let _ = T::Assets::transfer(
					system_token_asset_id.into(),
					&origin,
					&to,
					balance,
					Preservation::Protect,
				);
			},
			TransferFrom::Escrow => {
				let escrow = Self::get_escrow_account();
				let _ = T::Assets::transfer(
					system_token_asset_id.into(),
					&escrow,
					&to,
					balance,
					Preservation::Protect,
				);
			},
		}

		Ok(())
	}

	fn do_make_delegate_contract(
		agency: T::AccountId,
		params: DataDelegateContractParams<T::AccountId, BlockNumberFor<T>>,
	) -> DispatchResult {
		let DataDelegateContractParams {
			data_owner,
			data_owner_minimum_fee_ratio,
			deligated_data,
			duration,
		} = params;

		ensure!(Self::check_agency_exist(&agency), Error::<T>::InvalidAgency);

		let current_block_number = frame_system::Pallet::<T>::block_number();

		let mut signed_status: ContractSigner<T::AccountId> = BoundedBTreeMap::new();
		signed_status
			.try_insert(agency.clone(), SignStatus::Signed)
			.and_then(|_| signed_status.try_insert(data_owner.clone(), SignStatus::Unsigned))
			.map_err(|_| Error::<T>::ExceedContractSigner)?;

		let detail: DataDelegateContractDetail<T::AccountId, BlockNumberFor<T>> =
			DataDelegateContractDetail {
				data_owner: data_owner.clone(),
				agency: agency.clone(),
				data_owner_minimum_fee_ratio,
				deligated_data,
				effective_at: current_block_number,
				expired_at: current_block_number + duration,
				signed_status,
			};

		let contract_id =
			NextContractId::<T>::try_mutate(|c| -> Result<ContractId, DispatchError> {
				*c = c.checked_add(1).ok_or(Error::<T>::Overflow)?;
				Ok(*c - 1)
			})?;

		DataDelegateContracts::<T>::insert(contract_id, detail);

		DataDelegateContractList::<T>::mutate(&data_owner, |list| {
			list.push(contract_id);
		});

		if data_owner != agency {
			DataDelegateContractList::<T>::mutate(&agency, |list| {
				list.push(contract_id);
			});
		}

		Self::deposit_event(Event::<T>::MakeDataDelegateContract { contract_id, agency });

		Ok(())
	}

	fn do_sign_delegate_contract(
		maybe_owner: T::AccountId,
		contract_id: ContractId,
	) -> DispatchResult {
		let detail =
			DataDelegateContracts::<T>::get(contract_id).ok_or(Error::<T>::ContractNotExist)?;

		ensure!(maybe_owner == detail.data_owner, Error::<T>::InvalidOwner);

		let mut is_signed = false;
		DataDelegateContracts::<T>::try_mutate(contract_id, |detail| -> DispatchResult {
			let detail = detail.as_mut().ok_or(Error::<T>::ContractStatusNotExist)?;
			let signed = detail
				.signed_status
				.get_mut(&maybe_owner)
				.ok_or(Error::<T>::ContractSignerNotExist)?;
			*signed = SignStatus::Signed;
			is_signed = true;
			Ok(())
		})?;
		ensure!(is_signed, Error::<T>::NotSigned);

		Self::deposit_event(Event::<T>::SignDateDelegateContract {
			contract_id,
			data_owner: maybe_owner,
		});
		Ok(())
	}

	fn do_make_purchase_contract(
		data_buyer: T::AccountId,
		params: DataPurchaseContractParams<T::AccountId, BlockNumberFor<T>, AssetBalanceOf<T>>,
		is_agency_exist: bool,
	) -> DispatchResult {
		let DataPurchaseContractParams {
			data_verifier,
			data_purchase_info,
			system_token_id,
			agency,
			deposit,
			duration,
			price_per_data,
		} = params.clone();

		let current_block_number = frame_system::Pallet::<T>::block_number();

		let contract_id =
			NextContractId::<T>::try_mutate(|c| -> Result<ContractId, DispatchError> {
				*c = c.checked_add(1).ok_or(Error::<T>::Overflow)?;
				Ok(*c - 1)
			})?;

		let mut detail: DataPurchaseContractDetail<
			T::AccountId,
			BlockNumberFor<T>,
			AssetBalanceOf<T>,
		> = DataPurchaseContractDetail {
			data_buyer: data_buyer.clone(),
			data_verifier: data_verifier.clone(),
			effective_at: current_block_number,
			expired_at: current_block_number + duration,
			data_purchase_info,
			system_token_id,
			agency: None,
			price_per_data,
			deposit,
			trade_count: 0,
			signed_status: BoundedBTreeMap::new(),
			data_trade_record: vec![], // empty
		};

		let mut signed_status: ContractSigner<T::AccountId> = BoundedBTreeMap::new();
		signed_status
			.try_insert(data_buyer.clone(), SignStatus::Signed)
			.map_err(|_| Error::<T>::ExceedContractSigner)?;
		if is_agency_exist {
			let agency = agency.ok_or(Error::<T>::InvalidAgency)?;
			ensure!(data_verifier.is_none(), Error::<T>::InvalidVerifier);

			if data_buyer != agency {
				DataPurchaseContractList::<T>::mutate(&agency, |list| {
					list.push(contract_id);
				});
			}

			signed_status
				.try_insert(agency.clone(), SignStatus::Unsigned)
				.map_err(|_| Error::<T>::ExceedContractSigner)?;

			detail.agency = Some(agency.clone());
		} else {
			ensure!(agency.is_none(), Error::<T>::InvalidAgency);
			ensure!(data_verifier.is_some(), Error::<T>::InvalidVerifier);
		}
		detail.signed_status = signed_status;

		DataPurchaseContracts::<T>::insert(contract_id, detail);

		DataPurchaseContractList::<T>::mutate(&data_buyer, |list| {
			list.push(contract_id);
		});

		let escrow_account = Self::get_escrow_account();
		Self::transfer_escrow(
			TransferFrom::Origin(data_buyer.clone()),
			escrow_account,
			system_token_id,
			deposit.into(),
		)?;

		Self::deposit_event(Event::<T>::MakeDataPurchaseContract { contract_id, data_buyer });

		Ok(())
	}

	fn do_sign_purchase_contract(
		maybe_agency: T::AccountId,
		contract_id: ContractId,
		data_verifier: T::AccountId,
	) -> DispatchResult {
		let mut detail =
			DataPurchaseContracts::<T>::get(contract_id).ok_or(Error::<T>::ContractNotExist)?;
		let agency = detail.clone().agency.ok_or(Error::<T>::InvalidAgency)?;
		ensure!(maybe_agency == agency, Error::<T>::InvalidAgency);

		if detail.agency.is_some() {
			let mut is_signed = false;
			DataPurchaseContracts::<T>::try_mutate(contract_id, |detail| -> DispatchResult {
				let detail = detail.as_mut().ok_or(Error::<T>::ContractStatusNotExist)?;
				let signed = detail
					.signed_status
					.get_mut(&agency)
					.ok_or(Error::<T>::ContractSignerNotExist)?;
				*signed = SignStatus::Signed;
				is_signed = true;
				Ok(())
			})?;
			ensure!(is_signed, Error::<T>::NotSigned);

			detail.data_verifier = Some(data_verifier.clone());
			DataPurchaseContracts::<T>::insert(contract_id, detail);

			Self::deposit_event(Event::<T>::SignDataPurchaseContract {
				contract_id,
				agency: maybe_agency,
				data_verifier,
			});
		}

		Ok(())
	}

	fn do_terminate_delegate_contract(
		maybe_signer: T::AccountId,
		contract_id: ContractId,
	) -> DispatchResult {
		let mut detail =
			DataDelegateContracts::<T>::get(contract_id).ok_or(Error::<T>::ContractNotExist)?;

		let mut signed_status = detail.signed_status;
		let mut is_signed = false;
		signed_status.iter_mut().for_each(|(signer, status)| {
			if signer == &maybe_signer {
				*status = SignStatus::WantToTerminate;
				is_signed = true;
			}
		});
		ensure!(is_signed, Error::<T>::NotSigned);

		let current_block_number = <frame_system::Pallet<T>>::block_number();

		if signed_status.iter().all(|(_, signed)| *signed != SignStatus::Signed) ||
			current_block_number > detail.expired_at
		{
			DataDelegateContracts::<T>::remove(contract_id);
			Self::deposit_event(Event::<T>::ContractTerminated {
				contract_type: ContractType::Delegate,
				contract_id,
			});
		} else {
			// Storage update when pending terminate only
			detail.signed_status = signed_status;
			DataDelegateContracts::<T>::insert(contract_id, detail);
			Self::deposit_event(Event::<T>::PendingContractTerminate {
				contract_type: ContractType::Delegate,
				contract_id,
			});
		}

		Ok(())
	}

	fn do_terminate_purchase_contract(
		maybe_signer: T::AccountId,
		contract_id: ContractId,
	) -> DispatchResult {
		let mut detail =
			DataPurchaseContracts::<T>::get(contract_id).ok_or(Error::<T>::ContractNotExist)?;
		let mut signed_status = detail.signed_status;

		if detail.agency.is_some() {
			let mut is_signed = false;
			signed_status.iter_mut().for_each(|(signer, status)| {
				if signer == &maybe_signer {
					*status = SignStatus::WantToTerminate;
					is_signed = true;
				}
			});

			ensure!(is_signed, Error::<T>::NotSigned);
		}

		let current_block_number = <frame_system::Pallet<T>>::block_number();

		if signed_status.iter().all(|(_, signed)| *signed != SignStatus::Signed) ||
			current_block_number > detail.expired_at
		{
			DataPurchaseContracts::<T>::remove(contract_id);

			if detail.deposit > 0.into() {
				Self::transfer_escrow(
					TransferFrom::Escrow,
					detail.data_buyer,
					detail.system_token_id,
					detail.deposit.into(),
				)?;
			}

			Self::deposit_event(Event::<T>::ContractTerminated {
				contract_type: ContractType::Purchase,
				contract_id,
			});
		} else {
			// Storage update when pending terminate only
			detail.signed_status = signed_status;
			DataPurchaseContracts::<T>::insert(contract_id, detail);

			Self::deposit_event(Event::<T>::PendingContractTerminate {
				contract_type: ContractType::Purchase,
				contract_id,
			});
		}

		Ok(())
	}

	fn do_execute_data_trade(
		maybe_verifier: T::AccountId,
		contract_id: ContractId,
		data_owner: T::AccountId,
		data_issuer: Vec<(T::AccountId, IssuerWeight)>,
		data_owner_fee_ratio: u32,
		data_issuer_fee_ratio: u32,
		maybe_agency: Option<T::AccountId>,
		maybe_agency_fee_ratio: Option<u32>,
		data_verification_proof: VerificationProof<AnyText>,
	) -> DispatchResult {
		let mut detail =
			DataPurchaseContracts::<T>::get(contract_id).ok_or(Error::<T>::ContractNotExist)?;
		let price_per_data = detail.price_per_data;
		let status = detail.signed_status.clone();
		let current_block_number = <frame_system::Pallet<T>>::block_number();

		let data_verifier = detail.clone().data_verifier.ok_or(Error::<T>::InvalidVerifier)?;
		ensure!(maybe_verifier == data_verifier, Error::<T>::InvalidVerifier);
		ensure!(
			status.iter().all(|(_, signed)| *signed == SignStatus::Signed),
			Error::<T>::ContractNotActive
		);
		ensure!(detail.clone().expired_at > current_block_number, Error::<T>::ContractNotActive);
		ensure!(detail.clone().effective_at <= current_block_number, Error::<T>::ContractNotActive);
		if let Some(agency) = maybe_agency.clone() {
			let agency_from_detail = detail.clone().agency.ok_or(Error::<T>::InvalidAgency)?;
			ensure!(agency == agency_from_detail, Error::<T>::InvalidAgency);
		}

		ensure!(detail.trade_count < T::MaxPurchaseQuantity::get(), Error::<T>::TradeLimitReached);
		detail.trade_count += 1;

		ensure!(
			(detail.data_trade_record.len() as u128) < T::MaxPurchaseQuantity::get(),
			Error::<T>::TradeLimitReached
		);

		if detail.agency.is_none() {
			// If Agency doesnt exist, then the data owner only sell data one time
			ensure!(detail.data_trade_record.contains(&data_owner), Error::<T>::AlreadyPurchased);
		} else {
			// If Agency exist, then the data owner can sell data multiple times. but insert storage
			// only one time
			if !detail.data_trade_record.contains(&data_owner) {
				detail.data_trade_record.push(data_owner.clone());
			}
		}

		DataPurchaseContracts::<T>::insert(contract_id, detail.clone());

		let market_config = PlatformConfig::<T>::get();
		let MarketConfiguration { total_fee_ratio, min_platform_fee_ratio } = market_config;
		let agency_fee_ratio = maybe_agency_fee_ratio.unwrap_or(0);
		let sum_fee_ratio = agency_fee_ratio +
			data_issuer_fee_ratio +
			data_owner_fee_ratio +
			min_platform_fee_ratio;

		ensure!(sum_fee_ratio <= total_fee_ratio, Error::<T>::InvalidFeeRatio);

		let (data_owner_fee, data_issuer_fee, platform_fee, agency_fee) = Self::calculate_data_fee(
			price_per_data.into(),
			data_owner_fee_ratio,
			data_issuer_fee_ratio,
			agency_fee_ratio,
		);

		Self::settle_data_trade(
			data_owner.clone(),
			data_owner_fee,
			data_issuer.clone(),
			data_issuer_fee,
			platform_fee,
			maybe_agency,
			agency_fee,
			detail.system_token_id,
		)?;

		Self::update_deposit_in_detail(contract_id, detail.deposit - price_per_data);

		Self::deposit_event(Event::<T>::DataTradeExecuted {
			contract_id,
			data_owner,
			data_issuer,
			data_owner_fee,
			data_issuer_fee,
			platform_fee,
			data_verification_proof,
		});

		Ok(())
	}

	fn do_set_platform_config(config: MarketConfiguration) -> DispatchResult {
		ensure!(config.total_fee_ratio > 0, Error::<T>::InvalidConfig);
		ensure!(config.min_platform_fee_ratio > 0, Error::<T>::InvalidConfig);
		ensure!(config.min_platform_fee_ratio < config.total_fee_ratio, Error::<T>::InvalidConfig);

		Self::deposit_event(Event::<T>::SetPlatformConfig { config: config.clone() });
		PlatformConfig::<T>::put(config);
		Ok(())
	}
}

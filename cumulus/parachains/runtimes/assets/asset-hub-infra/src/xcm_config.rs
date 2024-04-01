// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

use super::{
	AccountId, AllPalletsWithSystem, Assets, Authorship, Balance, Balances, ForeignAssets,
	ForeignAssetsCall, ForeignAssetsInstance, InfraXcm, NativeAssetsInstance, ParachainInfo,
	ParachainSystem, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, WeightToFee, XcmpQueue,
};
use frame_support::{
	match_types, parameter_types,
	traits::{ConstU32, Contains, Everything, Nothing, PalletInfoAccess},
};
use infra_asset_common::{
	matching::{StartsWith, StartsWithExplicitGlobalConsensus},
	AssetFeeAsExistentialDepositMultiplier,
};

use pallet_xcm::XcmPassthrough;
use parachain_primitives::primitives::Sibling;
use sp_runtime::traits::ConvertInto;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowExplicitUnpaidExecutionFrom, AllowSubscriptionsFrom,
	AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, EnsureXcmOrigin, FungiblesAdapter,
	LocalMint, NoChecking, ParentAsSuperuser, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, WeightInfoBounds,
	WithUniqueTopic,
};
use xcm_executor::{traits::WithOriginFilter, XcmExecutor};

parameter_types! {
	pub const NativeLocation: MultiLocation = Here.into_location();
	pub const RelayNetwork: Option<NetworkId> = Some(NetworkId::InfraRelay);
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub UniversalLocation: InteriorMultiLocation =
		X2(GlobalConsensus(RelayNetwork::get().unwrap()), Parachain(ParachainInfo::parachain_id().into()));
	pub UniversalLocationNetworkId: NetworkId = UniversalLocation::get().global_consensus().unwrap();
	pub NativeAssetsPalletLocation: MultiLocation =
		PalletInstance(<Assets as PalletInfoAccess>::index() as u8).into();
	pub CheckingAccount: AccountId = InfraXcm::check_account();
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// `AssetId/Balancer` converter for `TrustBackedAssets``
pub type NativeAssetsConvertedConcreteId =
	infra_asset_common::NativeAssetsConvertedConcreteId<NativeAssetsPalletLocation, Balance>;

/// Means for transacting assets besides the native currency on this chain.
pub type NativeIssuedFungiblesTransactor = FungiblesAdapter<
	// Use this fungibles implementation:
	Assets,
	// Use this currency when it is a fungible asset matching the given location or name:
	NativeAssetsConvertedConcreteId,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We only want to allow teleports of known assets. We use non-zero issuance as an indication
	// that this asset is known.
	LocalMint<infra_asset_common::NonZeroIssuance<AccountId, Assets>>,
	// The account to use for tracking teleports.
	CheckingAccount,
>;

/// `AssetId`/`Balance` converter for `ForeignAssets`.
pub type ForeignAssetsConvertedConcreteId = infra_asset_common::ForeignAssetsConvertedConcreteId<
	(
		// Ignore `TrustBackedAssets` explicitly
		StartsWith<NativeAssetsPalletLocation>,
		// Ignore asset which starts explicitly with our `GlobalConsensus(NetworkId)`, means:
		// - foreign assets from our consensus should be: `Location {parents: 1, X*(Parachain(xyz),
		//   ..)}
		// - foreign assets outside our consensus with the same `GlobalConsensus(NetworkId)` wont
		//   be accepted here
		StartsWithExplicitGlobalConsensus<UniversalLocationNetworkId>,
	),
	Balance,
>;

/// Means for transacting foreign assets from different global consensus.
pub type ForeignFungiblesTransactor = FungiblesAdapter<
	// Use this fungibles implementation:
	ForeignAssets,
	// Use this currency when it is a fungible asset matching the given location or name:
	ForeignAssetsConvertedConcreteId,
	// Convert an XCM MultiLocation into a local account id:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We dont need to check teleports here.
	NoChecking,
	// The account to use for tracking teleports.
	CheckingAccount,
>;

/// Means for transacting assets on this chain.
pub type AssetTransactors = (NativeIssuedFungiblesTransactor, ForeignFungiblesTransactor);

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
	// recognised.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognised.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
	// transaction from the Root origin.
	ParentAsSuperuser<RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `RuntimeOrigin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
	pub XcmAssetFeesReceiver: Option<AccountId> = Authorship::author();
}

match_types! {
	pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
	pub type ParentOrParentsPlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { .. }) }
	};
	pub type ParentOrSiblings: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(_) }
	};
}

/// A call filter for the XCM Transact instruction. This is a temporary measure until we properly
/// account for proof size weights.
///
/// Calls that are allowed through this filter must:
/// 1. Have a fixed weight;
/// 2. Cannot lead to another call being made;
/// 3. Have a defined proof size weight, e.g. no unbounded vecs in call parameters.
pub struct SafeCallFilter;
impl Contains<RuntimeCall> for SafeCallFilter {
	fn contains(call: &RuntimeCall) -> bool {
		#[cfg(feature = "runtime-benchmarks")]
		{
			if matches!(call, RuntimeCall::System(frame_system::Call::remark_with_event { .. })) {
				return true
			}
		}

		match call {
			RuntimeCall::System(
				frame_system::Call::set_heap_pages { .. } |
				frame_system::Call::set_code { .. } |
				frame_system::Call::set_code_without_checks { .. } |
				frame_system::Call::kill_prefix { .. },
			) |
			RuntimeCall::ParachainSystem(..) |
			RuntimeCall::Timestamp(..) |
			RuntimeCall::Balances(..) |
			RuntimeCall::CollatorSelection(
				pallet_collator_selection::Call::set_desired_candidates { .. } |
				pallet_collator_selection::Call::set_candidacy_bond { .. } |
				pallet_collator_selection::Call::register_as_candidate { .. } |
				pallet_collator_selection::Call::leave_intent { .. },
			) |
			RuntimeCall::Session(pallet_session::Call::purge_keys { .. }) |
			RuntimeCall::XcmpQueue(..) |
			RuntimeCall::DmpQueue(..) |
			RuntimeCall::Utility(pallet_utility::Call::as_derivative { .. }) |
			RuntimeCall::ForeignAssets(
				ForeignAssetsCall::force_set_metadata { .. } |
				ForeignAssetsCall::create { .. } |
				ForeignAssetsCall::force_create { .. } |
				ForeignAssetsCall::start_destroy { .. } |
				ForeignAssetsCall::destroy_accounts { .. } |
				ForeignAssetsCall::destroy_approvals { .. } |
				ForeignAssetsCall::finish_destroy { .. } |
				ForeignAssetsCall::mint { .. } |
				ForeignAssetsCall::burn { .. } |
				ForeignAssetsCall::transfer { .. } |
				ForeignAssetsCall::transfer_keep_alive { .. } |
				ForeignAssetsCall::force_transfer { .. } |
				ForeignAssetsCall::force_transfer2 { .. } |
				ForeignAssetsCall::freeze { .. } |
				ForeignAssetsCall::thaw { .. } |
				ForeignAssetsCall::freeze_asset { .. } |
				ForeignAssetsCall::thaw_asset { .. } |
				ForeignAssetsCall::transfer_ownership { .. } |
				ForeignAssetsCall::set_team { .. } |
				ForeignAssetsCall::clear_metadata { .. } |
				ForeignAssetsCall::force_clear_metadata { .. } |
				ForeignAssetsCall::force_asset_status { .. } |
				ForeignAssetsCall::approve_transfer { .. } |
				ForeignAssetsCall::cancel_approval { .. } |
				ForeignAssetsCall::force_cancel_approval { .. } |
				ForeignAssetsCall::transfer_approved { .. } |
				ForeignAssetsCall::touch { .. } |
				ForeignAssetsCall::refund { .. },
			) |
			RuntimeCall::Uniques(
				pallet_uniques::Call::create { .. } |
				pallet_uniques::Call::force_create { .. } |
				pallet_uniques::Call::destroy { .. } |
				pallet_uniques::Call::mint { .. } |
				pallet_uniques::Call::burn { .. } |
				pallet_uniques::Call::transfer { .. } |
				pallet_uniques::Call::freeze { .. } |
				pallet_uniques::Call::thaw { .. } |
				pallet_uniques::Call::freeze_collection { .. } |
				pallet_uniques::Call::thaw_collection { .. } |
				pallet_uniques::Call::transfer_ownership { .. } |
				pallet_uniques::Call::set_team { .. } |
				pallet_uniques::Call::approve_transfer { .. } |
				pallet_uniques::Call::cancel_approval { .. } |
				pallet_uniques::Call::force_item_status { .. } |
				pallet_uniques::Call::set_attribute { .. } |
				pallet_uniques::Call::clear_attribute { .. } |
				pallet_uniques::Call::set_metadata { .. } |
				pallet_uniques::Call::clear_metadata { .. } |
				pallet_uniques::Call::set_collection_metadata { .. } |
				pallet_uniques::Call::clear_collection_metadata { .. } |
				pallet_uniques::Call::set_accept_ownership { .. } |
				pallet_uniques::Call::set_collection_max_supply { .. } |
				pallet_uniques::Call::set_price { .. } |
				pallet_uniques::Call::buy_item { .. },
			) |
			RuntimeCall::SystemTokenOracle(pallet_system_token_oracle::Call::request_fiat { .. }) |
			RuntimeCall::InfraParaCore(..) => true,
			_ => false,
		}
	}
}

pub type Barrier = (
	// Weight that is paid for may be consumed.
	TakeWeightCredit,
	AllowUnpaidExecutionFrom<ParentOrSiblings>,
	AllowTopLevelPaidExecutionFrom<Everything>,
	// Messages coming from system parachains need not pay for execution.
	AllowExplicitUnpaidExecutionFrom<Everything>,
	// Subscriptions for version tracking are OK.
	AllowSubscriptionsFrom<Everything>,
);

/// Multiplier used for dedicated `TakeFirstAssetTrader` with `Assets` instance.
pub type NativeAssetFeeAsEDMultiplierFeeCharger = AssetFeeAsExistentialDepositMultiplier<
	Runtime,
	WeightToFee,
	pallet_assets::BalanceToAssetBalance<Balances, Runtime, ConvertInto, NativeAssetsInstance>,
	NativeAssetsInstance,
>;

/// Multiplier used for dedicated `TakeFirstAssetTrader` with `ForeignAssets` instance.
pub type ForeignAssetFeeAsEDMultiplierFeeCharger = AssetFeeAsExistentialDepositMultiplier<
	Runtime,
	WeightToFee,
	pallet_assets::BalanceToAssetBalance<Balances, Runtime, ConvertInto, ForeignAssetsInstance>,
	ForeignAssetsInstance,
>;

parameter_types! {
	pub const ItestInfraSystemLocation: MultiLocation = X2(PalletInstance(50), GeneralIndex(99)).into_location();
	pub const ItestInfraSystemFilter: MultiAssetFilter = Wild(AllOf { fun: WildFungible, id: Concrete(ItestInfraSystemLocation::get()) });

	pub const TemplateParachain: MultiLocation = Parachain(2000).into_exterior(1);
	pub const InfraSystem: MultiLocation = Parachain(1000).into_exterior(1);

	pub const ItestForTemplateParachain: (MultiAssetFilter, MultiLocation) = (ItestInfraSystemFilter::get(), TemplateParachain::get());
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = AssetTransactors;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = ();
	type IsTeleporter = Everything;
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = WeightInfoBounds<
		crate::weights::xcm::StatemintXcmWeight<RuntimeCall>,
		RuntimeCall,
		MaxInstructions,
	>;
	type Trader = (
		// This trader allows to pay with `is_sufficient=true` "Trust Backed" assets from dedicated
		// `pallet_assets` instance - `Assets`.
		cumulus_primitives_utility::TakeFirstAssetTrader<
			AccountId,
			NativeAssetFeeAsEDMultiplierFeeCharger,
			NativeAssetsConvertedConcreteId,
			Assets,
			cumulus_primitives_utility::XcmFeesTo32ByteAccount<
				NativeIssuedFungiblesTransactor,
				AccountId,
				XcmAssetFeesReceiver,
			>,
		>,
		// This trader allows to pay with `is_sufficient=true` "Foreign" assets from dedicated
		// `pallet_assets` instance - `ForeignAssets`.
		cumulus_primitives_utility::TakeFirstAssetTrader<
			AccountId,
			ForeignAssetFeeAsEDMultiplierFeeCharger,
			ForeignAssetsConvertedConcreteId,
			ForeignAssets,
			cumulus_primitives_utility::XcmFeesTo32ByteAccount<
				ForeignFungiblesTransactor,
				AccountId,
				XcmAssetFeesReceiver,
			>,
		>,
	);
	type ResponseHandler = InfraXcm;
	type AssetTrap = InfraXcm;
	type AssetClaims = InfraXcm;
	type SubscriptionService = InfraXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type AssetLocker = ();
	type AssetExchanger = ();
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = WithOriginFilter<SafeCallFilter>;
	type SafeCallFilter = SafeCallFilter;
	type Aliasers = Nothing;
}

/// Converts a local signed origin into an XCM multilocation.
/// Forms the basis for local origins sending/executing XCMs.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = WithUniqueTopic<(
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, InfraXcm, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
)>;

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	// We want to disallow users sending (arbitrary) XCMs from this chain.
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, ()>;
	type XcmRouter = XcmRouter;
	// We support local origins dispatching XCM executions in principle...
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	// ... but disallow generic XCM execution. As a result only teleports and reserve transfers are
	// allowed.
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type Weigher = WeightInfoBounds<
		crate::weights::xcm::StatemintXcmWeight<RuntimeCall>,
		RuntimeCall,
		MaxInstructions,
	>;
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = LocationToAccountId;
	type MaxLockers = ConstU32<8>;
	type WeightInfo = crate::weights::pallet_xcm::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type ReachableDest = ReachableDest;
	type AdminOrigin = frame_system::EnsureRoot<AccountId>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

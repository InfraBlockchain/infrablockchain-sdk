#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod weights;
pub mod xcm_config;
use cumulus_pallet_parachain_system::RelayNumberMonotonicallyIncreases;
use cumulus_primitives_core::{AggregateMessageOrigin, ParaId};
use parachains_common::{
    impls::DealWithFees,
    message_queue::{NarrowOriginToSibling, ParaIdToSibling},
    AccountId, AssetIdForTrustBackedAssets, AuraId, Balance, BlockNumber, CollectionId, Hash,
    Header, ItemId, Nonce, Signature, AVERAGE_ON_INITIALIZE_RATIO, NORMAL_DISPATCH_RATIO,
};
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, Get, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto,
        TryConvertInto as JustTry,
    },
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult,
};
use xcm_config::{NativeAssetsPalletLocation, UniversalLocation, XcmRouter};

use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
    construct_runtime, derive_impl,
    dispatch::DispatchClass,
    genesis_builder_helper::{build_state, get_preset},
    parameter_types,
    traits::{
        tokens::fungibles::{Balanced, Credit, UnionOf},
        AsEnsureOriginWithArg, ConstBool, ConstU32, ConstU64, ConstU8, EitherOfDiverse,
        InstanceFilter, TransformOrigin,
    },
    weights::{ConstantMultiplier, Weight},
    PalletId,
};
use frame_system::{
    limits::{BlockLength, BlockWeights},
    EnsureRoot, EnsureSigned,
};
use infra_asset_common::{
    local_and_foreign_assets::LocalFromLeft, AssetIdForNativeAssets, AssetIdForNativeAssetsConvert,
};
use pallet_system_token_tx_payment::{HandleCredit, RewardOriginInfo, TransactionFeeCharger};
pub use sp_runtime::{infra::*, MultiAddress, Perbill, Permill};
use xcm_config::{XcmConfig, XcmOriginToTransactDispatchOrigin};

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

// TODO: These constants are para-agnostic, but we need to configure for DID-chain specific constants
use testnet_parachains_constants::yosemite::{
    consensus::*, currency::*, fee::WeightToFee, time::*,
};

// Polkadot imports
use polkadot_runtime_common::{prod_or_fast, BlockHashCount, SlowAdjustingFeeUpdate};
use weights::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight};

pub use pallet_did::{
    accumulator, anchor, attest, blob, common, did,
    offchain_signatures::{self, BBSPlusPublicKey, OffchainPublicKey, PSPublicKey},
    revoke, status_list_credential, trusted_entity,
};

// XCM Imports
use pallet_xcm::{EnsureXcm, IsMajorityOfBody};
use xcm::latest::{AssetId, BodyId, Location, Reanchorable};
use xcm_executor::XcmExecutor;

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_system_token_tx_payment::ChargeSystemToken<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;
pub type Migrate = ();

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrate,
>;

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
    }
}

// #[cfg(not(feature = "std"))]
// mod wasm_handlers {
//     #[panic_handler]
//     #[no_mangle]
//     pub fn panic(info: &core::panic::PanicInfo) -> ! {
//         let message = sp_std::alloc::format!("{}", info);
//         log::error!("{}", message);
//         ::core::arch::wasm32::unreachable();
//     }

//     #[cfg(enable_alloc_error_handler)]
//     #[alloc_error_handler]
//     pub fn oom(_: core::alloc::Layout) -> ! {
//         log::error!("Runtime memory exhausted. Aborting");
//         ::core::arch::wasm32::unreachable();
//     }
// }

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("infra-did-yosemite"),
    impl_name: create_runtime_str!("bcl-infra-did-yosemite"),
    authoring_version: 1,
    spec_version: 10_000,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const DepositToCreateAsset: Balance = 1 * UNITS; // 1 UNITS deposit to create fungible asset class
    pub const DepositToMaintainAsset: Balance = deposit(1, 16);
    pub const ApprovalDeposit: Balance = EXISTENTIAL_DEPOSIT;
    pub const StringLimit: u32 = 50;
    /// Key = 32 bytes, Value = 36 bytes (32+1+1+1+1)
    // https://github.com/paritytech/substrate/blob/069917b/frame/assets/src/lib.rs#L257L271
    pub const MetadataDepositBase: Balance = deposit(1, 68);
    pub const MetadataDepositPerByte: Balance = deposit(0, 1);
    pub const ExecutiveBody: BodyId = BodyId::Executive;
}

/// We allow root and the Relay Chain council to execute privileged asset operations.
pub type RootOrigin = EnsureRoot<AccountId>;

pub type NativeAssetsInstance = pallet_assets::Instance1;
type NativeAssetsCall = pallet_assets::Call<Runtime, NativeAssetsInstance>;
impl pallet_assets::Config<NativeAssetsInstance> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type AssetId = AssetIdForNativeAssets;
    type AssetIdParameter = codec::Compact<AssetIdForNativeAssets>;
    type SystemTokenWeight = SystemTokenWeight;
    type Currency = Balances;
    type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
    type ForceOrigin = RootOrigin;
    type AssetDeposit = DepositToCreateAsset;
    type AssetAccountDeposit = DepositToMaintainAsset;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type CallbackHandle = ();
    type WeightInfo = ();
    type RemoveItemsLimit = ConstU32<1000>;
}

pub type ForeignAssetsInstance = pallet_assets::Instance2;
type ForeignAssetsCall = pallet_assets::Call<Runtime, ForeignAssetsInstance>;
impl pallet_assets::Config<ForeignAssetsInstance> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type AssetId = Location;
    type AssetIdParameter = Location;
    type SystemTokenWeight = SystemTokenWeight;
    type Currency = Balances;
    type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
    type ForceOrigin = RootOrigin; //TODO
    type AssetDeposit = DepositToCreateAsset;
    type AssetAccountDeposit = DepositToMaintainAsset;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type CallbackHandle = ();
    type WeightInfo = ();
    type RemoveItemsLimit = ConstU32<1000>;
}

pub struct ReanchorHandler;
impl ReanchorSystemToken<Location> for ReanchorHandler {
    type Error = ();
    fn reanchor_system_token(l: &mut Location) -> Result<(), Self::Error> {
        let target = Location::parent();
        let context = UniversalLocation::get();
        l.reanchor(&target, &context).map_err(|_| {})?;
        Ok(())
    }
}

/// Union fungibles implementation for `Assets` and `ForeignAssets`.
pub type NativeAndForeignAssets = UnionOf<
    Assets,
    ForeignAssets,
    LocalFromLeft<
        AssetIdForNativeAssetsConvert<NativeAssetsPalletLocation>,
        AssetIdForNativeAssets,
        Location,
    >,
    Location,
    AccountId,
    ReanchorHandler,
>;

parameter_types! {
    pub const FeeTreasuryId: PalletId = PalletId(*b"infrapid");
}

pub struct BootstrapCallFilter;
impl frame_support::traits::Contains<RuntimeCall> for BootstrapCallFilter {
    #[cfg(not(feature = "fast-runtime"))]
    fn contains(call: &RuntimeCall) -> bool {
        match call {
            RuntimeCall::Assets(
                pallet_assets::Call::create { .. }
                | pallet_assets::Call::set_metadata { .. }
                | pallet_assets::Call::mint { .. },
            )
            | RuntimeCall::InfraXcm(pallet_xcm::Call::limited_teleport_assets { .. })
            | RuntimeCall::InfraParaCore(
                cumulus_pallet_infra_parachain_core::Call::request_register_system_token { .. },
            ) => true,
            _ => false,
        }
    }
    #[cfg(feature = "fast-runtime")]
    fn contains(call: &RuntimeCall) -> bool {
        match call {
            _ => true,
        }
    }
}

parameter_types! {
    pub const RewardFraction: Perbill = Perbill::from_percent(80);
}

impl pallet_system_token_tx_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type SystemConfig = InfraParaCore;
    type PoTHandler = ParachainSystem;
    type Fungibles = NativeAndForeignAssets;
    type RewardFraction = RewardFraction;
    type RewardOrigin = ValidatorRewardOrigin;
    type OnChargeSystemToken = TransactionFeeCharger<Runtime, SystemTokenConversion, CreditHandler>;
    type BootstrapCallFilter = BootstrapCallFilter;
    type PalletId = FeeTreasuryId;
}

pub struct ValidatorRewardOrigin;
impl RewardOriginInfo for ValidatorRewardOrigin {
    type Origin = u32;
    fn reward_origin_info() -> voting::RewardOrigin<Self::Origin> {
        let para_id =
            <parachain_info::Pallet<Runtime> as Get<cumulus_primitives_core::ParaId>>::get().into();
        voting::RewardOrigin::Remote(para_id)
    }
}

pub struct CreditHandler;
impl HandleCredit<AccountId, NativeAndForeignAssets> for CreditHandler {
    fn handle_credit(credit: Credit<AccountId, NativeAndForeignAssets>) {
        if let Some(author) = pallet_authorship::Pallet::<Runtime>::author() {
            let _ = <NativeAndForeignAssets as Balanced<AccountId>>::resolve(&author, credit);
        }
    }
}

impl pallet_system_token_conversion::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = SystemTokenBalance;
    type AssetKind = Location;
    type Fungibles = NativeAndForeignAssets;
    type SystemConfig = InfraParaCore;
}

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;

    // This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
    //  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
    // `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
    // the lazy contract deletion.
    pub RuntimeBlockLength: BlockLength =
        BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
        .base_block(BlockExecutionWeight::get())
        .for_class(DispatchClass::all(), |weights| {
            weights.base_extrinsic = ExtrinsicBaseWeight::get();
        })
        .for_class(DispatchClass::Normal, |weights| {
            weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
        })
        .for_class(DispatchClass::Operational, |weights| {
            weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
            // Operational transactions have some extra reserved space, so that they
            // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
            weights.reserved = Some(
                MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
            );
        })
        .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
        .build_or_panic();
    pub const SS58Prefix: u16 = 42;
}

// Configure FRAME pallets to include in runtime.
#[derive_impl(frame_system::config_preludes::ParaChainDefaultConfig)]
impl frame_system::Config for Runtime {
    type BlockWeights = RuntimeBlockWeights;
    type BlockLength = RuntimeBlockLength;
    type AccountId = AccountId;
    type Nonce = Nonce;
    type Hash = Hash;
    type Block = Block;
    type BlockHashCount = BlockHashCount;
    type DbWeight = RocksDbWeight;
    type Version = Version;
    type AccountData = pallet_balances::AccountData<Balance>;
    type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
    type SS58Prefix = SS58Prefix;
    type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = ();
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
}

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
    type EventHandler = (CollatorSelection,);
}

parameter_types! {
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type FreezeIdentifier = ();
    type MaxFreezes = ConstU32<0>;
}

parameter_types! {
    /// Relay Chain `TransactionByteFee` / 10
    pub const TransactionByteFee: Balance = 1 * 100_000;
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction =
        pallet_transaction_payment::FungibleAdapter<Balances, DealWithFees<Runtime>>;
    type WeightToFee = WeightToFee;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = ();
    type OperationalFeeMultiplier = ConstU8<5>;
}

parameter_types! {
    pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
    pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

type ConsensusHook = cumulus_pallet_aura_ext::FixedVelocityConsensusHook<
    Runtime,
    RELAY_CHAIN_SLOT_DURATION_MILLIS,
    BLOCK_PROCESSING_VELOCITY,
    UNINCLUDED_SEGMENT_CAPACITY,
>;

parameter_types! {
    pub MessageQueueServiceWeight: Weight = Perbill::from_percent(35) * RuntimeBlockWeights::get().max_block;
}

impl pallet_message_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_message_queue::WeightInfo<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<
        cumulus_primitives_core::AggregateMessageOrigin,
    >;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MessageProcessor = xcm_builder::ProcessXcmMessage<
        AggregateMessageOrigin,
        xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
        RuntimeCall,
    >;
    type Size = u32;
    // The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
    type QueueChangeHandler = NarrowOriginToSibling<XcmpQueue>;
    type QueuePausedQuery = NarrowOriginToSibling<XcmpQueue>;
    type HeapSize = sp_core::ConstU32<{ 64 * 1024 }>;
    type MaxStale = sp_core::ConstU32<8>;
    type ServiceWeight = MessageQueueServiceWeight;
    type IdleMaxServiceWeight = MessageQueueServiceWeight;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
    type WeightInfo = weights::cumulus_pallet_parachain_system::WeightInfo<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type OnSystemEvent = ();
    type SelfParaId = parachain_info::Pallet<Runtime>;
    type DmpQueue = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
    type ReservedDmpWeight = ReservedDmpWeight;
    type OutboundXcmpMessageSource = XcmpQueue;
    type XcmpMessageHandler = XcmpQueue;
    type ReservedXcmpWeight = ReservedXcmpWeight;
    type UpdateRCConfig = InfraParaCore;
    type CheckAssociatedRelayNumber = RelayNumberMonotonicallyIncreases;
    type ConsensusHook = ConsensusHook;
}

parameter_types! {
    pub const RelayOrigin: AggregateMessageOrigin = AggregateMessageOrigin::Parent;
}

parameter_types! {
    pub const ActiveRequestPeriod: u32 = DAYS;
}

impl cumulus_pallet_infra_parachain_core::Config for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeEvent = RuntimeEvent;
    type SystemTokenId = Location;
    type UniversalLocation = UniversalLocation;
    type Fungibles = NativeAndForeignAssets;
    type ActiveRequestPeriod = ActiveRequestPeriod;
    type FeeTreasuryId = FeeTreasuryId;
}

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

pub type PriceForSiblingParachainDelivery =
    polkadot_runtime_common::xcm_sender::NoPriceForMessageDelivery<cumulus_primitives_core::ParaId>;

impl cumulus_pallet_xcmp_queue::Config for Runtime {
    type WeightInfo = weights::cumulus_pallet_xcmp_queue::WeightInfo<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type ChannelInfo = ParachainSystem;
    type VersionWrapper = InfraXcm;
    type XcmpQueue = TransformOrigin<MessageQueue, AggregateMessageOrigin, ParaId, ParaIdToSibling>;
    type MaxInboundSuspended = ConstU32<1_000>;
    type MaxActiveOutboundChannels = ConstU32<128>;
    // Most on-chain HRMP channels are configured to use 102400 bytes of max message size, so we
    // need to set the page size larger than that until we reduce the channel size on-chain.
    type MaxPageSize = ConstU32<{ 103 * 1024 }>;
    type ControllerOrigin = EnsureRoot<AccountId>;
    type ControllerOriginConverter = xcm_config::XcmOriginToTransactDispatchOrigin;
    type PriceForSiblingDelivery = PriceForSiblingParachainDelivery;
}

parameter_types! {
    pub const Period: u32 = 6 * HOURS;
    pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    // we don't have stash and controller, thus we don't need the convert as well.
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = CollatorSelection;
    // Essentially just Aura, but let's be pedantic.
    type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<100_000>;
    type AllowMultipleBlocksPerSlot = ConstBool<true>;
    type SlotDuration = ConstU64<SLOT_DURATION>;
}

parameter_types! {
    pub const PotId: PalletId = PalletId(*b"PotStake");
    pub const MaxCandidates: u32 = 1000;
    pub const MinCandidates: u32 = 5;
    pub const SessionLength: BlockNumber = 6 * HOURS;
    pub const MaxInvulnerables: u32 = 100;
}

// We allow root only to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EnsureRoot<AccountId>;

impl pallet_collator_selection::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type UpdateOrigin = CollatorSelectionUpdateOrigin;
    type PotId = PotId;
    type MaxCandidates = ConstU32<100>;
    type MinEligibleCollators = ConstU32<4>;
    type MaxInvulnerables = ConstU32<20>;
    // should be a multiple of session or things will get inconsistent
    type KickThreshold = Period;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
    type ValidatorRegistration = Session;
    type WeightInfo = weights::pallet_collator_selection::WeightInfo<Runtime>;
}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
        RuntimeBlockWeights::get().max_block;
}

impl pallet_scheduler::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    #[cfg(feature = "runtime-benchmarks")]
    type MaxScheduledPerBlock = ConstU32<512>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MaxScheduledPerBlock = ConstU32<50>;
    type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
    type OriginPrivilegeCmp = frame_support::traits::EqualPrivilegeOnly;
    type Preimages = Preimage;
}

parameter_types! {
    pub const PreimageBaseDeposit: Balance = CENTS;
    pub const PreimageByteDeposit: Balance = MILLICENTS;
    pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
}

impl pallet_preimage::Config for Runtime {
    type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type Consideration = frame_support::traits::fungible::HoldConsideration<
        AccountId,
        Balances,
        PreimageHoldReason,
        frame_support::traits::LinearStoragePrice<
            PreimageBaseDeposit,
            PreimageByteDeposit,
            Balance,
        >,
    >;
}

parameter_types! {
    /// 8KB
    pub const MaxBlobSize: u32 = 8192;
    /// 1KB
    pub const MaxIriSize: u32 = 1024;

    /// 128 bytes, for large labels, hash of a label can be used
    pub const MaxAccumulatorLabelSize: u32 = 128;

    pub const MaxAccumulatorParamsSize: u32 = 512;

    /// 128 bytes, for large labels, hash of a label can be used
    pub const MaxOffchainParamsLabelSize: u32 = 128;
    /// 16KB
    pub const MaxOffchainParamsBytesSize: u32 = 65536;

    pub const FixedPublicKeyMaxSize: u32 = 256;
    pub const PSPublicKeyMaxSize: u32 = 65536;

    pub const AccumulatedMaxSize: u32 = 128;

    pub const MaxDidDocRefSize: u16 = 1024;
    pub const MaxDidServiceEndpointIdSize: u16 = 1024;
    pub const MaxDidServiceEndpointOrigins: u16 = 64;
    pub const MaxDidServiceEndpointOriginSize: u16 = 1025;

    pub const MaxPolicyControllers: u32 = 15;
    pub const MinStatusListCredentialSize: u32 = 500;
    pub const MaxStatusListCredentialSize: u32 = 40_000;

    pub const MaxMasterMembers: u32 = 25;
}

impl did::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnDidRemoval = OffchainSignatures;
}

impl revoke::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl trusted_entity::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl common::Limits for Runtime {
    type MaxPolicyControllers = MaxPolicyControllers;

    type MaxDidDocRefSize = MaxDidDocRefSize;
    type MaxDidServiceEndpointIdSize = MaxDidServiceEndpointIdSize;
    type MaxDidServiceEndpointOriginSize = MaxDidServiceEndpointOriginSize;
    type MaxDidServiceEndpointOrigins = MaxDidServiceEndpointOrigins;

    type MinStatusListCredentialSize = MinStatusListCredentialSize;
    type MaxStatusListCredentialSize = MaxStatusListCredentialSize;

    type MaxPSPublicKeySize = PSPublicKeyMaxSize;
    type MaxBBSPublicKeySize = FixedPublicKeyMaxSize;
    type MaxBBSPlusPublicKeySize = FixedPublicKeyMaxSize;

    type MaxOffchainParamsLabelSize = MaxOffchainParamsLabelSize;
    type MaxOffchainParamsBytesSize = MaxOffchainParamsBytesSize;

    type MaxAccumulatorLabelSize = MaxAccumulatorLabelSize;
    type MaxAccumulatorParamsSize = MaxAccumulatorParamsSize;
    type MaxAccumulatorPublicKeySize = FixedPublicKeyMaxSize;
    type MaxAccumulatorAccumulatedSize = AccumulatedMaxSize;

    type MaxBlobSize = MaxBlobSize;
    type MaxIriSize = MaxIriSize;

    type MaxMasterMembers = MaxMasterMembers;
}

impl status_list_credential::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl offchain_signatures::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl accumulator::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl blob::Config for Runtime {}

impl anchor::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl attest::Config for Runtime {}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime {
        // System support stuff.
        System: frame_system = 0,
        ParachainSystem: cumulus_pallet_parachain_system = 1,
        InfraParaCore: cumulus_pallet_infra_parachain_core = 2,
        Timestamp: pallet_timestamp = 3,
        ParachainInfo: parachain_info = 4,

        // Monetary stuff.
        Balances: pallet_balances = 10,
        TransactionPayment: pallet_transaction_payment = 11,
        SystemTokenTxPayment: pallet_system_token_tx_payment = 12,
        SystemTokenConversion: pallet_system_token_conversion = 13,

        // Collator support. the order of these 5 are important and shall not change.
        Authorship: pallet_authorship = 20,
        CollatorSelection: pallet_collator_selection = 21,
        Session: pallet_session = 22,
        Aura: pallet_aura = 23,
        AuraExt: cumulus_pallet_aura_ext = 24,

        // XCM helpers.
        XcmpQueue: cumulus_pallet_xcmp_queue = 30,
        InfraXcm: pallet_xcm = 31,
        CumulusXcm: cumulus_pallet_xcm = 32,
        MessageQueue: pallet_message_queue = 33,

        // Governance
        Preimage: pallet_preimage = 40,
        Scheduler: pallet_scheduler = 41,

        // Assets
        Assets: pallet_assets::<Instance1> = 50,
        ForeignAssets: pallet_assets::<Instance2> = 51,

        // Main Stage - DID
        DIDModule: did = 61,
        Revoke: revoke = 62,
        BlobStore: blob = 63,
        Anchor: anchor = 64,
        Attest: attest = 65,
        Accumulator: accumulator = 66,
        OffchainSignatures: offchain_signatures = 67,
        StatusListCredential: status_list_credential = 68,
        TrustedEntity: trusted_entity = 69,

        Sudo: pallet_sudo = 99,
    }
);

#[cfg(not(feature = "std"))]
mod wasm_handlers {
    #[panic_handler]
    #[no_mangle]
    pub fn panic(info: &core::panic::PanicInfo) -> ! {
        let message = sp_std::alloc::format!("{}", info);
        log::error!("{}", message);
        ::core::arch::wasm32::unreachable();
    }

    #[cfg(enable_alloc_error_handler)]
    #[alloc_error_handler]
    pub fn oom(_: core::alloc::Layout) -> ! {
        log::error!("Runtime memory exhausted. Aborting");
        ::core::arch::wasm32::unreachable();
    }
}

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    frame_benchmarking::define_benchmarks!(
        [frame_system, SystemBench::<Runtime>]
        [pallet_balances, Balances]
        [pallet_session, SessionBench::<Runtime>]
        [pallet_timestamp, Timestamp]
        [pallet_collator_selection, CollatorSelection]
        [cumulus_pallet_xcmp_queue, XcmpQueue]
    );
}

impl_runtime_apis! {
    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            pallet_aura::Authorities::<Runtime>::get().into_inner()
        }
    }

    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
        for Runtime
    {
        fn query_call_info(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_call_info(call, len)
        }
        fn query_call_fee_details(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_call_fee_details(call, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
        fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
            ParachainSystem::collect_collation_info(header)
        }
    }

    impl cumulus_primitives_aura::AuraUnincludedSegmentApi<Block> for Runtime {
        fn can_build_upon(
            included_hash: <Block as BlockT>::Hash,
            slot: cumulus_primitives_aura::Slot,
        ) -> bool {
            ConsensusHook::can_build_upon(included_hash, slot)
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, RuntimeBlockWeights::get().max_block)
        }

        fn execute_block(
            block: Block,
            state_root_check: bool,
            signature_check: bool,
            select: frame_try_runtime::TryStateSelect,
        ) -> Weight {
            // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
            // have a backtrace here.
            Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {

            use frame_benchmarking::{Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;
            use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

            // This is defined once again in dispatch_benchmark, because list_benchmarks!
            // and add_benchmarks! are macros exported by define_benchmarks! macros and those types
            // are referenced in that call.
            type XcmBalances = pallet_xcm_benchmarks::fungible::Pallet::<Runtime>;
            type XcmGeneric = pallet_xcm_benchmarks::generic::Pallet::<Runtime>;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();
            return (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey, BenchmarkError};

            use frame_system_benchmarking::Pallet as SystemBench;
            impl frame_system_benchmarking::Config for Runtime {}

            use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
            impl cumulus_pallet_session_benchmarking::Config for Runtime {}

            use xcm::latest::prelude::*;
            use xcm_config::{NativeLocation, MaxAssetsIntoHolding};
            use pallet_xcm_benchmarks::asset_instance_from;

            impl pallet_xcm_benchmarks::Config for Runtime {
                type XcmConfig = xcm_config::XcmConfig;
                type AccountIdConverter = xcm_config::LocationToAccountId;
                fn valid_destination() -> Result<Location, BenchmarkError> {
                    Ok(NativeLocation::get())
                }
                fn worst_case_holding(depositable_count: u32) -> MultiAssets {
                    // A mix of fungible, non-fungible, and concrete assets.
                    let holding_non_fungibles = MaxAssetsIntoHolding::get() / 2 - depositable_count;
                    let holding_fungibles = holding_non_fungibles - 1;
                    let fungibles_amount: u128 = 100;
                    let mut assets = (0..holding_fungibles)
                        .map(|i| {
                            MultiAsset {
                                id: Concrete(GeneralIndex(i as u128).into()),
                                fun: Fungible(fungibles_amount * i as u128),
                            }
                            .into()
                        })
                        .chain(core::iter::once(MultiAsset { id: Concrete(Here.into()), fun: Fungible(u128::MAX) }))
                        .chain((0..holding_non_fungibles).map(|i| MultiAsset {
                            id: Concrete(GeneralIndex(i as u128).into()),
                            fun: NonFungible(asset_instance_from(i)),
                        }))
                        .collect::<Vec<_>>();

                    assets.push(MultiAsset {
                        id: Concrete(NativeLocation::get()),
                        fun: Fungible(1_000_000 * UNITS),
                    });
                    assets.into()
                }
            }

            parameter_types! {
                pub const TrustedTeleporter: Option<(Location, MultiAsset)> = Some((
                    NativeLocation::get(),
                    MultiAsset { fun: Fungible(1 * UNITS), id: Concrete(NativeLocation::get()) },
                ));
                pub const CheckedAccount: Option<(AccountId, xcm_builder::MintLocation)> = None;
            }

            impl pallet_xcm_benchmarks::fungible::Config for Runtime {
                type TransactAsset = Balances;

                type CheckedAccount = CheckedAccount;
                type TrustedTeleporter = TrustedTeleporter;

                fn get_multi_asset() -> MultiAsset {
                    MultiAsset {
                        id: Concrete(NativeLocation::get()),
                        fun: Fungible(1 * UNITS),
                    }
                }
            }

            impl pallet_xcm_benchmarks::generic::Config for Runtime {
                type RuntimeCall = RuntimeCall;

                fn worst_case_response() -> (u64, Response) {
                    (0u64, Response::Version(Default::default()))
                }

                fn worst_case_asset_exchange() -> Result<(MultiAssets, MultiAssets), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }

                fn universal_alias() -> Result<Junction, BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }

                fn transact_origin_and_runtime_call() -> Result<(Location, RuntimeCall), BenchmarkError> {
                    Ok((NativeLocation::get(), frame_system::Call::remark_with_event { remark: vec![] }.into()))
                }

                fn subscribe_origin() -> Result<Location, BenchmarkError> {
                    Ok(NativeLocation::get())
                }

                fn claimable_asset() -> Result<(Location, Location, MultiAssets), BenchmarkError> {
                    let origin = NativeLocation::get();
                    let assets: MultiAssets = (Concrete(NativeLocation::get()), 1_000 * UNITS).into();
                    let ticket = Location { parents: 0, interior: Here };
                    Ok((origin, ticket, assets))
                }

                fn unlockable_asset() -> Result<(Location, Location, MultiAsset), BenchmarkError> {
                    Err(BenchmarkError::Skip)
                }
            }

            type XcmBalances = pallet_xcm_benchmarks::fungible::Pallet::<Runtime>;
            type XcmGeneric = pallet_xcm_benchmarks::generic::Pallet::<Runtime>;

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
                //TODO: use from relay_well_known_keys::ACTIVE_CONFIG
                hex_literal::hex!("06de3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmarks!(params, batches);

            Ok(batches)
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, |_| None)
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![]
        }
    }
}

cumulus_pallet_parachain_system::register_validate_block! {
    Runtime = Runtime,
    BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
}

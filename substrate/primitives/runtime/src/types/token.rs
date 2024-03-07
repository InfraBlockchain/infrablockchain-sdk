//! Module that defines System Token type

use crate::{
	codec::{Decode, Encode, MaxEncodedLen},
	scale_info::TypeInfo,
	types::vote::*,
	RuntimeDebug,
};
use bounded_collections::{BoundedVec, ConstU32};
use sp_arithmetic::traits::Zero;
use sp_std::prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// ParaId of Relay Chain
pub const RELAY_CHAIN_PARA_ID: SystemTokenParaId = 0;

// TODO: SystemTokenInterface

/// General type of unix time
pub type StandardUnixTime = u64;
/// General type of exchange rate
pub type ExchangeRate = u64;
/// General para id type for System Token
pub type SystemTokenParaId = u32;
/// General pallet id type for System Token
pub type SystemTokenPalletId = u8;
/// General asset id type for System Token
pub type SystemTokenAssetId = u32;
/// Generale weight type for System Token
pub type SystemTokenWeight = u128;
/// General balance type for System Token
pub type SystemTokenBalance = u128;
/// General decimal type for System Token
pub type SystemTokenDecimal = u8;
/// Bounded name for System Token
pub type BoundedStringMetadata = BoundedVec<u8, ConstU32<20>>;

/// System Token configuration for transaction fee calculation
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	serde::Serialize,
	serde::Deserialize,
)]
pub struct SystemTokenConfig<SystemTokenWeight> {
	/// Detail of base system token
	pub base_system_token_detail: BaseSystemTokenDetail<SystemTokenWeight>,
	/// Scale of weight for calculating tx fee
	pub weight_scale: SystemTokenWeight,
}

#[derive(RuntimeDebug)]
pub enum InitError {
	/// Base system token is not initialized
	InvalidBaseSystemTokenDetail,
	/// Weight scale is not initialized
	InvalidWeightScale,
}

impl<SystemTokenWeight: Clone + PartialEq + Zero + sp_std::fmt::Debug> SystemTokenConfig<SystemTokenWeight> {
	/// Clone of base_currency type of `BaseSystemTokenDetail`
	pub fn base_currency(&self) -> Fiat {
		self.base_system_token_detail.clone().base_currency
	}
	/// Clone of base_weight type of `BaseSystemTokenDetail`
	pub fn base_weight(&self) -> SystemTokenWeight {
		self.base_system_token_detail.clone().base_weight
	}
	/// Clone of base_decimals type of `BaseSystemTokenDetail`
	pub fn base_decimals(&self) -> u8 {
		self.base_system_token_detail.clone().base_decimals
	}

	pub fn check_validity(&self) -> Result<(), InitError> {
		if self.base_system_token_detail.base_weight == Zero::zero() {
			return Err(InitError::InvalidBaseSystemTokenDetail)
		}
		if self.weight_scale == Zero::zero() {
			return Err(InitError::InvalidWeightScale)
		}
		Ok(())
	}

	pub fn panic_if_not_validated(&self) {
		if let Err(err) = self.check_validity() {
			panic!("System configuration is not initalized: {:?}\nSCfg:\n{:#?}", err, self);
		}
	}
}

#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	serde::Serialize,
	serde::Deserialize,
)]
/// Detail of base system token
pub struct BaseSystemTokenDetail<SystemTokenWeight> {
	/// Currency type of base system token
	pub base_currency: Fiat,
	/// Weight of base system token
	pub base_weight: SystemTokenWeight,
	/// Decimal of base system token
	pub base_decimals: u8,
}

impl<SystemTokenWeight> BaseSystemTokenDetail<SystemTokenWeight> {
	fn new(fiat: Fiat, weight: SystemTokenWeight, decimals: u8) -> Self {
		Self { base_currency: fiat, base_weight: weight, base_decimals: decimals }
	}
}

pub const MAX_REQUESTED_ASSETS: u32 = 1;
/// Upper limit of number of assets to be requested
pub type BoundedRequestedAssets<AssetId, Balance> = BoundedVec<RemoteAssetMetadata<AssetId, Balance>, ConstU32<MAX_REQUESTED_ASSETS>>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Default, Hash))]
pub struct RemoteAssetMetadata<AssetId, Balance> {
	/// General asset id on Runtime(e.g `MultiLocation`)
	#[codec(compact)]
	pub asset_id: AssetId,
	/// Human readable name of System Token. Accept unbounded 'Vec<u8>' because it would be checked `bounded` when initiated
	pub name: Vec<u8>,
	/// Human readable symbol of System Token. Accept unbounded 'Vec<u8>' because it would be checked `bounded` when initiated
	pub symbol: Vec<u8>,
	/// Currency type of base system token
	pub currency_type: Fiat,
	/// Decimal of base system token
	pub decimals: u8,
	/// Minimum balance of system token
	#[codec(compact)]
	pub min_balance: Balance,
}

/// API for interacting with local assets on Runtime
pub trait LocalAssetProvider<Asset, Account> {
	/// Get a list of local assets created on local chain
	fn system_token_list() -> Vec<Asset>;
	/// Get the most account balance of given `asset_id`
	fn get_most_account_system_token_balance(
		asset_ids: impl IntoIterator<Item = Asset>,
		account: Account,
	) -> Asset;
}

pub trait AssetMetadataProvider {
	fn requested(byte: Vec<u8>);
}

pub trait AssetLinkInterface<AssetId> {
	type Error;

	fn link(asset_id: &AssetId, parents: u8, original: AssetId) -> Result<(), Self::Error>;
	fn unlink(asset_id: &AssetId) -> Result<(), Self::Error>;
}

impl<AssetId> AssetLinkInterface<AssetId> for () {
	type Error = ();

	fn link(
		_asset_id: &AssetId,
		_parents: u8,
		_original: AssetId,
	) -> Result<(), Self::Error> {
		Ok(())
	}
	fn unlink(_asset_id: &AssetId) -> Result<(), Self::Error> {
		Ok(())
	}
}

#[derive(
	Clone,
	Encode,
	Decode,
	Eq,
	PartialEq,
	RuntimeDebug,
	MaxEncodedLen,
	TypeInfo,
	Default,
	serde::Serialize,
	serde::Deserialize,
)]
#[cfg_attr(feature = "std", derive(Hash))]
pub enum Fiat {
	#[default]
	USD,
	AED,
	AFN,
	ALL,
	AMD,
	ANG,
	AOA,
	ARS,
	AUD,
	AWG,
	AZN,
	BAM,
	BBD,
	BDT,
	BGN,
	BHD,
	BIF,
	BMD,
	BND,
	BOB,
	BRL,
	BSD,
	BTN,
	BWP,
	BYN,
	BZD,
	CAD,
	CDF,
	CHF,
	CLP,
	CNY,
	COP,
	CRC,
	CUP,
	CVE,
	CZK,
	DJF,
	DKK,
	DOP,
	DZD,
	EGP,
	ERN,
	ETB,
	EUR,
	FJD,
	FKP,
	FOK,
	GBP,
	GEL,
	GGP,
	GHS,
	GIP,
	GMD,
	GNF,
	GTQ,
	GYD,
	HKD,
	HNL,
	HRK,
	HTG,
	HUF,
	IDR,
	ILS,
	IMP,
	INR,
	IQD,
	IRR,
	ISK,
	JEP,
	JMD,
	JOD,
	JPY,
	KES,
	KGS,
	KHR,
	KID,
	KMF,
	KRW,
	KWD,
	KYD,
	KZT,
	LAK,
	LBP,
	LKR,
	LRD,
	LSL,
	LYD,
	MAD,
	MDL,
	MGA,
	MKD,
	MMK,
	MNT,
	MOP,
	MRU,
	MUR,
	MVR,
	MWK,
	MXN,
	MYR,
	MZN,
	NAD,
	NGN,
	NIO,
	NOK,
	NPR,
	NZD,
	OMR,
	PAB,
	PEN,
	PGK,
	PHP,
	PKR,
	PLN,
	PYG,
	QAR,
	RON,
	RSD,
	RUB,
	RWF,
	SAR,
	SBD,
	SCR,
	SDG,
	SEK,
	SGD,
	SHP,
	SLE,
	SLL,
	SOS,
	SRD,
	SSP,
	STN,
	SYP,
	SZL,
	THB,
	TJS,
	TMT,
	TND,
	TOP,
	TRY,
	TTD,
	TVD,
	TWD,
	TZS,
	UAH,
	UGX,
	UYU,
	UZS,
	VES,
	VND,
	VUV,
	WST,
	XAF,
	XCD,
	XDR,
	XOF,
	XPF,
	YER,
	ZAR,
	ZMW,
	ZWL,
}

impl TryFrom<Vec<u8>> for Fiat {
	type Error = ();

	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
		Ok(match value.as_slice() {
			b"USD" => Fiat::USD,
			b"AED" => Fiat::AED,
			b"AFN" => Fiat::AFN,
			b"ALL" => Fiat::ALL,
			b"AMD" => Fiat::AMD,
			b"ANG" => Fiat::ANG,
			b"AOA" => Fiat::AOA,
			b"ARS" => Fiat::ARS,
			b"AUD" => Fiat::AUD,
			b"AWG" => Fiat::AWG,
			b"AZN" => Fiat::AZN,
			b"BAM" => Fiat::BAM,
			b"BBD" => Fiat::BBD,
			b"BDT" => Fiat::BDT,
			b"BGN" => Fiat::BGN,
			b"BHD" => Fiat::BHD,
			b"BIF" => Fiat::BIF,
			b"BMD" => Fiat::BMD,
			b"BND" => Fiat::BND,
			b"BOB" => Fiat::BOB,
			b"BRL" => Fiat::BRL,
			b"BSD" => Fiat::BSD,
			b"BTN" => Fiat::BTN,
			b"BWP" => Fiat::BWP,
			b"BYN" => Fiat::BYN,
			b"BZD" => Fiat::BZD,
			b"CAD" => Fiat::CAD,
			b"CDF" => Fiat::CDF,
			b"CHF" => Fiat::CHF,
			b"CLP" => Fiat::CLP,
			b"CNY" => Fiat::CNY,
			b"COP" => Fiat::COP,
			b"CRC" => Fiat::CRC,
			b"CUP" => Fiat::CUP,
			b"CVE" => Fiat::CVE,
			b"CZK" => Fiat::CZK,
			b"DJF" => Fiat::DJF,
			b"DKK" => Fiat::DKK,
			b"DOP" => Fiat::DOP,
			b"DZD" => Fiat::DZD,
			b"EGP" => Fiat::EGP,
			b"ERN" => Fiat::ERN,
			b"ETB" => Fiat::ETB,
			b"EUR" => Fiat::EUR,
			b"FJD" => Fiat::FJD,
			b"FKP" => Fiat::FKP,
			b"FOK" => Fiat::FOK,
			b"GBP" => Fiat::GBP,
			b"GEL" => Fiat::GEL,
			b"GGP" => Fiat::GGP,
			b"GHS" => Fiat::GHS,
			b"GIP" => Fiat::GIP,
			b"GMD" => Fiat::GMD,
			b"GNF" => Fiat::GNF,
			b"GTQ" => Fiat::GTQ,
			b"GYD" => Fiat::GYD,
			b"HKD" => Fiat::HKD,
			b"HNL" => Fiat::HNL,
			b"HRK" => Fiat::HRK,
			b"HTG" => Fiat::HTG,
			b"HUF" => Fiat::HUF,
			b"IDR" => Fiat::IDR,
			b"ILS" => Fiat::ILS,
			b"IMP" => Fiat::IMP,
			b"INR" => Fiat::INR,
			b"IQD" => Fiat::IQD,
			b"IRR" => Fiat::IRR,
			b"ISK" => Fiat::ISK,
			b"JEP" => Fiat::JEP,
			b"JMD" => Fiat::JMD,
			b"JOD" => Fiat::JOD,
			b"JPY" => Fiat::JPY,
			b"KES" => Fiat::KES,
			b"KGS" => Fiat::KGS,
			b"KHR" => Fiat::KHR,
			b"KID" => Fiat::KID,
			b"KMF" => Fiat::KMF,
			b"KRW" => Fiat::KRW,
			b"KWD" => Fiat::KWD,
			b"KYD" => Fiat::KYD,
			b"KZT" => Fiat::KZT,
			b"LAK" => Fiat::LAK,
			b"LBP" => Fiat::LBP,
			b"LKR" => Fiat::LKR,
			b"LRD" => Fiat::LRD,
			b"LSL" => Fiat::LSL,
			b"LYD" => Fiat::LYD,
			b"MAD" => Fiat::MAD,
			b"MDL" => Fiat::MDL,
			b"MGA" => Fiat::MGA,
			b"MKD" => Fiat::MKD,
			b"MMK" => Fiat::MMK,
			b"MNT" => Fiat::MNT,
			b"MOP" => Fiat::MOP,
			b"MRU" => Fiat::MRU,
			b"MUR" => Fiat::MUR,
			b"MVR" => Fiat::MVR,
			b"MWK" => Fiat::MWK,
			b"MXN" => Fiat::MXN,
			b"MYR" => Fiat::MYR,
			b"MZN" => Fiat::MZN,
			b"NAD" => Fiat::NAD,
			b"NGN" => Fiat::NGN,
			b"NIO" => Fiat::NIO,
			b"NOK" => Fiat::NOK,
			b"NPR" => Fiat::NPR,
			b"NZD" => Fiat::NZD,
			b"OMR" => Fiat::OMR,
			b"PAB" => Fiat::PAB,
			b"PEN" => Fiat::PEN,
			b"PGK" => Fiat::PGK,
			b"PHP" => Fiat::PHP,
			b"PKR" => Fiat::PKR,
			b"PLN" => Fiat::PLN,
			b"PYG" => Fiat::PYG,
			b"QAR" => Fiat::QAR,
			b"RON" => Fiat::RON,
			b"RSD" => Fiat::RSD,
			b"RUB" => Fiat::RUB,
			b"RWF" => Fiat::RWF,
			b"SAR" => Fiat::SAR,
			b"SBD" => Fiat::SBD,
			b"SCR" => Fiat::SCR,
			b"SDG" => Fiat::SDG,
			b"SEK" => Fiat::SEK,
			b"SGD" => Fiat::SGD,
			b"SHP" => Fiat::SHP,
			b"SLE" => Fiat::SLE,
			b"SLL" => Fiat::SLL,
			b"SOS" => Fiat::SOS,
			b"SRD" => Fiat::SRD,
			b"SSP" => Fiat::SSP,
			b"STN" => Fiat::STN,
			b"SYP" => Fiat::SYP,
			b"SZL" => Fiat::SZL,
			b"THB" => Fiat::THB,
			b"TJS" => Fiat::TJS,
			b"TMT" => Fiat::TMT,
			b"TND" => Fiat::TND,
			b"TOP" => Fiat::TOP,
			b"TRY" => Fiat::TRY,
			b"TTD" => Fiat::TTD,
			b"TVD" => Fiat::TVD,
			b"TWD" => Fiat::TWD,
			b"TZS" => Fiat::TZS,
			b"UAH" => Fiat::UAH,
			b"UGX" => Fiat::UGX,
			b"UYU" => Fiat::UYU,
			b"UZS" => Fiat::UZS,
			b"VES" => Fiat::VES,
			b"VND" => Fiat::VND,
			b"VUV" => Fiat::VUV,
			b"WST" => Fiat::WST,
			b"XAF" => Fiat::XAF,
			b"XCD" => Fiat::XCD,
			b"XDR" => Fiat::XDR,
			b"XOF" => Fiat::XOF,
			b"XPF" => Fiat::XPF,
			b"YER" => Fiat::YER,
			b"ZAR" => Fiat::ZAR,
			b"ZMW" => Fiat::ZMW,
			b"ZWL" => Fiat::ZWL,
			_ => return Err(()),
		})
	}
}

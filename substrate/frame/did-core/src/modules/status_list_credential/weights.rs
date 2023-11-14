//! Autogenerated weights for status_list_credential
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-03, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/dock-node
// benchmark
// pallet
// --wasm-execution=compiled
// --pallet=status_list_credential
// --extra
// --repeat=20
// --extrinsic=*
// --steps=50
// --template=node/module-weight-template.hbs
// --output=./pallets/core/src/modules/revoke/weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for status_list_credential.
pub trait WeightInfo {
    fn update_sr25519(r: u32) -> Weight;
    fn update_ed25519(r: u32) -> Weight;
    fn update_secp256k1(r: u32) -> Weight;
    fn remove_sr25519() -> Weight;
    fn remove_ed25519() -> Weight;
    fn remove_secp256k1() -> Weight;
    fn create(r: u32, c: u32) -> Weight;
}

/// Weights for status_list_credential using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn update_sr25519(r: u32) -> Weight {
        Weight::from_ref_time(69_710_000) // Standard Error: 0
            .saturating_add(Weight::from_ref_time(4_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    fn update_ed25519(r: u32) -> Weight {
        Weight::from_ref_time(68_958_000) // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    fn update_secp256k1(r: u32) -> Weight {
        Weight::from_ref_time(164_889_000) // Standard Error: 0
            .saturating_add(Weight::from_ref_time(4_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    fn remove_sr25519() -> Weight {
        Weight::from_ref_time(70_000_000)
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    fn remove_ed25519() -> Weight {
        Weight::from_ref_time(72_000_000)
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    fn remove_secp256k1() -> Weight {
        Weight::from_ref_time(159_000_000)
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    fn create(_r: u32, c: u32) -> Weight {
        Weight::from_ref_time(15_669_000) // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(74_000).saturating_mul(c as u64))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn update_sr25519(r: u32) -> Weight {
        Weight::from_ref_time(69_710_000) // Standard Error: 0
            .saturating_add(Weight::from_ref_time(4_000).saturating_mul(r as u64))
            .saturating_add(RocksDbWeight::get().reads(4))
            .saturating_add(RocksDbWeight::get().writes(3))
    }
    fn update_ed25519(r: u32) -> Weight {
        Weight::from_ref_time(68_958_000) // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000).saturating_mul(r as u64))
            .saturating_add(RocksDbWeight::get().reads(4))
            .saturating_add(RocksDbWeight::get().writes(3))
    }
    fn update_secp256k1(r: u32) -> Weight {
        Weight::from_ref_time(164_889_000) // Standard Error: 0
            .saturating_add(Weight::from_ref_time(4_000).saturating_mul(r as u64))
            .saturating_add(RocksDbWeight::get().reads(4))
            .saturating_add(RocksDbWeight::get().writes(3))
    }
    fn remove_sr25519() -> Weight {
        Weight::from_ref_time(70_000_000)
            .saturating_add(RocksDbWeight::get().reads(4))
            .saturating_add(RocksDbWeight::get().writes(3))
    }
    fn remove_ed25519() -> Weight {
        Weight::from_ref_time(72_000_000)
            .saturating_add(RocksDbWeight::get().reads(4))
            .saturating_add(RocksDbWeight::get().writes(3))
    }
    fn remove_secp256k1() -> Weight {
        Weight::from_ref_time(159_000_000)
            .saturating_add(RocksDbWeight::get().reads(4))
            .saturating_add(RocksDbWeight::get().writes(3))
    }
    fn create(_r: u32, c: u32) -> Weight {
        Weight::from_ref_time(15_669_000) // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(74_000).saturating_mul(c as u64))
            .saturating_add(RocksDbWeight::get().reads(2))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
}
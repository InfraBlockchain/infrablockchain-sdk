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

mod parachain;
mod relay_chain;

use codec::DecodeLimit;
use polkadot_core_primitives::AccountId;
use parachain_primitives::primitives::Id as ParaId;
use sp_runtime::{traits::AccountIdConversion, BuildStorage};
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain, TestExt};

use frame_support::assert_ok;
use xcm::{latest::prelude::*, MAX_XCM_DECODE_DEPTH};

use arbitrary::{Arbitrary, Error, Unstructured};

pub const INITIAL_BALANCE: u128 = 1_000_000_000;

decl_test_parachain! {
	pub struct ParaA {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MsgQueue,
		DmpMessageHandler = parachain::MsgQueue,
		new_ext = para_ext(1),
	}
}

decl_test_parachain! {
	pub struct ParaB {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MsgQueue,
		DmpMessageHandler = parachain::MsgQueue,
		new_ext = para_ext(2),
	}
}

decl_test_parachain! {
	pub struct ParaC {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MsgQueue,
		DmpMessageHandler = parachain::MsgQueue,
		new_ext = para_ext(3),
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay_chain::Runtime,
		RuntimeCall = relay_chain::RuntimeCall,
		RuntimeEvent = relay_chain::RuntimeEvent,
		XcmConfig = relay_chain::XcmConfig,
		MessageQueue = relay_chain::MessageQueue,
		System = relay_chain::System,
		new_ext = relay_ext(),
	}
}

decl_test_network! {
	pub struct MockNet {
		relay_chain = Relay,
		parachains = vec![
			(1, ParaA),
			(2, ParaB),
			(3, ParaC),
		],
	}
}

// An XCM message that will be generated by the fuzzer through the Arbitrary trait
struct XcmMessage {
	// Source chain
	source: u32,
	// Destination chain
	destination: u32,
	// XCM message
	message: Xcm<()>,
}

impl<'a> Arbitrary<'a> for XcmMessage {
	fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, Error> {
		let source: u32 = u.arbitrary()?;
		let destination: u32 = u.arbitrary()?;
		let mut encoded_message: &[u8] = u.arbitrary()?;
		if let Ok(message) =
			DecodeLimit::decode_with_depth_limit(MAX_XCM_DECODE_DEPTH, &mut encoded_message)
		{
			return Ok(XcmMessage { source, destination, message })
		}
		Err(Error::IncorrectFormat)
	}
}

pub fn para_account_id(id: u32) -> relay_chain::AccountId {
	ParaId::from(id).into_account_truncating()
}

pub fn para_ext(para_id: u32) -> sp_io::TestExternalities {
	use parachain::{MsgQueue, Runtime, System};

	let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: (0..6).map(|i| ([i; 32].into(), INITIAL_BALANCE)).collect(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
		MsgQueue::set_para_id(para_id.into());
	});
	ext
}

pub fn relay_ext() -> sp_io::TestExternalities {
	use relay_chain::{Runtime, System};

	let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

	let mut balances: Vec<(AccountId, u128)> = vec![];
	balances.append(&mut (1..=3).map(|i| (para_account_id(i), INITIAL_BALANCE)).collect());
	balances.append(&mut (0..6).map(|i| ([i; 32].into(), INITIAL_BALANCE)).collect());

	pallet_balances::GenesisConfig::<Runtime> { balances }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub type RelayChainPalletXcm = pallet_xcm::Pallet<relay_chain::Runtime>;
pub type ParachainPalletXcm = pallet_xcm::Pallet<parachain::Runtime>;

fn run_input(xcm_messages: [XcmMessage; 5]) {
	MockNet::reset();

	#[cfg(not(fuzzing))]
	println!();

	for xcm_message in xcm_messages {
		if xcm_message.source % 4 == 0 {
			// We get the destination for the message
			let parachain_id = (xcm_message.destination % 3) + 1;
			let destination: MultiLocation = Parachain(parachain_id).into();
			#[cfg(not(fuzzing))]
			{
				println!("  source:      Relay Chain");
				println!("  destination: Parachain {parachain_id}");
				println!("  message:     {:?}", xcm_message.message);
			}
			Relay::execute_with(|| {
				assert_ok!(RelayChainPalletXcm::send_xcm(Here, destination, xcm_message.message));
			})
		} else {
			// We get the source's execution method
			let execute_with = match xcm_message.source % 4 {
				1 => ParaA::execute_with,
				2 => ParaB::execute_with,
				_ => ParaC::execute_with,
			};
			// We get the destination for the message
			let destination: MultiLocation = match xcm_message.destination % 4 {
				n @ 1..=3 => (Parent, Parachain(n)).into(),
				_ => Parent.into(),
			};
			#[cfg(not(fuzzing))]
			{
				let destination_str = match xcm_message.destination % 4 {
					n @ 1..=3 => format!("Parachain {n}"),
					_ => "Relay Chain".to_string(),
				};
				println!("  source:      Parachain {}", xcm_message.source % 4);
				println!("  destination: {}", destination_str);
				println!("  message:     {:?}", xcm_message.message);
			}
			// We execute the message with the appropriate source and destination
			execute_with(|| {
				assert_ok!(ParachainPalletXcm::send_xcm(Here, destination, xcm_message.message));
			});
		}
		#[cfg(not(fuzzing))]
		println!();
	}
	Relay::execute_with(|| {});
}

fn main() {
	#[cfg(fuzzing)]
	{
		loop {
			honggfuzz::fuzz!(|xcm_messages: [XcmMessage; 5]| {
				run_input(xcm_messages);
			})
		}
	}
	#[cfg(not(fuzzing))]
	{
		use std::{env, fs, fs::File, io::Read};
		let args: Vec<_> = env::args().collect();
		let md = fs::metadata(&args[1]).unwrap();
		let all_files = match md.is_dir() {
			true => fs::read_dir(&args[1])
				.unwrap()
				.map(|x| x.unwrap().path().to_str().unwrap().to_string())
				.collect::<Vec<String>>(),
			false => (args[1..]).to_vec(),
		};
		println!("All_files {:?}", all_files);
		for argument in all_files {
			println!("Now doing file {:?}", argument);
			let mut buffer: Vec<u8> = Vec::new();
			let mut f = File::open(argument).unwrap();
			f.read_to_end(&mut buffer).unwrap();
			let mut unstructured = Unstructured::new(&buffer);
			if let Ok(xcm_messages) = unstructured.arbitrary() {
				run_input(xcm_messages);
			}
		}
	}
}

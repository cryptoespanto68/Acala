// This file is part of Acala.

// Copyright (C) 2020-2024 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_transaction_pause
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-42-209`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_transaction_pause.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_transaction_pause::WeightInfo for WeightInfo<T> {
	// Storage: `TransactionPause::PausedTransactions` (r:1 w:1)
	// Proof: `TransactionPause::PausedTransactions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn pause_transaction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1172`
		//  Estimated: `4637`
		// Minimum execution time: 18_497 nanoseconds.
		Weight::from_parts(19_034_000, 4637)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `TransactionPause::PausedTransactions` (r:1 w:1)
	// Proof: `TransactionPause::PausedTransactions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unpause_transaction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1223`
		//  Estimated: `4688`
		// Minimum execution time: 19_712 nanoseconds.
		Weight::from_parts(20_100_000, 4688)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `TransactionPause::PausedEvmPrecompiles` (r:1 w:1)
	// Proof: `TransactionPause::PausedEvmPrecompiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn pause_evm_precompile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1172`
		//  Estimated: `4637`
		// Minimum execution time: 18_231 nanoseconds.
		Weight::from_parts(18_697_000, 4637)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `TransactionPause::PausedEvmPrecompiles` (r:1 w:1)
	// Proof: `TransactionPause::PausedEvmPrecompiles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unpause_evm_precompile() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1233`
		//  Estimated: `4698`
		// Minimum execution time: 19_852 nanoseconds.
		Weight::from_parts(20_246_000, 4698)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

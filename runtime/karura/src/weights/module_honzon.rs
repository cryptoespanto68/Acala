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

//! Autogenerated weights for module_honzon
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-37-73`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_honzon.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_honzon::WeightInfo for WeightInfo<T> {
	// Storage: `Honzon::Authorization` (r:1 w:1)
	// Proof: `Honzon::Authorization` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	fn authorize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1448`
		//  Estimated: `3633`
		// Minimum execution time: 38_484 nanoseconds.
		Weight::from_parts(39_130_000, 3633)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `Honzon::Authorization` (r:1 w:1)
	// Proof: `Honzon::Authorization` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	fn unauthorize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1668`
		//  Estimated: `3633`
		// Minimum execution time: 43_002 nanoseconds.
		Weight::from_parts(43_903_000, 3633)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `Honzon::Authorization` (r:4 w:4)
	// Proof: `Honzon::Authorization` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Balances::Reserves` (r:1 w:1)
	// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 4]`.
	fn unauthorize_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1287 + c * (176 ±0)`
		//  Estimated: `3633 + c * (2622 ±0)`
		// Minimum execution time: 21_455 nanoseconds.
		Weight::from_parts(27_945_826, 3633)
			// Standard Error: 210_866
			.saturating_add(Weight::from_parts(6_673_194, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2622).saturating_mul(c.into()))
	}
	// Storage: `EmergencyShutdown::IsShutdown` (r:1 w:0)
	// Proof: `EmergencyShutdown::IsShutdown` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	// Storage: `CdpEngine::CollateralParams` (r:1 w:0)
	// Proof: `CdpEngine::CollateralParams` (`max_values`: None, `max_size`: Some(135), added: 2610, mode: `MaxEncodedLen`)
	// Storage: `Loans::Positions` (r:1 w:1)
	// Proof: `Loans::Positions` (`max_values`: None, `max_size`: Some(123), added: 2598, mode: `MaxEncodedLen`)
	// Storage: `Rewards::PoolInfos` (r:1 w:1)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Rewards::SharesAndWithdrawnRewards` (r:1 w:1)
	// Proof: `Rewards::SharesAndWithdrawnRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Loans::TotalPositions` (r:1 w:1)
	// Proof: `Loans::TotalPositions` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:3 w:3)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `CdpEngine::DebitExchangeRate` (r:1 w:0)
	// Proof: `CdpEngine::DebitExchangeRate` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Prices::LockedPrice` (r:2 w:0)
	// Proof: `Prices::LockedPrice` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `AcalaOracle::Values` (r:1 w:0)
	// Proof: `AcalaOracle::Values` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:2 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn adjust_loan() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2401`
		//  Estimated: `8856`
		// Minimum execution time: 122_385 nanoseconds.
		Weight::from_parts(125_606_000, 8856)
			.saturating_add(T::DbWeight::get().reads(17))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `EmergencyShutdown::IsShutdown` (r:1 w:0)
	// Proof: `EmergencyShutdown::IsShutdown` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	// Storage: `Honzon::Authorization` (r:1 w:0)
	// Proof: `Honzon::Authorization` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Loans::Positions` (r:2 w:2)
	// Proof: `Loans::Positions` (`max_values`: None, `max_size`: Some(123), added: 2598, mode: `MaxEncodedLen`)
	// Storage: `CdpEngine::DebitExchangeRate` (r:1 w:0)
	// Proof: `CdpEngine::DebitExchangeRate` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Prices::LockedPrice` (r:2 w:0)
	// Proof: `Prices::LockedPrice` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `AcalaOracle::Values` (r:1 w:0)
	// Proof: `AcalaOracle::Values` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:2 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `CdpEngine::CollateralParams` (r:1 w:0)
	// Proof: `CdpEngine::CollateralParams` (`max_values`: None, `max_size`: Some(135), added: 2610, mode: `MaxEncodedLen`)
	// Storage: `Rewards::SharesAndWithdrawnRewards` (r:2 w:2)
	// Proof: `Rewards::SharesAndWithdrawnRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Rewards::PoolInfos` (r:1 w:1)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Loans::TotalPositions` (r:1 w:1)
	// Proof: `Loans::TotalPositions` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	fn transfer_loan_from() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2746`
		//  Estimated: `8686`
		// Minimum execution time: 101_419 nanoseconds.
		Weight::from_parts(104_870_000, 8686)
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: `EmergencyShutdown::IsShutdown` (r:1 w:0)
	// Proof: `EmergencyShutdown::IsShutdown` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	// Storage: `Loans::Positions` (r:1 w:1)
	// Proof: `Loans::Positions` (`max_values`: None, `max_size`: Some(123), added: 2598, mode: `MaxEncodedLen`)
	// Storage: `Prices::LockedPrice` (r:2 w:0)
	// Proof: `Prices::LockedPrice` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `AcalaOracle::Values` (r:1 w:0)
	// Proof: `AcalaOracle::Values` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:2 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `CdpEngine::DebitExchangeRate` (r:1 w:0)
	// Proof: `CdpEngine::DebitExchangeRate` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `CdpEngine::CollateralParams` (r:1 w:0)
	// Proof: `CdpEngine::CollateralParams` (`max_values`: None, `max_size`: Some(135), added: 2610, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:8 w:6)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:3 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `CdpTreasury::DebitPool` (r:1 w:1)
	// Proof: `CdpTreasury::DebitPool` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	// Storage: `Rewards::SharesAndWithdrawnRewards` (r:1 w:1)
	// Proof: `Rewards::SharesAndWithdrawnRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Rewards::PoolInfos` (r:1 w:1)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Loans::TotalPositions` (r:1 w:1)
	// Proof: `Loans::TotalPositions` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	// Storage: `AuctionManager::TotalCollateralInAuction` (r:1 w:0)
	// Proof: `AuctionManager::TotalCollateralInAuction` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Dex::TradingPairStatuses` (r:3 w:0)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::LiquidityPool` (r:3 w:2)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `StableAsset::Pools` (r:2 w:0)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `AggregatedDex::AggregatedSwapPaths` (r:1 w:0)
	// Proof: `AggregatedDex::AggregatedSwapPaths` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:0)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn close_loan_has_debit_by_dex() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5423`
		//  Estimated: `21966`
		// Minimum execution time: 355_391 nanoseconds.
		Weight::from_parts(360_410_000, 21966)
			.saturating_add(T::DbWeight::get().reads(39))
			.saturating_add(T::DbWeight::get().writes(15))
	}
	// Storage: `CdpEngine::CollateralParams` (r:1 w:0)
	// Proof: `CdpEngine::CollateralParams` (`max_values`: None, `max_size`: Some(135), added: 2610, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:4 w:4)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Dex::TradingPairStatuses` (r:3 w:0)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::LiquidityPool` (r:3 w:2)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `StableAsset::Pools` (r:2 w:0)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `AggregatedDex::AggregatedSwapPaths` (r:1 w:0)
	// Proof: `AggregatedDex::AggregatedSwapPaths` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `CdpEngine::DebitExchangeRate` (r:1 w:0)
	// Proof: `CdpEngine::DebitExchangeRate` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Loans::Positions` (r:1 w:1)
	// Proof: `Loans::Positions` (`max_values`: None, `max_size`: Some(123), added: 2598, mode: `MaxEncodedLen`)
	// Storage: `Rewards::PoolInfos` (r:1 w:1)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Rewards::SharesAndWithdrawnRewards` (r:1 w:1)
	// Proof: `Rewards::SharesAndWithdrawnRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Loans::TotalPositions` (r:1 w:1)
	// Proof: `Loans::TotalPositions` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	// Storage: `Prices::LockedPrice` (r:2 w:0)
	// Proof: `Prices::LockedPrice` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `AcalaOracle::Values` (r:1 w:0)
	// Proof: `AcalaOracle::Values` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:2 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn expand_position_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4709`
		//  Estimated: `11478`
		// Minimum execution time: 243_941 nanoseconds.
		Weight::from_parts(250_315_000, 11478)
			.saturating_add(T::DbWeight::get().reads(27))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	// Storage: `CdpEngine::CollateralParams` (r:1 w:0)
	// Proof: `CdpEngine::CollateralParams` (`max_values`: None, `max_size`: Some(135), added: 2610, mode: `MaxEncodedLen`)
	// Storage: `Loans::Positions` (r:1 w:1)
	// Proof: `Loans::Positions` (`max_values`: None, `max_size`: Some(123), added: 2598, mode: `MaxEncodedLen`)
	// Storage: `Dex::TradingPairStatuses` (r:3 w:0)
	// Proof: `Dex::TradingPairStatuses` (`max_values`: None, `max_size`: Some(195), added: 2670, mode: `MaxEncodedLen`)
	// Storage: `Dex::LiquidityPool` (r:3 w:2)
	// Proof: `Dex::LiquidityPool` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	// Storage: `StableAsset::Pools` (r:2 w:0)
	// Proof: `StableAsset::Pools` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `AggregatedDex::AggregatedSwapPaths` (r:1 w:0)
	// Proof: `AggregatedDex::AggregatedSwapPaths` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:7 w:5)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:0)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:2 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `CdpEngine::DebitExchangeRate` (r:1 w:0)
	// Proof: `CdpEngine::DebitExchangeRate` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Rewards::SharesAndWithdrawnRewards` (r:1 w:1)
	// Proof: `Rewards::SharesAndWithdrawnRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Rewards::PoolInfos` (r:1 w:1)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Loans::TotalPositions` (r:1 w:1)
	// Proof: `Loans::TotalPositions` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	fn shrink_position_debit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4615`
		//  Estimated: `19344`
		// Minimum execution time: 282_899 nanoseconds.
		Weight::from_parts(288_074_000, 19344)
			.saturating_add(T::DbWeight::get().reads(29))
			.saturating_add(T::DbWeight::get().writes(13))
	}
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:2 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `CdpEngine::CollateralParams` (r:2 w:0)
	// Proof: `CdpEngine::CollateralParams` (`max_values`: None, `max_size`: Some(135), added: 2610, mode: `MaxEncodedLen`)
	// Storage: `Loans::Positions` (r:2 w:2)
	// Proof: `Loans::Positions` (`max_values`: None, `max_size`: Some(123), added: 2598, mode: `MaxEncodedLen`)
	// Storage: `Loans::TotalPositions` (r:2 w:2)
	// Proof: `Loans::TotalPositions` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	// Storage: `CdpEngine::DebitExchangeRate` (r:2 w:0)
	// Proof: `CdpEngine::DebitExchangeRate` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Prices::LockedPrice` (r:3 w:0)
	// Proof: `Prices::LockedPrice` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `AcalaOracle::Values` (r:1 w:0)
	// Proof: `AcalaOracle::Values` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:2 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:0)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn transfer_debit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2921`
		//  Estimated: `8861`
		// Minimum execution time: 154_449 nanoseconds.
		Weight::from_parts(158_423_000, 8861)
			.saturating_add(T::DbWeight::get().reads(20))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `Loans::Positions` (r:1 w:0)
	// Proof: `Loans::Positions` (`max_values`: None, `max_size`: Some(123), added: 2598, mode: `MaxEncodedLen`)
	// Storage: `Prices::LockedPrice` (r:2 w:0)
	// Proof: `Prices::LockedPrice` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `AcalaOracle::Values` (r:1 w:0)
	// Proof: `AcalaOracle::Values` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::AssetMetadatas` (r:2 w:0)
	// Proof: `AssetRegistry::AssetMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:0)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `CdpEngine::DebitExchangeRate` (r:1 w:0)
	// Proof: `CdpEngine::DebitExchangeRate` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	fn precompile_get_current_collateral_ratio() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2020`
		//  Estimated: `7960`
		// Minimum execution time: 42_000 nanoseconds.
		Weight::from_parts(43_216_000, 7960)
			.saturating_add(T::DbWeight::get().reads(11))
	}
}

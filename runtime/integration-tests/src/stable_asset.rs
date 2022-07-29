// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
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

use crate::evm::alice_evm_addr;
use crate::payment::{with_fee_aggregated_path_call, with_fee_currency_call, with_fee_path_call, INFO, POST_INFO};
use crate::setup::*;
use module_aggregated_dex::SwapPath;
use module_support::{AggregatedSwapPath, ExchangeRate, Swap, SwapLimit, EVM as EVMTrait};
use primitives::{currency::AssetMetadata, evm::EvmAddress};
use sp_runtime::{
	traits::{SignedExtension, UniqueSaturatedInto},
	transaction_validity::{InvalidTransaction, TransactionValidityError},
	Percent,
};
use std::str::FromStr;

pub fn enable_stable_asset(currencies: Vec<CurrencyId>, amounts: Vec<u128>, minter: Option<AccountId>) {
	let pool_asset = CurrencyId::StableAssetPoolToken(0);
	let precisions = currencies.iter().map(|_| 1u128).collect::<Vec<_>>();
	assert_ok!(StableAsset::create_pool(
		Origin::root(),
		pool_asset,
		currencies, // assets
		precisions,
		10_000_000u128,           // mint fee
		20_000_000u128,           // swap fee
		50_000_000u128,           // redeem fee
		1_000u128,                // initialA
		AccountId::from(BOB),     // fee recipient
		AccountId::from(CHARLIE), // yield recipient
		1_000_000_000_000u128,    // precision
	));

	let asset_metadata = AssetMetadata {
		name: b"Token Name".to_vec(),
		symbol: b"TN".to_vec(),
		decimals: 12,
		minimal_balance: 1,
	};
	assert_ok!(AssetRegistry::register_stable_asset(
		RawOrigin::Root.into(),
		Box::new(asset_metadata.clone())
	));

	assert_ok!(StableAsset::mint(
		Origin::signed(minter.unwrap_or(AccountId::from(ALICE))),
		0,
		amounts,
		0u128
	));
}

pub fn enable_stable_asset2(
	currencies: Vec<CurrencyId>,
	amounts: Vec<u128>,
	minter: Option<AccountId>,
	precisions: Vec<u128>,
) {
	System::reset_events();
	let pool_asset = CurrencyId::StableAssetPoolToken(0);
	assert_ok!(StableAsset::create_pool(
		Origin::root(),
		pool_asset,
		currencies, // assets
		precisions,
		0u128,                    // mint fee
		25_000_000u128,           // swap fee
		30_000_000u128,           // redeem fee
		3_000u128,                // initialA
		AccountId::from(BOB),     // fee recipient
		AccountId::from(CHARLIE), // yield recipient
		1_000_000_000_000u128,    // precision
	));

	let asset_metadata = AssetMetadata {
		name: b"Token Name".to_vec(),
		symbol: b"TN".to_vec(),
		decimals: 12,
		minimal_balance: 1,
	};
	assert_ok!(AssetRegistry::register_stable_asset(
		RawOrigin::Root.into(),
		Box::new(asset_metadata.clone())
	));

	assert_ok!(StableAsset::mint(
		Origin::signed(minter.unwrap_or(AccountId::from(ALICE))),
		0,
		amounts,
		0u128
	));

	for ev in System::events() {
		println!("{:?}", ev);
	}
}

#[test]
fn stable_asset_mint_works() {
	ExtBuilder::default()
		.balances(vec![
			(
				// NetworkContractSource
				MockAddressMapping::get_account_id(&H160::from_low_u64_be(0)),
				NATIVE_CURRENCY,
				1_000_000_000 * dollar(NATIVE_CURRENCY),
			),
			(
				AccountId::from(ALICE),
				RELAY_CHAIN_CURRENCY,
				1_000_000_000 * dollar(NATIVE_CURRENCY),
			),
			(
				AccountId::from(ALICE),
				LIQUID_CURRENCY,
				12_000_000_000 * dollar(NATIVE_CURRENCY),
			),
		])
		.build()
		.execute_with(|| {
			let exchange_rate = Homa::current_exchange_rate();
			assert_eq!(exchange_rate, ExchangeRate::saturating_from_rational(1, 10)); // 0.1

			let ksm_target_amount = 10_000_123u128;
			let lksm_target_amount = 10_000_456u128;
			let account_id: AccountId = StableAssetPalletId::get().into_sub_account_truncating(0);
			enable_stable_asset(
				vec![RELAY_CHAIN_CURRENCY, LIQUID_CURRENCY],
				vec![ksm_target_amount, lksm_target_amount],
				None,
			);
			System::assert_last_event(Event::StableAsset(nutsfinance_stable_asset::Event::Minted {
				minter: AccountId::from(ALICE),
				pool_id: 0,
				a: 1000,
				input_amounts: vec![10_000_123u128, 10_000_456u128],
				min_output_amount: 0,
				balances: vec![10_000_123u128, 10_000_456u128],
				total_supply: 20_000_579u128,
				fee_amount: 20000,
				output_amount: 19_980_579u128,
			}));

			let ksm_balance = Currencies::free_balance(RELAY_CHAIN_CURRENCY, &account_id);
			let lksm_balance = Currencies::free_balance(LIQUID_CURRENCY, &account_id);
			assert_eq!(ksm_target_amount, ksm_balance);

			#[cfg(any(feature = "with-karura-runtime", feature = "with-acala-runtime"))]
			let lksm_amount = 100_004_560u128;
			#[cfg(feature = "with-mandala-runtime")]
			let lksm_amount = 10_000_456u128;
			assert_eq!(lksm_amount, lksm_balance);

			let converted_lksm_balance = exchange_rate.checked_mul_int(lksm_balance).unwrap_or_default();
			#[cfg(any(feature = "with-karura-runtime", feature = "with-acala-runtime"))]
			assert_eq!(converted_lksm_balance == lksm_target_amount, true);
			#[cfg(feature = "with-mandala-runtime")]
			assert_eq!(converted_lksm_balance < lksm_target_amount, true);
		});
}

#[test]
fn decimal12_stable_asset_swap_works() {
	let dollar = dollar(NATIVE_CURRENCY);
	ExtBuilder::default()
		.balances(vec![
			(AccountId::from(ALICE), RELAY_CHAIN_CURRENCY, 100_000_000_000 * dollar),
			(AccountId::from(ALICE), LIQUID_CURRENCY, 100_000_000_000 * dollar),
			(AccountId::from(ALICE), NATIVE_CURRENCY, 100_000_000_000 * dollar),
			(AccountId::from(ALICE), USD_CURRENCY, 100_000_000_000 * dollar),
			(AccountId::from(BOB), LIQUID_CURRENCY, 100_000_000_000 * dollar),
			(AccountId::from(BOB), NATIVE_CURRENCY, 10 * dollar),
		])
		.build()
		.execute_with(|| {
			let USDC = LIQUID_CURRENCY; // mock USDC
			enable_stable_asset2(
				vec![USD_CURRENCY, USDC],
				vec![105_135_943_457_866_107_649u128, 29_203_658_077_783_000_000u128],
				None,
				vec![1u128, 1_000_000u128], // precisions
			);
			assert_ok!(inject_liquidity2(
				AccountId::from(ALICE),
				USD_CURRENCY,
				RELAY_CHAIN_CURRENCY,
				2_108_218_744_012_282_700u128,
				9_154_402_197_674_471u128
			));
			assert_ok!(inject_liquidity2(
				AccountId::from(ALICE),
				RELAY_CHAIN_CURRENCY,
				NATIVE_CURRENCY,
				198_590_513_699_867u128,
				5_120_901_048_663_808u128
			));

			assert_ok!(AggregatedDex::update_aggregated_swap_paths(
				Origin::root(),
				vec![(
					(USDC, NATIVE_CURRENCY), // mock USDC, KAR
					Some(vec![
						SwapPath::Taiga(0, 1, 0),
						SwapPath::Dex(vec![USD_CURRENCY, RELAY_CHAIN_CURRENCY, NATIVE_CURRENCY])
					])
				),]
			));

			// Ok((1000000000000, 12362762499))
			println!(
				"{:?}",
				AcalaSwap::swap(
					&AccountId::from(BOB),
					USDC,
					NATIVE_CURRENCY,
					SwapLimit::ExactSupply(dollar, 0)
				)
			);
			// Ok((242_664_959_050, 3_000_000_025)) => ? USDC -> 0.003 KAR
			println!(
				"{:?}",
				AcalaSwap::swap(
					&AccountId::from(BOB),
					USDC,
					NATIVE_CURRENCY,
					SwapLimit::ExactTarget(Balance::MAX, 3_000_000_000)
				)
			);
		});
}

#[test]
fn stable_asset_swap_works() {
	let dollar = dollar(NATIVE_CURRENCY);
	ExtBuilder::default()
		.balances(vec![
			(AccountId::from(ALICE), RELAY_CHAIN_CURRENCY, 100_000_000_000 * dollar),
			(AccountId::from(ALICE), LIQUID_CURRENCY, 1_000_000_000_000_000 * dollar),
			(AccountId::from(ALICE), NATIVE_CURRENCY, 100_000_000_000 * dollar),
			(AccountId::from(ALICE), USD_CURRENCY, 100_000_000_000 * dollar),
			(AccountId::from(BOB), LIQUID_CURRENCY, 100_000_000_000 * dollar),
			(AccountId::from(BOB), NATIVE_CURRENCY, 100_000_000_000 * dollar),
		])
		.build()
		.execute_with(|| {
			let USDC = LIQUID_CURRENCY; // mock USDC
			enable_stable_asset2(
				vec![USD_CURRENCY, USDC],
				vec![105_135_943_457_866_107_649u128, 29_203_658_077_783_000_000_000_000u128],
				None,
				vec![1u128, 1u128], // precisions
			);
			assert_ok!(inject_liquidity2(
				AccountId::from(ALICE),
				USD_CURRENCY,
				RELAY_CHAIN_CURRENCY,
				2_108_218_744_012_282_700u128,
				9_154_402_197_674_471u128
			));
			assert_ok!(inject_liquidity2(
				AccountId::from(ALICE),
				RELAY_CHAIN_CURRENCY,
				NATIVE_CURRENCY,
				198_590_513_699_867u128,
				5_120_901_048_663_808u128
			));

			assert_ok!(AggregatedDex::update_aggregated_swap_paths(
				Origin::root(),
				vec![(
					(USDC, NATIVE_CURRENCY), // mock USDC, KAR
					Some(vec![
						SwapPath::Taiga(0, 1, 0),
						SwapPath::Dex(vec![USD_CURRENCY, RELAY_CHAIN_CURRENCY, NATIVE_CURRENCY])
					])
				),]
			));

			// Ok((1000000000000, 12362762499))
			// println!("{:?}", AcalaSwap::swap(
			// 	&AccountId::from(BOB),
			// 	USDC,
			// 	NATIVE_CURRENCY,
			// 	SwapLimit::ExactSupply(dollar, 0)
			// ));
			// Ok((242_664_959_050, 3_000_000_025)) => ? USDC -> 0.003 KAR
			println!(
				"{:?}",
				AcalaSwap::swap(
					&AccountId::from(BOB),
					USDC,
					NATIVE_CURRENCY,
					SwapLimit::ExactTarget(Balance::MAX, 3_000_000_000)
				)
			);
		});
}

#[test]
fn three_usd_pool_works() {
	let dollar = dollar(NATIVE_CURRENCY);
	let fee_pool_size = 5 * dollar;
	let alith = MockAddressMapping::get_account_id(&alice_evm_addr());
	ExtBuilder::default()
		.balances(vec![
			// alice() used to deploy erc20 contract
			(alice(), NATIVE_CURRENCY, 1_000_000 * dollar),
			(
				// NetworkContractSource
				MockAddressMapping::get_account_id(&H160::from_low_u64_be(0)),
				NATIVE_CURRENCY,
				1_000_000_000 * dollar,
			),
			// alith used to mint 3USD.
			(alith.clone(), NATIVE_CURRENCY, 1_000_000_000 * dollar),
			(alith.clone(), USD_CURRENCY, 1_000_000_000 * dollar),
			(AccountId::from(ALICE), USD_CURRENCY, 1_000_000 * dollar),
			(AccountId::from(BOB), USD_CURRENCY, 1_000_000 * dollar),
			(AccountId::from(BOB), NATIVE_CURRENCY, 1_000_000 * dollar),
		])
		.build()
		.execute_with(|| {
			let treasury_account = TreasuryAccount::get();
			let usdt: CurrencyId = CurrencyId::ForeignAsset(0);
			let usdc: CurrencyId = CurrencyId::Erc20(erc20_address_0());
			let usdt_sub_account: AccountId = TransactionPaymentPalletId::get().into_sub_account_truncating(usdt);
			let usdc_sub_account: AccountId = TransactionPaymentPalletId::get().into_sub_account_truncating(usdc);
			let minimal_balance: u128 = Balances::minimum_balance() / 10;

			assert_ok!(Currencies::update_balance(
				Origin::root(),
				MultiAddress::Id(treasury_account.clone()),
				NATIVE_CURRENCY,
				100 * dollar as i128,
			));

			// USDT is asset on Statemine
			assert_ok!(AssetRegistry::register_foreign_asset(
				Origin::root(),
				Box::new(
					MultiLocation::new(
						1,
						X2(
							Parachain(1000),
							GeneralKey("USDT".as_bytes().to_vec().try_into().unwrap())
						)
					)
					.into()
				),
				Box::new(AssetMetadata {
					name: b"USDT".to_vec(),
					symbol: b"USDT".to_vec(),
					decimals: 12,
					minimal_balance
				})
			));
			// deposit USDT to alith, used for liquidity provider
			assert_ok!(Currencies::deposit(usdt, &alith, 1_000_000 * dollar));
			// deposit USDT to BOB, used for swap
			assert_ok!(Currencies::deposit(usdt, &AccountId::from(BOB), 1_000_000 * dollar));
			assert_ok!(Currencies::deposit(usdt, &treasury_account, 10 * dollar));

			// USDC is Erc20 token
			deploy_erc20_contracts();

			let usdt_ed: u128 =
				(<Currencies as MultiCurrency<AccountId>>::minimum_balance(usdt)).unique_saturated_into();
			// erc20 minimum_balance/ED is 0.
			let usdc_ed: u128 =
				(<Currencies as MultiCurrency<AccountId>>::minimum_balance(usdc)).unique_saturated_into();
			assert_eq!(usdt_ed, minimal_balance);
			assert_eq!(usdc_ed, 0);

			let total_erc20 = 100_000_000_000_000_000_000_000u128;
			// alith has USDC when create Erc20 token
			assert_eq!(Currencies::free_balance(usdc, &alith), total_erc20);

			assert_ok!(EvmAccounts::claim_account(
				Origin::signed(AccountId::from(ALICE)),
				EvmAccounts::eth_address(&alice_key()),
				EvmAccounts::eth_sign(&alice_key(), &AccountId::from(ALICE))
			));
			assert_ok!(EvmAccounts::claim_account(
				Origin::signed(AccountId::from(BOB)),
				EvmAccounts::eth_address(&bob_key()),
				EvmAccounts::eth_sign(&bob_key(), &AccountId::from(BOB))
			));
			// transfer USDC erc20 from alith to ALICE/BOB, used for swap
			<EVM as EVMTrait<AccountId>>::set_origin(alith.clone());
			assert_ok!(Currencies::transfer(
				Origin::signed(alith.clone()),
				sp_runtime::MultiAddress::Id(AccountId::from(BOB)),
				usdc,
				10 * dollar,
			));
			assert_ok!(Currencies::transfer(
				Origin::signed(alith.clone()),
				sp_runtime::MultiAddress::Id(AccountId::from(ALICE)),
				usdc,
				10 * dollar,
			));
			assert_ok!(Currencies::transfer(
				Origin::signed(alith.clone()),
				sp_runtime::MultiAddress::Id(treasury_account.clone()),
				usdc,
				10 * dollar,
			));
			assert_eq!(Currencies::free_balance(usdc, &AccountId::from(BOB)), 10 * dollar);
			assert_eq!(Currencies::free_balance(usdc, &bob()), 10 * dollar);
			assert_eq!(Currencies::free_balance(usdc, &AccountId::from(ALICE)), 10 * dollar);
			assert_eq!(Currencies::free_balance(usdc, &alice()), 10 * dollar);

			// create three stable asset pool
			let three_usds = vec![
				usdt,         // PoolTokenIndex=0: USDT
				usdc,         // PoolTokenIndex=1: USDC
				USD_CURRENCY, // PoolTokenIndex=2: AUSD
			];
			enable_stable_asset(
				three_usds,
				vec![1000 * dollar, 1000 * dollar, 1000 * dollar],
				Some(alith.clone()),
			);
			System::assert_last_event(Event::StableAsset(nutsfinance_stable_asset::Event::Minted {
				minter: alith,
				pool_id: 0,
				a: 1000,
				input_amounts: vec![1000 * dollar, 1000 * dollar, 1000 * dollar],
				min_output_amount: 0,
				balances: vec![1000 * dollar, 1000 * dollar, 1000 * dollar],
				total_supply: 3000 * dollar,
				fee_amount: 3 * dollar,
				output_amount: 2997 * dollar,
			}));

			// inject liquidity of AUSD to native token. Notice: USDC/USDT to AUSD liquidity is provided by
			// stable-asset pool, not by dex.
			assert_ok!(inject_liquidity(
				USD_CURRENCY,
				NATIVE_CURRENCY,
				1000 * dollar,
				10000 * dollar
			));
			assert_eq!(
				Dex::get_liquidity_pool(USD_CURRENCY, NATIVE_CURRENCY),
				(1000 * dollar, 10000 * dollar)
			);
			// Taiga(USDT, AUSD), Dex(AUSD, ACA)
			assert_ok!(AggregatedDex::update_aggregated_swap_paths(
				Origin::root(),
				vec![(
					(CurrencyId::ForeignAsset(0), NATIVE_CURRENCY),
					Some(vec![
						SwapPath::Taiga(0, 0, 2),
						SwapPath::Dex(vec![USD_CURRENCY, NATIVE_CURRENCY])
					])
				),]
			));
			// Taiga(USDC, AUSD), Dex(AUSD, ACA)
			assert_ok!(AggregatedDex::update_aggregated_swap_paths(
				Origin::root(),
				vec![(
					(usdc, NATIVE_CURRENCY),
					Some(vec![
						SwapPath::Taiga(0, 1, 2),
						SwapPath::Dex(vec![USD_CURRENCY, NATIVE_CURRENCY])
					])
				),]
			));
			// AggregatedDex::swap works: USDC->AUSD->ACA, USDT->AUSD->ACA, AUSD->ACA
			let usd_tokens: Vec<CurrencyId> = vec![usdc, usdt, USD_CURRENCY];
			#[cfg(any(feature = "with-karura-runtime", feature = "with-acala-runtime"))]
			let swap_amounts: Vec<u128> = vec![9_940_060_348_765u128, 9_920_180_467_236u128, 9_920_507_587_087u128];
			#[cfg(feature = "with-mandala-runtime")]
			let swap_amounts: Vec<u128> = vec![9_959_980_429_142u128, 9_940_040_907_508u128, 9_940_348_860_887u128];
			for (token, swap_amount) in usd_tokens.iter().zip(swap_amounts.iter()) {
				assert_eq!(
					AcalaSwap::swap(
						&AccountId::from(BOB),
						*token,
						NATIVE_CURRENCY,
						SwapLimit::ExactSupply(dollar, 0)
					),
					Ok((dollar, *swap_amount))
				);
			}

			// USDC=Erc20(contract) or USDT=ForeignAsset(0) as fee token.
			// before USDC/USDT enabled as fee pool, it works by direct swap.
			payment_validate_with_fee_call_works(usdc, with_fee_currency_call(usdc), None);
			payment_validate_with_fee_call_works(usdt, with_fee_currency_call(usdt), None);

			// AUSD as fee token, only dex swap event produced.
			assert_ok!(
				<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
					&AccountId::from(BOB),
					&with_fee_currency_call(USD_CURRENCY),
					&INFO,
					50
				)
			);
			#[cfg(any(feature = "with-karura-runtime", feature = "with-acala-runtime"))]
			let (amount1, amount2) = (227029695u128, 2250001739u128);
			#[cfg(feature = "with-mandala-runtime")]
			let (amount1, amount2) = (906308684u128, 9000001739u128);
			System::assert_has_event(Event::Dex(module_dex::Event::Swap {
				trader: AccountId::from(BOB),
				path: vec![USD_CURRENCY, NATIVE_CURRENCY],
				liquidity_changes: vec![amount1, amount2],
			}));

			// with_fee_path_call failed
			let invalid_swap_path = vec![
				vec![usdt, USD_CURRENCY, NATIVE_CURRENCY],
				vec![usdt, USD_CURRENCY],
				vec![usdt, NATIVE_CURRENCY],
				vec![usdc, USD_CURRENCY, NATIVE_CURRENCY],
				vec![usdc, USD_CURRENCY],
				vec![usdc, NATIVE_CURRENCY],
			];
			for path in invalid_swap_path {
				assert_noop!(
					<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
						&AccountId::from(BOB),
						&with_fee_path_call(path),
						&INFO,
						50
					),
					TransactionValidityError::Invalid(InvalidTransaction::Payment)
				);
			}
			// USD_CURRENCY to NATIVE_CURRENCY is valid, because it exist in dex swap.
			assert_ok!(
				<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
					&AccountId::from(BOB),
					&with_fee_path_call(vec![USD_CURRENCY, NATIVE_CURRENCY]),
					&INFO,
					50
				)
			);

			// with_fee_aggregated_path_call also works by direct swap.
			let usdt_aggregated_path = vec![
				AggregatedSwapPath::<CurrencyId>::Taiga(0, 0, 2), // USDT, AUSD
				AggregatedSwapPath::<CurrencyId>::Dex(vec![USD_CURRENCY, NATIVE_CURRENCY]),
			];
			let usdc_aggregated_path = vec![
				AggregatedSwapPath::<CurrencyId>::Taiga(0, 1, 2), // USDC, AUSD
				AggregatedSwapPath::<CurrencyId>::Dex(vec![USD_CURRENCY, NATIVE_CURRENCY]),
			];
			let invalid_aggregated_path = vec![
				AggregatedSwapPath::<CurrencyId>::Taiga(0, 0, 1), // USDT, USDC
				AggregatedSwapPath::<CurrencyId>::Dex(vec![USD_CURRENCY, NATIVE_CURRENCY]),
			];
			assert_noop!(
				<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
					&AccountId::from(BOB),
					&with_fee_aggregated_path_call(invalid_aggregated_path),
					&INFO,
					50
				),
				TransactionValidityError::Invalid(InvalidTransaction::Payment)
			);
			payment_validate_with_fee_call_works(usdc, with_fee_aggregated_path_call(usdc_aggregated_path), None);
			payment_validate_with_fee_call_works(usdt, with_fee_aggregated_path_call(usdt_aggregated_path), None);

			// enable USDT/USDC as charge fee pool
			#[cfg(any(feature = "with-karura-runtime", feature = "with-acala-runtime"))]
			let len = 33300;
			#[cfg(feature = "with-mandala-runtime")]
			let len = 3330;
			let fee = module_transaction_payment::Pallet::<Runtime>::compute_fee(len, &INFO, 0);
			let surplus_perc = Percent::from_percent(50); // CustomFeeSurplus
			let fee_surplus = surplus_perc.mul_ceil(fee);
			let fee = fee + fee_surplus; // 501,000,001,739
			assert_ok!(TransactionPayment::enable_charge_fee_pool(
				Origin::root(),
				usdt,
				fee_pool_size,
				fee_pool_size - fee,
			));
			assert_ok!(TransactionPayment::enable_charge_fee_pool(
				Origin::root(),
				usdc,
				fee_pool_size,
				fee_pool_size - fee,
			));
			assert_eq!(
				fee_pool_size,
				Currencies::free_balance(NATIVE_CURRENCY, &usdt_sub_account)
			);
			assert_eq!(
				fee_pool_size,
				Currencies::free_balance(NATIVE_CURRENCY, &usdc_sub_account)
			);
			assert_eq!(usdt_ed, Currencies::free_balance(usdt, &usdt_sub_account));
			assert_eq!(usdc_ed, Currencies::free_balance(usdc, &usdc_sub_account));
			assert!(module_transaction_payment::Pallet::<Runtime>::token_exchange_rate(usdt).is_some());
			assert!(module_transaction_payment::Pallet::<Runtime>::token_exchange_rate(usdc).is_some());
			let rate = module_transaction_payment::Pallet::<Runtime>::token_exchange_rate(usdt).unwrap();
			let usd_fee_amount: u128 = rate.saturating_mul_int(fee);
			let usdt_amount = Currencies::free_balance(usdt, &AccountId::from(BOB));
			let usdc_amount = Currencies::free_balance(usdc, &AccountId::from(BOB));
			assert_ok!(
				<module_transaction_payment::ChargeTransactionPayment<Runtime>>::from(0).validate(
					&AccountId::from(BOB),
					&with_fee_currency_call(usdt),
					&INFO,
					len as usize,
				)
			);
			assert_ok!(
				<module_transaction_payment::ChargeTransactionPayment<Runtime>>::from(0).validate(
					&AccountId::from(BOB),
					&with_fee_currency_call(usdc),
					&INFO,
					len as usize,
				)
			);
			assert_eq!(
				usd_fee_amount,
				usdt_amount - Currencies::free_balance(usdt, &AccountId::from(BOB))
			);
			assert_eq!(
				usd_fee_amount,
				usdc_amount - Currencies::free_balance(usdc, &AccountId::from(BOB))
			);
			assert_eq!(
				fee,
				fee_pool_size - Currencies::free_balance(NATIVE_CURRENCY, &usdc_sub_account)
			);
			assert_eq!(
				fee,
				fee_pool_size - Currencies::free_balance(NATIVE_CURRENCY, &usdt_sub_account)
			);

			assert_ok!(
				<module_transaction_payment::ChargeTransactionPayment<Runtime>>::from(0).validate(
					&AccountId::from(BOB),
					&with_fee_currency_call(usdt),
					&INFO,
					len as usize,
				)
			);
			assert_ok!(
				<module_transaction_payment::ChargeTransactionPayment<Runtime>>::from(0).validate(
					&AccountId::from(BOB),
					&with_fee_currency_call(usdc),
					&INFO,
					len as usize,
				)
			);

			// when sub-account has not enough native token, trigger swap
			payment_validate_with_fee_call_works(usdt, with_fee_currency_call(usdt), Some(len as usize));
			payment_validate_with_fee_call_works(usdc, with_fee_currency_call(usdc), Some(len as usize));
		});
}

#[test]
fn usdc_works() {
	// env_logger::init();
	let dollar = dollar(NATIVE_CURRENCY);
	let alith = MockAddressMapping::get_account_id(&alice_evm_addr());
	ExtBuilder::default()
		.balances(vec![
			// alice() used to deploy erc20 contract
			(alice(), NATIVE_CURRENCY, 1_000_000 * dollar),
			(
				// NetworkContractSource
				MockAddressMapping::get_account_id(&H160::from_low_u64_be(0)),
				NATIVE_CURRENCY,
				1_000_000_000 * dollar,
			),
			// alith used to mint 3USD.
			(alith.clone(), NATIVE_CURRENCY, 1_000_000_000 * dollar),
			(alith.clone(), USD_CURRENCY, 1_000_000_000 * dollar),
			(AccountId::from(ALICE), USD_CURRENCY, 1_000_000_000 * dollar),
			(AccountId::from(ALICE), RELAY_CHAIN_CURRENCY, 1_000_000_000 * dollar),
			(AccountId::from(ALICE), NATIVE_CURRENCY, 1_000_000_000 * dollar),
			(AccountId::from(BOB), USD_CURRENCY, 1_000_000 * dollar),
			(AccountId::from(BOB), NATIVE_CURRENCY, 1_000_000 * dollar),
		])
		.build()
		.execute_with(|| {
			// THIS IS USDC AMOUNT TRANSFERED TO BOB, and used to swap operation.
			let transfer_usdc = 100_000_000; // USDC amount

			let usdt: CurrencyId = CurrencyId::ForeignAsset(0);
			// USDT is asset on Statemine
			assert_ok!(AssetRegistry::register_foreign_asset(
				Origin::root(),
				Box::new(
					MultiLocation::new(
						1,
						X2(
							Parachain(1000),
							GeneralKey("USDT".as_bytes().to_vec().try_into().unwrap())
						)
					)
					.into()
				),
				Box::new(AssetMetadata {
					name: b"USDT".to_vec(),
					symbol: b"USDT".to_vec(),
					decimals: 6,
					minimal_balance: 10_000
				})
			));
			// deposit USDT to alith, used for liquidity provider
			assert_ok!(Currencies::deposit(usdt, &alith, 1_000_000_000 * dollar));
			// deposit USDT to BOB, used for swap
			assert_ok!(Currencies::deposit(usdt, &AccountId::from(BOB), 1_000_000 * dollar));

			// USDC is Erc20 token, decimals=6
			deploy_usdc_contracts();

			let usdc: CurrencyId = CurrencyId::Erc20(erc20_address_0());
			let total_erc20 = 100_000_000_000_000_000_000_000u128;
			// alith has USDC when create Erc20 token
			assert_eq!(Currencies::free_balance(usdc, &alith), total_erc20);
			assert_eq!(Currencies::free_balance(usdt, &alith), 1_000_000_000 * dollar);

			assert_ok!(EvmAccounts::claim_account(
				Origin::signed(AccountId::from(BOB)),
				EvmAccounts::eth_address(&bob_key()),
				EvmAccounts::eth_sign(&bob_key(), &AccountId::from(BOB))
			));
			// transfer USDC erc20 from alith to ALICE/BOB, used for swap
			<EVM as EVMTrait<AccountId>>::set_origin(alith.clone());
			assert_ok!(Currencies::transfer(
				Origin::signed(alith.clone()),
				sp_runtime::MultiAddress::Id(AccountId::from(BOB)),
				usdc,
				transfer_usdc,
			));
			assert_eq!(Currencies::free_balance(usdc, &AccountId::from(BOB)), transfer_usdc);
			assert_eq!(Currencies::free_balance(usdc, &bob()), transfer_usdc);

			// create three stable asset pool
			// StableAsset::Pools
			enable_stable_asset2(
				vec![
					USD_CURRENCY, // 0: AUSD
					usdc,         // 1: USDC, decimals=6
					usdt,         // 2: USDT, decimals=6
				],
				vec![
					105_135_943_457_866_107_649u128,
					29_203_658_077_783u128,
					781_072_047_861_505u128,
				],
				Some(alith.clone()),
				vec![1u128, 1_000_000u128, 1_000_000u128],
			);

			// Dex::LiquidityPool of KUSD-KSM and KAR-KSM
			assert_ok!(inject_liquidity2(
				AccountId::from(ALICE),
				USD_CURRENCY,
				RELAY_CHAIN_CURRENCY,
				2_108_218_744_012_282_700u128,
				9_154_402_197_674_471u128
			));
			assert_ok!(inject_liquidity2(
				AccountId::from(ALICE),
				NATIVE_CURRENCY,
				RELAY_CHAIN_CURRENCY,
				5_120_901_048_663_808u128,
				198_590_513_699_867u128,
			));

			// Taiga(USDC, AUSD), Dex(AUSD, ACA)
			let aggregated_path = vec![
				SwapPath::Taiga(0, 1, 0), // USDC -> AUSD
				SwapPath::Dex(vec![USD_CURRENCY, RELAY_CHAIN_CURRENCY, NATIVE_CURRENCY]),
			];
			assert_ok!(AggregatedDex::update_aggregated_swap_paths(
				Origin::root(),
				vec![((usdc, NATIVE_CURRENCY), Some(aggregated_path.clone()))]
			));

			// direct aggregated dex swap works
			aggregate_swap_test(usdc);

			// charge transaction payment using USDC as fee token
			// 15925 USDC --> 2264110407 KAR
			payment_validate_with_fee_call_works(usdc, with_fee_currency_call(usdc), None);
			// 15925 USDC --> 2264108376 KAR
			payment_validate_with_fee_call_works(usdc, with_fee_aggregated_path_call(aggregated_path), None);

			pre_post_dispatch_with_fee_call_works(usdc);
		});
}

fn aggregate_swap_test(usdc: CurrencyId) {
	System::reset_events();
	println!("###########################################################1");
	println!(
		"{:?}",
		AcalaSwap::swap(
			&AccountId::from(BOB),
			usdc,
			NATIVE_CURRENCY,
			SwapLimit::ExactSupply(1_000_000, 0)
		)
	);
	// Ok((1_000_000, 142_177_564_621)) ==> 1 USDC - 0.142 KAR
	for ev in System::events() {
		if matches!(
			ev.event,
			Event::StableAsset(nutsfinance_stable_asset::Event::TokenSwapped { .. })
				| Event::Dex(module_dex::Event::Swap { .. })
		) {
			println!("  🌛 ExactSupply>>{:?}", ev);
		}
	}

	System::reset_events();
	println!("###########################################################2");
	println!(
		"{:?}",
		AcalaSwap::swap(
			&AccountId::from(BOB),
			usdc,
			NATIVE_CURRENCY,
			SwapLimit::ExactTarget(Balance::MAX, 2_000_000_000)
		)
	);
	for ev in System::events() {
		if matches!(
			ev.event,
			Event::StableAsset(nutsfinance_stable_asset::Event::TokenSwapped { .. })
				| Event::Dex(module_dex::Event::Swap { .. })
		) {
			println!("  🌏 ExactTarget>>{:?}", ev);
		}
	}
	println!("###########################################################3");
}

fn payment_validate_with_fee_call_works(
	_usd_token: CurrencyId,
	with_fee_call: <Runtime as module_transaction_payment::Config>::Call,
	len: Option<usize>,
) {
	System::reset_events();
	assert_ok!(
		<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::from(0).validate(
			&AccountId::from(BOB),
			&with_fee_call,
			&INFO,
			len.unwrap_or(50)
		)
	);
	for ev in System::events() {
		if matches!(
			ev.event,
			Event::StableAsset(nutsfinance_stable_asset::Event::TokenSwapped { .. })
				| Event::Dex(module_dex::Event::Swap { .. })
		) {
			println!("  🔥 {:?}", ev);
		}
	}
}

fn pre_post_dispatch_with_fee_call_works(usdc: CurrencyId) {
	println!("###########################################################4");
	// pre post dispatch
	System::reset_events();
	// 15925 USDC --> 2264106320 KAR
	let pre = <module_transaction_payment::ChargeTransactionPayment<Runtime>>::from(0)
		.pre_dispatch(&AccountId::from(BOB), &with_fee_currency_call(usdc), &INFO, 50)
		.unwrap();
	// println!("fee:{:?}", pre.3); // 2250001739
	// println!("sur:{:?}", pre.4); // 750000580
	// let fee = module_transaction_payment::Pallet::<Runtime>::compute_fee(50, &INFO, 0);
	// println!("FEE:{:?}", fee); // 1500001159, fee + pre.4 = pre.3
	assert_ok!(
		<module_transaction_payment::ChargeTransactionPayment::<Runtime>>::post_dispatch(
			Some(pre),
			&INFO,
			&POST_INFO,
			50,
			&Ok(())
		)
	);
	for ev in System::events() {
		if matches!(
			ev.event,
			Event::StableAsset(nutsfinance_stable_asset::Event::TokenSwapped { .. })
				| Event::Dex(module_dex::Event::Swap { .. })
		) {
			println!("  🚀 {:?}", ev);
		}
	}
}

pub fn deploy_erc20_contracts() {
	let json: serde_json::Value =
		serde_json::from_str(include_str!("../../../ts-tests/build/Erc20DemoContract2.json")).unwrap();
	let code = hex::decode(json.get("bytecode").unwrap().as_str().unwrap()).unwrap();

	assert_ok!(EVM::create(Origin::signed(alice()), code, 0, 2100_000, 100000, vec![]));
	assert_ok!(EVM::publish_free(Origin::root(), erc20_address_0()));
	assert_ok!(AssetRegistry::register_erc20_asset(
		Origin::root(),
		erc20_address_0(),
		100_000_000_000
	));
}

pub fn deploy_usdc_contracts() {
	let json: serde_json::Value =
		serde_json::from_str(include_str!("../../../ts-tests/build/Erc20DemoContract1.json")).unwrap();
	let code = hex::decode(json.get("bytecode").unwrap().as_str().unwrap()).unwrap();

	assert_ok!(EVM::create(Origin::signed(alice()), code, 0, 2100_000, 100000, vec![]));
	assert_ok!(EVM::publish_free(Origin::root(), erc20_address_0()));
	assert_ok!(AssetRegistry::register_erc20_asset(
		Origin::root(),
		erc20_address_0(),
		10_000
	));
}

pub fn erc20_address_0() -> EvmAddress {
	EvmAddress::from_str("0x5e0b4bfa0b55932a3587e648c3552a6515ba56b1").unwrap()
}

fn inject_liquidity(
	currency_id_a: CurrencyId,
	currency_id_b: CurrencyId,
	max_amount_a: Balance,
	max_amount_b: Balance,
) -> Result<(), &'static str> {
	let alith = MockAddressMapping::get_account_id(&alice_evm_addr());
	let _ = Dex::enable_trading_pair(Origin::root(), currency_id_a, currency_id_b);
	Dex::add_liquidity(
		Origin::signed(alith),
		currency_id_a,
		currency_id_b,
		max_amount_a,
		max_amount_b,
		Default::default(),
		false,
	)?;
	Ok(())
}

fn inject_liquidity2(
	account: AccountId,
	currency_id_a: CurrencyId,
	currency_id_b: CurrencyId,
	max_amount_a: Balance,
	max_amount_b: Balance,
) -> Result<(), &'static str> {
	let _ = Dex::enable_trading_pair(Origin::root(), currency_id_a, currency_id_b);
	Dex::add_liquidity(
		Origin::signed(account),
		currency_id_a,
		currency_id_b,
		max_amount_a,
		max_amount_b,
		Default::default(),
		false,
	)?;
	Ok(())
}

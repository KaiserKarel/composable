use super::*;

#[allow(unused)]
use crate::Pallet as Lending;
use composable_traits::{lending::MarketConfigInput, rate_model::NormalizedCollateralFactor};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	assert_ok,
	traits::{Currency, EnsureOrigin, Get, fungibles::Mutate},
};
use frame_system::{EventRecord, RawOrigin};
use sp_runtime::{DispatchResult, FixedPointNumber, Percent, Perquintill, SaturatedConversion};
use sp_std::prelude::*;

// pub type BalanceOf<T> =
//     <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

macro_rules! whitelist {
	($acc:ident) => {
		frame_benchmarking::benchmarking::add_to_whitelist(
			frame_system::Account::<T>::hashed_key_for(&$acc).into(),
		);
	};
}

type AccountId = u32;
type BorrowAssetVault = VaultId;
type CurrencyId = u64;
type VaultId = u64;

pub const ALICE: AccountId = 0;
const BTC: u64 = 1000;
const USDT: u64 = 2000;

/*
fn create_market<T: Config<AssetId = u64>>(
	borrow_asset: u64,
	collateral_asset: u64,
	manager: T::AccountId,
	reserved: Perquintill,
	collateral_factor: NormalizedCollateralFactor,
) -> (MarketIndex, T::VaultId) {
	let market_config = MarketConfigInput { manager, reserved, collateral_factor };
	let market = Lending::<T>::create(
        borrow_asset,
        collateral_asset,
        market_config
    );
	assert_ok!(market);
	market.expect("unreachable; qed;")
}

fn create_simple_market<T: Config<AccountId = u32, AssetId = u64>>() -> (MarketIndex, T::VaultId) {
	create_market::<T>(
        BTC,
        USDT,
		ALICE,
		Perquintill::from_percent(10),
		NormalizedCollateralFactor::saturating_from_rational(200, 100),
	)
}

fn asset_id<T: Config>(id: u64) -> T::AssetId {
    unsafe {
        sp_std::mem::transmute::<u64, T::AssetId>(id)
    }
}
*/

benchmarks! {
    deposit_collateral {
		let caller: T::AccountId = whitelisted_caller();
        let market: MarketIndex = MarketIndex::new(1u32);
        let amount: T::Balance = 1_000_000u64.into();
        let usdt: T::AssetId = T::AssetId::default();
        T::Currency::mint_into(usdt, &caller, amount);
    }: _(RawOrigin::Signed(caller.clone()), market, amount)
    verify {
        assert_last_event::<T>(Event::CollateralDeposited(caller, market, amount).into())
    }

    withdraw_collateral {
		let caller: T::AccountId = whitelisted_caller();
        let market: MarketIndex = MarketIndex::new(1u32);
        let amount: T::Balance = 1_000_000u64.into();
        let usdt: T::AssetId = T::AssetId::default();
        T::Currency::mint_into(usdt, &caller, amount);
    }: _(RawOrigin::Signed(caller.clone()), market, amount)
    verify {
        assert_last_event::<T>(Event::CollateralWithdrawed(caller, market, amount).into())
    }

    accrue_interests {
        // let m in 1 .. u32::MAX; // values of MarketIndex
        let block = 10u32.into();
    }:  {
        Lending::<T>::accrue_interests(block)
    }
}


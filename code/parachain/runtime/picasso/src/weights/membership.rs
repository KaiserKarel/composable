
//! Autogenerated weights for `membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-25, STEPS: `2`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `dev`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/gpsh9wvfcrwyck2nw62gpkqhf0bhc0cw-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=2
// --repeat=2
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> membership::WeightInfo for WeightInfo<T> {
	// Storage: CouncilMembership Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 99]`.
	fn add_member(m: u32, ) -> Weight {
		// Minimum execution time: 24_334 nanoseconds.
		Weight::from_ref_time(25_525_204 as u64)
			// Standard Error: 30_915
			.saturating_add(Weight::from_ref_time(79_295 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CouncilMembership Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: CouncilMembership Prime (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[2, 100]`.
	fn remove_member(m: u32, ) -> Weight {
		// Minimum execution time: 27_250 nanoseconds.
		Weight::from_ref_time(28_930_867 as u64)
			// Standard Error: 26_470
			.saturating_add(Weight::from_ref_time(45_066 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CouncilMembership Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: CouncilMembership Prime (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[2, 100]`.
	fn swap_member(m: u32, ) -> Weight {
		// Minimum execution time: 26_834 nanoseconds.
		Weight::from_ref_time(27_605_938 as u64)
			// Standard Error: 11_217
			.saturating_add(Weight::from_ref_time(103_530 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CouncilMembership Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: CouncilMembership Prime (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn reset_member(m: u32, ) -> Weight {
		// Minimum execution time: 26_542 nanoseconds.
		Weight::from_ref_time(28_288_005 as u64)
			// Standard Error: 58_306
			.saturating_add(Weight::from_ref_time(191_494 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CouncilMembership Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: CouncilMembership Prime (r:1 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn change_key(m: u32, ) -> Weight {
		// Minimum execution time: 27_584 nanoseconds.
		Weight::from_ref_time(28_989_555 as u64)
			// Standard Error: 18_516
			.saturating_add(Weight::from_ref_time(73_444 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: CouncilMembership Members (r:1 w:0)
	// Storage: CouncilMembership Prime (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn set_prime(m: u32, ) -> Weight {
		// Minimum execution time: 11_667 nanoseconds.
		Weight::from_ref_time(12_797_646 as u64)
			// Standard Error: 14_071
			.saturating_add(Weight::from_ref_time(15_353 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: CouncilMembership Prime (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn clear_prime(m: u32, ) -> Weight {
		// Minimum execution time: 7_125 nanoseconds.
		Weight::from_ref_time(6_905_090 as u64)
			// Standard Error: 528_847
			.saturating_add(Weight::from_ref_time(532_409 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

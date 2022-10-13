#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::pallet_prelude::PhantomData;
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for pallet_nft.
pub trait WeightInfo {
	fn create_nft_class() -> Weight;
	fn mint_ipfs_nft() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_nft_class() -> Weight {
		(38_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn mint_ipfs_nft() -> Weight {
		(56_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}


// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_nft_class() -> Weight {
		(38_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn mint_ipfs_nft() -> Weight {
		(56_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
}
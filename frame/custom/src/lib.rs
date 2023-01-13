// SPDX-License-Identifier: Apache-2.0
// This file is part of Frontier.
//
// Copyright (c) 2021-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::comparison_chain)]

pub use pallet::*;
use sp_core::{Get, U256};

#[frame_support::pallet]
pub mod pallet {

	use frame_support::pallet_prelude::{StorageValue, ValueQuery};
	use frame_support::traits::Hooks;
	use sp_core::{Get, U256};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type InitialBlockGasLimit: Get<U256>;
	}

	#[pallet::storage]
	pub(super) type CurrentBlockGasLimit<T: Config> =
		StorageValue<_, U256, ValueQuery, T::InitialBlockGasLimit>;

	#[pallet::hooks]
	impl<T: Config> Hooks<frame_system::pallet_prelude::BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_n: T::BlockNumber) {
			<CurrentBlockGasLimit<T>>::mutate(|x| *x = x.saturating_add(U256::from(1_000_000)))
		}
	}
}

impl<T: Config> Get<U256> for Pallet<T> {
	fn get() -> U256 {
		<CurrentBlockGasLimit<T>>::get()
	}
}

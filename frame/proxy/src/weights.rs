

// Copyright (C) 2021 MIT open source license.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE peer BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-08-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_proxy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/proxy/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_proxy.
pub trait WeightInfo {
	fn proxy(p: u32, ) -> Weight;
	fn proxy_announced(a: u32, p: u32, ) -> Weight;
	fn remove_announcement(a: u32, p: u32, ) -> Weight;
	fn reject_announcement(a: u32, p: u32, ) -> Weight;
	fn announce(a: u32, p: u32, ) -> Weight;
	fn add_proxy(p: u32, ) -> Weight;
	fn remove_proxy(p: u32, ) -> Weight;
	fn remove_proxies(p: u32, ) -> Weight;
	fn anonymous(p: u32, ) -> Weight;
	fn kill_anonymous(p: u32, ) -> Weight;
}

/// Weights for pallet_proxy using the peer node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	fn proxy(p: u32, ) -> Weight {
		(23_213_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((153_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		(53_286_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((549_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((138_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		(36_864_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((550_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reject_announcement(a: u32, _p: u32, ) -> Weight {
		(36_755_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((550_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn announce(a: u32, p: u32, ) -> Weight {
		(50_765_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((547_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((141_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn add_proxy(p: u32, ) -> Weight {
		(35_556_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((211_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxy(p: u32, ) -> Weight {
		(35_284_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((229_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxies(p: u32, ) -> Weight {
		(34_449_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((146_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	fn anonymous(p: u32, ) -> Weight {
		(49_149_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((15_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn kill_anonymous(p: u32, ) -> Weight {
		(36_399_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((152_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Proxy Proxies (r:1 w:0)
	fn proxy(p: u32, ) -> Weight {
		(23_213_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((153_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		(53_286_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((549_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((138_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		(36_864_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((550_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn reject_announcement(a: u32, _p: u32, ) -> Weight {
		(36_755_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((550_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn announce(a: u32, p: u32, ) -> Weight {
		(50_765_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((547_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((141_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn add_proxy(p: u32, ) -> Weight {
		(35_556_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((211_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxy(p: u32, ) -> Weight {
		(35_284_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((229_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxies(p: u32, ) -> Weight {
		(34_449_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((146_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	fn anonymous(p: u32, ) -> Weight {
		(49_149_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((15_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn kill_anonymous(p: u32, ) -> Weight {
		(36_399_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((152_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}

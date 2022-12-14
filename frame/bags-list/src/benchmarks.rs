

// Copyright (C) 2021 MIT open source license.
// SPDX-License-Identifier: Apache-2.0

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

//! Benchmarks for the bags list pallet.

use super::*;
use crate::list::List;
use frame_benchmarking::{account, whitelisted_caller};
use frame_election_provider_support::VoteWeightProvider;
use frame_support::{assert_ok, traits::Get};
use frame_system::RawOrigin as SystemOrigin;

frame_benchmarking::benchmarks! {
	rebag_non_terminal {
		// An expensive case for rebag-ing (rebag a non-terminal node):
		//
		// - The node to be rebagged, _R_, should exist as a non-terminal node in a bag with at
		//   least 2 other nodes. Thus _R_ will have both its `prev` and `next` nodes updated when
		//   it is removed. (3 W/R)
		// - The destination bag is not empty, thus we need to update the `next` pointer of the last
		//   node in the destination in addition to the work we do otherwise. (2 W/R)

		// clear any pre-existing storage.
		// NOTE: safe to call outside block production
		List::<T>::unsafe_clear();

		// define our origin and destination thresholds.
		let origin_bag_thresh = T::BagThresholds::get()[0];
		let dest_bag_thresh = T::BagThresholds::get()[1];

		// seed items in the origin bag.
		let origin_head: T::AccountId = account("origin_head", 0, 0);
		assert_ok!(List::<T>::insert(origin_head.clone(), origin_bag_thresh));

		let origin_middle: T::AccountId = account("origin_middle", 0, 0); // the node we rebag (_R_)
		assert_ok!(List::<T>::insert(origin_middle.clone(), origin_bag_thresh));

		let origin_tail: T::AccountId  = account("origin_tail", 0, 0);
		assert_ok!(List::<T>::insert(origin_tail.clone(), origin_bag_thresh));

		// seed items in the destination bag.
		let dest_head: T::AccountId  = account("dest_head", 0, 0);
		assert_ok!(List::<T>::insert(dest_head.clone(), dest_bag_thresh));

		// the bags are in the expected state after initial setup.
		assert_eq!(
			List::<T>::get_bags(),
			vec![
				(origin_bag_thresh, vec![origin_head.clone(), origin_middle.clone(), origin_tail.clone()]),
				(dest_bag_thresh, vec![dest_head.clone()])
			]
		);

		let caller = whitelisted_caller();
		// update the weight of `origin_middle` to guarantee it will be rebagged into the destination.
		T::VoteWeightProvider::set_vote_weight_of(&origin_middle, dest_bag_thresh);
	}: rebag(SystemOrigin::Signed(caller), origin_middle.clone())
	verify {
		// check the bags have updated as expected.
		assert_eq!(
			List::<T>::get_bags(),
			vec![
				(
					origin_bag_thresh,
					vec![origin_head, origin_tail],
				),
				(
					dest_bag_thresh,
					vec![dest_head, origin_middle],
				)
			]
		);
	}

	rebag_terminal {
		// An expensive case for rebag-ing (rebag a terminal node):
		//
		// - The node to be rebagged, _R_, is a terminal node; so _R_, the node pointing to _R_ and
		//   the origin bag itself will need to be updated. (3 W/R)
		// - The destination bag is not empty, thus we need to update the `next` pointer of the last
		//   node in the destination in addition to the work we do otherwise. (2 W/R)

		// clear any pre-existing storage.
		// NOTE: safe to call outside block production
		List::<T>::unsafe_clear();

		// define our origin and destination thresholds.
		let origin_bag_thresh = T::BagThresholds::get()[0];
		let dest_bag_thresh = T::BagThresholds::get()[1];

		// seed items in the origin bag.
		let origin_head: T::AccountId = account("origin_head", 0, 0);
		assert_ok!(List::<T>::insert(origin_head.clone(), origin_bag_thresh));

		let origin_tail: T::AccountId  = account("origin_tail", 0, 0); // the node we rebag (_R_)
		assert_ok!(List::<T>::insert(origin_tail.clone(), origin_bag_thresh));

		// seed items in the destination bag.
		let dest_head: T::AccountId  = account("dest_head", 0, 0);
		assert_ok!(List::<T>::insert(dest_head.clone(), dest_bag_thresh));

		// the bags are in the expected state after initial setup.
		assert_eq!(
			List::<T>::get_bags(),
			vec![
				(origin_bag_thresh, vec![origin_head.clone(), origin_tail.clone()]),
				(dest_bag_thresh, vec![dest_head.clone()])
			]
		);

		let caller = whitelisted_caller();
		// update the weight of `origin_tail` to guarantee it will be rebagged into the destination.
		T::VoteWeightProvider::set_vote_weight_of(&origin_tail, dest_bag_thresh);
	}: rebag(SystemOrigin::Signed(caller), origin_tail.clone())
	verify {
		// check the bags have updated as expected.
		assert_eq!(
			List::<T>::get_bags(),
			vec![
				(origin_bag_thresh, vec![origin_head.clone()]),
				(dest_bag_thresh, vec![dest_head.clone(), origin_tail.clone()])
			]
		);
	}

	impl_benchmark_test_suite!(
		Pallet,
		crate::mock::ExtBuilder::default().build(),
		crate::mock::Runtime,
	)
}

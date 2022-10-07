#![cfg_attr(not(feature = "std"),no_std)]

pub use pallet::*;

// use codec::FullCodec;
use codec::{Decode, Encode, MaxEncodedLen};

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;


use core::default;

use super::*;
	use frame_support::{
		// pallet_prelude::*, 
		// traits::KeyOwnerProofSystem, 
		pallet_prelude::ValueQuery,
		traits::GetDefault};
		
	use frame_support::inherent::Vec;
use scale_info::form::MetaForm;
use sp_runtime::AccountId32;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	//    #[pallet::generate_storage_info]
	
	pub struct Pallet<T>(_);
		
	// pub struct NFTS {
	// 	id: u32,
	// 	meta_data: Vec<u8>,
	// 	owner: T::AccountId,
	// }

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + 
		IsType<<Self as frame_system::Config>::Event>;
		// type KeyOwnerProof: Parameter;
		// type ProtocolOrigin: EnsureOrigin<Self::Origin>;
	

		type MetaData: Member
			+ Parameter
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;
	}
			


	#[pallet::storage]
	#[pallet::getter(fn total)]
	pub type Total<T> = StorageValue<_, u128>;


	#[pallet::storage]
	#[pallet::getter(fn abcd)]
	pub type Nfts<T> = StorageMap<_,
	Blake2_128Concat,
	u32,
	Vec<u8>,
	OptionQuery,
	>;	


	#[pallet::storage]
	#[pallet::getter(fn abc)]
	pub(crate) type NFTs<T:Config> = StorageMap<_, 
	Blake2_128Concat, 
	u32, 
	T::AccountId,
	ValueQuery,
	>;

	// #[pallet::storage]
	// /// The number of units of assets held by any given account.
	// pub(super) type Account<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
	// 	_,
	// 	Blake2_128Concat,
	// 	T::AssetId,
	// 	Blake2_128Concat,
	// 	T::AccountId,
	// 	AssetBalance<T::Balance, T::Extra>,
	// 	ValueQuery,
	// 	GetDefault,
	// 	ConstU32<300_000>,
	// >;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NftMinted(u32, Vec<u8> , T::AccountId),
		// Transferred{id: u32, MetaData: Vec<u8>, dest: T::AccountId},
		Transferred{id: u32, dest: T::AccountId},
		Burned(u32, Vec<u8>, T::AccountId),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {

		NoneValue,
		StorageOverflow,
		NftAlreadyExists,
		NftDoesNotExist,
		NotOwner,

	}



	#[pallet::call]
		impl<T: Config> Pallet<T> {
	 	#[pallet::weight(10_000 + 
			T::DbWeight::get().writes(1))]
	 	pub fn mint(
			origin: OriginFor<T>, 
			nft_id: u32, 
			meta_data: Vec<u8>) ->
			 DispatchResult {
			
			let signer = 
			ensure_signed(origin)?;
			
			ensure!(!Self::existing_nft(
				nft_id.clone()), 
			Error::<T>::NftAlreadyExists);
			

				NFTs::<T>::insert(
					nft_id, 
					signer.clone()
				);
				Nfts::<T>::insert(
					nft_id, 
					meta_data.clone()
				);
			
			Self::deposit_event(Event::NftMinted(
				nft_id.clone(), 
				meta_data.clone(),
				signer.clone()));
			
				Ok(())
		}

	
	 	#[pallet::weight(10_000 + 
			T::DbWeight::get().writes(1))]
	 	pub fn transfer(
			origin: OriginFor<T>, 
			nft_id: u32, 
			//meta_data: Vec<u8>, 
			new_owner: T::AccountId,) -> 
			DispatchResult {
			let signer = 
			ensure_signed(origin)?;
			
		 ensure!(!Self::non_existing_nft(
			nft_id.clone()),
		 Error::<T>::NftDoesNotExist);
			
		 ensure!(!Self::owner_account(
			nft_id.clone(), 
			signer),
		 Error::<T>::NotOwner);
		// let who = T::Lookup::lookup(who)?;
		// ensure!(Account::<T, I>::contains_key(id, &who), Error::<T, I>::BalanceZero);

			   
			   NFTs::<T>::remove(nft_id.clone());
			   Nfts::<T>::remove(nft_id.clone());


			   NFTs::<T>::insert(
				nft_id, 
				new_owner.clone()
			);


				// Nfts::<T>::insert(
				// 	nft_id, 
				// 	// meta_data.clone()
				// );

			   Self::deposit_event(Event::Transferred {
				id: nft_id.clone(), 
				// MetaData: meta_data.clone(),
				dest: new_owner.clone()});
			
				Ok(())
   
			}
		
		
		   
		

	   #[pallet::weight(10_000 + 
		T::DbWeight::get().writes(1))]
	 	pub fn burn(
			origin: OriginFor<T>, 
			nft_id: u32, 
			meta_data: Vec<u8> ) -> 
			DispatchResult {
			let signer = 
			ensure_signed(origin)?;

			ensure!(!Self::non_existing_nft(
				nft_id.clone()), 
			Error::<T>::NftDoesNotExist);

			ensure!(!Self::owner_account(
				nft_id.clone(), signer.clone()),
			Error::<T>::NotOwner);

			NFTs::<T>::remove(nft_id);
			NFTs::<T>::remove(nft_id);

			Self::deposit_event(
				Event::Burned(
				nft_id.clone(), 
				meta_data.clone(),
				signer));
			
				Ok(())
		
		
		}
	}         

		


	

		impl<T: Config> Pallet<T> {
		pub fn existing_nft(item: u32) -> 
		bool {
			let item_nft_id = item;
			// let item_nft_meta_data = item_data;
					Nfts::<T>::get(
						item_nft_id).
				is_some()
		}
	
		pub fn non_existing_nft(
			item: u32) -> 
			bool {
			let item_nft_id = item;
			Nfts::<T>::get(
				item_nft_id).
				is_none()

		}

		pub fn owner_account
		(
			// origin: OriginFor<T>, 
			item: u32, is_owner: T::AccountId) -> 
			bool {
			// let	who =ensure_signed(origin)?; 
			let item_nft_id = item;
			 let item_nft_owner= is_owner;
			let abc = NFTs::<T>::get(
				item_nft_id) != item_nft_owner;
				return abc;
			}
		}
	
	}

 

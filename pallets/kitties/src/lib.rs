#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// use parity_scale_codec::MaxEncodedLen;
	use frame_support::{pallet_prelude::*, sp_runtime::traits::Hash, traits::Randomness};
	use frame_system::pallet_prelude::*;
	use pallet_balances;
	use sp_core::H256;

	/// #[pallet::pallet] is a mandatory pallet attribute
	/// that enables you to define a structure (struct)
	/// for the pallet so it can store data that can be easily retrieved.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config + TypeInfo {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// Specify the type for Randomness we want to specify for runtime.
		type KittyRandomness: Randomness<H256, u32>;

		// Specify the hashing algorithm used
		type Hashing: Hash<Output = Self::Hash> + TypeInfo;
	}

	// Struct for holding Kitty information.
	#[derive(Encode, Decode, Default, PartialEq, MaxEncodedLen, TypeInfo)]
	pub struct Kitty<Hash, Balance> {
		id: Hash,
		dna: Hash,
		price: Balance,
		gender: Gender,
	}

	// Enum declaration for Gender.
	#[derive(Encode, Decode, Debug, Clone, PartialEq, MaxEncodedLen, TypeInfo)]
	pub enum Gender {
		Male,
		Female,
	}

	// Implementation to handle Gender type in Kitty struct.
	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}

	// Nonce storage item.
	#[pallet::storage]
	#[pallet::getter(fn get_nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	// The pallet's runtime storage items for holding Kitty information.
	#[pallet::storage]
	#[pallet::getter(fn get_kitties_count)]
	pub(super) type AllKittiesCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_kitty)]
	pub(super) type Kitties<T: Config> = StorageMap<
		_,                          // prefix
		Twox64Concat,               // hasher
		T::Hash,                    // key
		Kitty<T::Hash, T::Balance>, // value
		ValueQuery,
	>;

	// TODO Part IV: Our pallet's genesis configuration.

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	// ACTION: Storage item to keep track of all Kitties.

	// TODO Part II: Remaining storage items.

	// TODO Part III: Our pallet's genesis configuration.

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO Part III: create_kitty

		// TODO Part III: set_price

		// TODO Part III: transfer

		// TODO Part III: buy_kitty

		// TODO Part III: breed_kitty
	}

	// helper function for Kitty struct
	impl<T: Config> Kitty<T, T> {
		pub fn gender(dna: T::Hash) -> Gender {
			if dna.as_ref()[0] % 2 == 0 {
				Gender::Male
			} else {
				Gender::Female
			}
		}
	}

	impl<T: Config> Pallet<T> {
		// TODO Part III: helper functions for dispatchable functions

		// increment_nonce helper
		fn increment_nonce() -> DispatchResult {
			<Nonce<T>>::try_mutate(|nonce| {
				let next = nonce.checked_add(1).ok_or("Overflow")?; // TODO Part III: Add error handling
				*nonce = next;

				Ok(())
			})
		}

		// random_hash helper
		fn random_hash(sender: &T::AccountId) -> T::Hash {
			let nonce = <Nonce<T>>::get();
			let seed = T::KittyRandomness::random_seed();

			<T as Config>::Hashing::hash_of(&(seed, &sender, nonce))
		}

		// TODO: mint, transfer_from
	}
}

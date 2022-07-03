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

		/// The maximum amount of Kitties a single account can own.
		#[pallet::constant]
		type MaxKittyOwned: Get<u32>;
	}

	// TODO Part IV: Our pallet's genesis configuration.

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new Kitty was successfully created. \[sender, kitty_id\]
		Created(T::AccountId, T::Hash),
		/// Kitty price was successfully set. \[sender, kitty_id, new_price\]
		PriceSet(T::AccountId, T::Hash, T::Balance),
		/// A Kitty was successfully transferred. \[from, to, kitty_id\]
		Transferred(T::AccountId, T::AccountId, T::Hash),
		/// A Kitty was successfully bought. \[buyer, seller, kitty_id, bid_price\]
		Bought(T::AccountId, T::AccountId, T::Hash, T::Balance),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	// Nonce storage item.
	#[pallet::storage]
	#[pallet::getter(fn get_nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	// Owns Kitties data structure by kitty_id
	#[pallet::storage]
	pub(super) type Kitties<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, Kitty<T::Hash, T::Balance>, ValueQuery>;

	// Keeps track of what accounts own what Kitty.
	#[pallet::storage]
	pub(super) type KittiesByOwner<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<T::Hash, T::MaxKittyOwned>,
		ValueQuery,
	>;

	// Stores the total amount of Kitties in existence.
	#[pallet::storage]
	pub(super) type KittiesCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	// TODO Part II: Remaining storage items.

	// TODO Part III: Our pallet's genesis configuration.

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100)]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let random_hash = Self::random_hash(&sender);

			let new_kitty = Kitty {
				id: random_hash,
				dna: random_hash,
				price: 0u8.into(), // number 0 as an unsigned 8-bit integer
				gender: Kitty::<T, T>::gender(random_hash),
			};

			Self::mint(sender, random_hash, new_kitty)?;
			Self::increment_nonce()?;

			Ok(().into())
		}

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

	// helper functions for dispatchable functions
	impl<T: Config> Pallet<T> {
		fn increment_nonce() -> DispatchResult {
			<Nonce<T>>::try_mutate(|nonce| {
				let next = nonce.checked_add(1).ok_or("Overflow")?; // TODO Part III: Add error handling
				*nonce = next;

				Ok(())
			})
		}

		fn random_hash(sender: &T::AccountId) -> T::Hash {
			let nonce = <Nonce<T>>::get();
			let seed = T::KittyRandomness::random_seed();

			<T as Config>::Hashing::hash_of(&(seed, &sender, nonce))
		}

		fn mint(
			to: T::AccountId,
			kitty_id: T::Hash,
			new_kitty: Kitty<T::Hash, T::Balance>,
		) -> DispatchResult {
			ensure!(!<Kitties<T>>::contains_key(kitty_id), "Kitty id already exists");

			// Update total Kitty counts.
			let all_kitties_count = <KittiesCount<T>>::get();
			let new_all_kitties_count = all_kitties_count
				.checked_add(1)
				.ok_or("Overflow adding a new kitty to total supply")?;

			// Performs this operation first because as it may fail
			<KittiesByOwner<T>>::try_mutate(&to, |kitty_vec| kitty_vec.try_push(kitty_id))
				.expect("MaxKittyOwned reached");

			// Update storage with new Kitty.
			<Kitties<T>>::insert(kitty_id, new_kitty);

			// Write Kitty counting information to storage.
			<KittiesCount<T>>::put(new_all_kitties_count);

			Self::deposit_event(Event::Created(to, kitty_id));

			Ok(().into())
		}
	}
}

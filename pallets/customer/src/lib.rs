#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::inherent::Vec;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	
	#[pallet::storage]
	pub type CustomerID<T> = StorageValue<_, u64, ValueQuery>;
	#[pallet::storage]
	pub type CustomerName<T> = StorageValue<_, Vec<u8>, ValueQuery>;
	#[pallet::storage]
	pub type CustomerMobileNo<T> = StorageValue<_, Vec<u8>, ValueQuery>;
	#[pallet::storage]
	pub type BookName<T> = StorageValue<_, Vec<u8>, ValueQuery>;
	#[pallet::storage]
	pub type BookAuthor<T> = StorageValue<_, Vec<u8>, ValueQuery>;
	#[pallet::storage]
	pub type BookCopies<T> = StorageValue<_, u64, ValueQuery>;
	#[pallet::storage]
	pub type BookPrice<T> = StorageValue<_, u64, ValueQuery>;

	
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		
		DetailStored(T::AccountId, u64, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, u64, u64),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn customer_details(origin: OriginFor<T>, C_Id: u64, C_Name: Vec<u8>, C_MobileNo: Vec<u8>, B_Name: Vec<u8>, B_Author: Vec<u8>, B_Copies: u64, Price: u64 ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;
			
			// Update storage.
			<CustomerID<T>>::put(C_Id);
			<CustomerName<T>>::put(C_Name.clone());
			<CustomerMobileNo<T>>::put(C_MobileNo.clone());
			<BookName<T>>::put(B_Name.clone());
			<BookAuthor<T>>::put(B_Author.clone());
			<BookCopies<T>>::put(B_Copies.clone());
			<BookPrice<T>>::put(Price.clone());
			// Emit an event.
			Self::deposit_event(Event::DetailStored(sender,C_Id, C_Name, C_MobileNo, B_Name, B_Author, B_Copies, Price));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		
		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}






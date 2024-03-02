#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn my_class)]
	pub type Class<T: Config> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn students_info)]
	pub type StudentsInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn dorm_info)]
	pub type DormInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32, //dorm number
		Blake2_128Concat,
		u32, //bed number
		u32, // student number
		ValueQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetClass(u32),
		SetStudentInfo(u32, u128),
		SetDormInfo(u32, u32, u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		#[pallet::call_index(0)]
		pub fn set_class_info(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			Class::<T>::put(class);
			Self::deposit_event(Event::SetClass(class));

			Ok(().into())
		}

		#[pallet::weight(0)]
		#[pallet::call_index(1)]
		pub fn set_student_info(
			origin: OriginFor<T>,
			student_number: u32,
			student_name: u128,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			StudentsInfo::<T>::insert(&student_number, &student_name);
			Self::deposit_event(Event::SetStudentInfo(student_number, student_name));

			Ok(().into())
		}

		#[pallet::weight(0)]
		#[pallet::call_index(2)]
		pub fn set_dorm_info(
			origin: OriginFor<T>,
			dorm_number: u32,
			bed_number: u32,
			student_number: u32,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			DormInfo::<T>::insert(&dorm_number, &bed_number, &student_number);
			Self::deposit_event(Event::SetDormInfo(dorm_number, bed_number, student_number));

			Ok(().into())
		}
	}
}

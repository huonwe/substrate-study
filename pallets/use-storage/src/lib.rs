#![cfg_attr(not(feature = "std"), no_std)]
// Add required imports and dependencies
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{DispatchResultWithPostInfo, ValueQuery, *}, transactional};
	use frame_system::{
		ensure_root,
		pallet_prelude::{OriginFor, *},
	};

	// Declare the pallet type
	// This is a placeholder to implement traits and methods.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Add the runtime configuration trait
	// All types and constants go here.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn my_class)]
	pub type Class<T: Config> = StorageValue<_, u32>;

	// #[pallet::storage]
	// #[pallet::getter(fn students_info)]
	// pub type StudentsInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn dorm_info)]
	// pub type DormInfo<T: Config> =
	// 	StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, u32, ValueQuery>;

	// Add runtime events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetClass(u32),
		// SetStudentInfo(u32, u128),
		// SetDormInfo(u32, u32, u32),
	}
	
	#[pallet::error]
	pub enum Error<T> {
		SetClassDuplicate,
		NumberTooSmall,
	}

	// Add hooks to define some logic that should be executed
	// in a specific context, for example on_initialize.
	//  #[pallet::hooks]
	//  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {  }

	// Add functions that are callable from outside the runtime.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[transactional]
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn set_class_info(origin: OriginFor<T>, class: u32, number: u32) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			
			
			if Class::<T>::exists() {
				return Err(Error::<T>::SetClassDuplicate.into())
			}
			
			if number <= 100u32 {
				return Err(Error::<T>::NumberTooSmall.into())
			}
			
			
			Class::<T>::put(class);
			Self::deposit_event(Event::SetClass(class));
			Ok(().into())
		}

		// #[pallet::call_index(1)]
		// #[pallet::weight(0)]
		// pub fn set_student_info(
		// 	origin: OriginFor<T>,
		// 	student_number: u32,
		// 	student_name: u128,
		// ) -> DispatchResultWithPostInfo {
		// 	ensure_signed(origin)?;

		// 	StudentsInfo::<T>::insert(&student_number, &student_name);
		// 	Self::deposit_event(Event::SetStudentInfo(student_number, student_name));
		// 	Ok(().into())
		// }

		// #[pallet::call_index(2)]
		// #[pallet::weight(0)]
		// pub fn set_dorm_info(
		// 	origin: OriginFor<T>,
		// 	dorm_number: u32,
		// 	bed_number: u32,
		// 	student_number: u32,
		// ) -> DispatchResultWithPostInfo {
		// 	ensure_signed(origin)?;
		// 	DormInfo::<T>::insert(&dorm_number, &bed_number, &student_number);
		// 	Self::deposit_event(Event::SetDormInfo(dorm_number, bed_number, student_number));
		// 	Ok(().into())
		// }
	}
}

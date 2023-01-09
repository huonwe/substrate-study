#![cfg_attr(not(feature = "std"), no_std)]
// Add required imports and dependencies
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
 use frame_support::pallet_prelude::*;
 use frame_system::pallet_prelude::*;
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

 // Add runtime storage to declare storage items.
 #[pallet::storage]
 pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128>;
 #[pallet::storage]
 pub type MyID<T: Config> = StorageValue<_, u128, ValueQuery>;
 // Add runtime events

 // Add runtime events
 #[pallet::event]
 #[pallet::generate_deposit(pub(super) fn deposit_event)]
 pub enum Event<T: Config> { 
  ClaimCreated(u32, u128),
 }

 // Add functions that are callable from outside the runtime.
 #[pallet::call]
 impl<T:Config> Pallet<T> {
  #[pallet::call_index(0)]
  #[pallet::weight(0)]
  pub fn create_claim(
   origin: OriginFor<T>,
   id: u32,
   claim: u128) -> DispatchResultWithPostInfo {
    ensure_signed(origin)?;
    Proofs::<T>::insert(
     &id,
     &claim,
    );
    Self::deposit_event(Event::ClaimCreated(id, claim));
    Ok(().into())
   }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              
 }
}
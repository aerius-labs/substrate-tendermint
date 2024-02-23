#![cfg_attr(not(feature = "std"), no_std)]

pub use tendermint_primitives as fp_primitives;

pub use fp_primitives::{AuthorityId, AuthorityList};
use frame_support::{
    pallet_prelude::Get, sp_runtime::BoundToRuntimeAppPublic, storage, traits::StorageVersion,
};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use tendermint_primitives::{SetId, TMNT_AUTHORITIES_KEY};
pub mod weights;

pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(4);
    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The maximum number of authorities that the pallet can hold.
        type MaxAuthorities: Get<u32>;
    }

    /// The number of changes (both in terms of keys and underlying economic responsibilities)
    /// in the "set" of Tendermint validators from genesis.
    #[pallet::storage]
    #[pallet::getter(fn current_set_id)]
    pub(super) type CurrentSetId<T: Config> = StorageValue<_, SetId, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub authorities: AuthorityList,
        #[serde(skip)]
        pub _config: sp_std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            CurrentSetId::<T>::put(tendermint_primitives::SetId::default());
            Pallet::<T>::initialize(&self.authorities);
        }
    }
}

impl<T: Config> Pallet<T> {
    /// Get the current set of authorities.
    pub fn authorities() -> AuthorityList {
        storage::unhashed::get_or_default::<AuthorityList>(TMNT_AUTHORITIES_KEY)
    }

    /// Set the current set of authorities.
    pub fn set_authorities(new_authorities: &AuthorityList) {
        storage::unhashed::put(TMNT_AUTHORITIES_KEY, new_authorities);
    }
    /// Initial authorities
    pub fn initialize(authorities: &AuthorityList) {
        if !authorities.is_empty() {
            assert!(
                Self::authorities().is_empty(),
                "Authorities are already initialized!"
            );
            Self::set_authorities(authorities);
        }
    }
}

impl<T: Config> BoundToRuntimeAppPublic for Pallet<T> {
    type Public = AuthorityId;
}

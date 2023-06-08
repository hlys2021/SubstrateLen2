use crate::Config;
use crate::Kitties;
use crate::Kitty;
use crate::KittyId;
use frame_system::Pallet;

use frame_support::{
    pallet_prelude::*, 
    storage::StoragePrefixedMap, 
    traits::GetStorageVersion, 
    weights::Weight,
};

use frame_system::pallet_prelude::*;
use frame_support::{migration::storage_key_iter, Blake2_128Concat};

#[derive(Encode, Decode, Clone, Debug, TypeInfo, MaxEncodedLen, PartialEq, Eq)]
	pub struct OldKitty(pub [u8; 16]);

pub fn migrate<T: Config>() -> Weight {
    let on_chain_version = Pallet::<T>::on_chain_storage_version();
    let current_version = Pallet::<T>::current_storage_version();

    if on_chain_version != 0 {
        return Weight::zero();
    }

    if current_version != 1 {
        return Weight::zero();
    }

    let module = Kitties::<T>::module_prefix();
    let item = Kitties::<T>::storage_prefix();

    for (index, kitty) in storage_key_iter::<KittyId, OldKitty, Blake2_128Concat>(module, item).drain(){
        
        let kitty = Kitty {
            dna: kitty.0,
            name: *b"abcd",
        };

        Kitties::<T>::insert(index, kitty);
    }
    Weight::zero()


}
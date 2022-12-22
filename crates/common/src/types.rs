//! Types definition for RMRK contract

use ink_prelude::vec::Vec;
use ink_primitives::{
    Key,
    KeyPtr,
};
use ink_storage::{
    traits::{
        ExtKeyPtr,
        PackedAllocate,
        PackedLayout,
        SpreadAllocate,
        SpreadLayout,
    },
    Mapping,
};
use openbrush::{
    contracts::psp34::Id,
    traits::{
        AccountId,
        String,
    },
};

// Collection id is the address of child contract
pub type CollectionId = AccountId;

// Nft is a tuple of collection and TokenId and refers to the Child nft
pub type ChildNft = (CollectionId, Id);

pub type BaseId = u32;
pub type SlotId = u32;
pub type PartId = u32;
pub type AssetId = u32;
pub type EquippableGroupId = u32;

/// Part's details
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default, Debug)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub struct Asset {
    pub asset_id: AssetId,
    pub equippable_group_id: EquippableGroupId,
    pub asset_uri: String,
}

impl ink_storage::traits::PackedAllocate for Asset {
    fn allocate_packed(&mut self, at: &Key) {
        PackedAllocate::allocate_packed(&mut *self, at)
    }
}

impl SpreadAllocate for Asset {
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        ptr.next_for::<Asset>();
        Asset::default()
    }
}

/// Part's details
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug, Clone)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub struct Part {
    /// Part type `None`, `Slot` or `Fixed`.
    pub part_type: PartType,

    /// Depth used for composing parts
    pub z: u8,

    /// Collections that can be equipped into this part
    pub equippable: Vec<AccountId>,

    /// Uri for this part
    pub metadata_uri: String,

    /// Is accepting to be equipped by any collection
    pub is_equippable_by_all: bool,
}

/// Used to define a type of the part. Possible values are `None`, `Slot` or `Fixed`.
#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
)]
pub enum PartType {
    None,
    Slot,
    Fixed,
}

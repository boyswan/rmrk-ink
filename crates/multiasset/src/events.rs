use rmrk_common::types::*;

use crate::{
    traits::Events,
    MultiAssetData,
};

use ink_prelude::vec::Vec;
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::Storage,
};

/// Event trait for MultiAssets
impl<T> Events for T
where
    T: Storage<MultiAssetData>,
{
    /// Used to notify listeners that an asset object is initialized at `assetId`.
    default fn _emit_asset_set_event(&self, _asset_id: &AssetId) {}

    /// Used to notify listeners that an asset object at `assetId` is added to token's pending asset array.
    default fn _emit_asset_added_to_token_event(
        &self,
        _token_id: &Id,
        _asset_id: &AssetId,
        _replaces_id: Option<Id>,
    ) {
    }

    /// Used to notify listeners that an asset object at `assetId` is accepted by the token and migrated
    default fn _emit_asset_accepted_event(&self, _token_id: &Id, _asset_id: &AssetId) {}

    /// Used to notify listeners that an asset object at `assetId` is rejected from token and is dropped from the pending assets array of the token.
    default fn _emit_asset_rejected_event(&self, _token_id: &Id, _asset_id: &AssetId) {}

    /// Used to notify listeners that an asset object at `assetId` is removed from token
    default fn _emit_asset_removed_event(&self, _token_id: &Id, _asset_id: &AssetId) {}

    /// Used to notify listeners that token's prioritiy array is reordered.
    default fn _emit_asset_priority_set_event(&self, _token_id: &Id, _priorities: Vec<AssetId>) {}
}

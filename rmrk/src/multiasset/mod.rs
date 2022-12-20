//! This module enables multiasset capability of RMRK

pub use crate::traits::multiasset::{
    Internal,
    MultiAsset,
    MultiAssetEvents,
};
use crate::{
    errors::RmrkError,
    types::*,
};
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::enumerable::*,
    },
    modifiers,
    storage::Mapping,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

mod events;
mod internal;

pub const STORAGE_MUSLTIASSET_KEY: u32 = openbrush::storage_unique_key!(MultiAssetData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_MUSLTIASSET_KEY)]
pub struct Data {
    /// List of available asset entries for this collection
    pub collection_asset_entries: Vec<Asset>,

    /// Mapping of tokenId to an array of active assets
    pub accepted_assets: Mapping<Id, Vec<AssetId>>,

    /// Mapping of tokenId to an array of pending assets
    pub pending_assets: Mapping<Id, Vec<AssetId>>,
}

impl<T> MultiAsset for T
where
    T: Storage<Data> + Storage<psp34::Data<enumerable::Balances>> + Storage<ownable::Data>,
{
    /// Used to add a asset entry.
    #[modifiers(only_owner)]
    fn add_asset_entry(
        &mut self,
        asset_id: AssetId,
        equippable_group_id: EquippableGroupId,
        base_id: BaseId,
        asset_uri: String,
        part_ids: Vec<PartId>,
    ) -> Result<(), PSP34Error> {
        if self.asset_id_exists(asset_id).is_some() {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::AssetIdAlreadyExists.as_str(),
            )))
        };
        self.data::<Data>().collection_asset_entries.push(Asset {
            asset_id,
            equippable_group_id,
            base_id,
            asset_uri,
            part_ids,
        });
        self._emit_asset_set_event(&asset_id);

        Ok(())
    }

    /// Used to add an asset to a token.
    fn add_asset_to_token(
        &mut self,
        token_id: Id,
        asset_id: AssetId,
        _replaces_asset_with_id: Option<Id>, // TODO implement replacement
    ) -> Result<(), PSP34Error> {
        self.asset_id_exists(asset_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::AssetIdNotFound.as_str(),
            )))?;
        let token_owner = self.ensure_exists(&token_id)?;
        self.in_accepted(&token_id, &asset_id)?;
        self.in_pending(&token_id, &asset_id)?;

        self._emit_asset_added_to_token_event(&token_id, &asset_id, None);
        let caller = Self::env().caller();
        if caller == token_owner {
            self.add_to_accepted_assets(&token_id, &asset_id);
        } else {
            self.add_to_pending_assets(&token_id, &asset_id);
        }

        Ok(())
    }

    /// Accepts an asset from the pending array of given token.
    fn accept_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error> {
        self.is_pending(&token_id, &asset_id)?;
        let token_owner = self.ensure_exists(&token_id)?;
        let caller = Self::env().caller();
        if caller == token_owner {
            self.remove_from_pending_assets(&token_id, &asset_id)?;
            self.add_to_accepted_assets(&token_id, &asset_id);
        } else {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::NotAuthorised.as_str(),
            )))
        }
        Ok(())
    }

    /// Rejects an asset from the pending array of given token.
    fn reject_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error> {
        self.is_pending(&token_id, &asset_id)?;
        let token_owner = self.ensure_exists(&token_id)?;
        self.ensure_token_owner(token_owner)?;

        self.remove_from_pending_assets(&token_id, &asset_id)?;

        self._emit_asset_rejected_event(&token_id, &asset_id);
        Ok(())
    }

    /// Rejects an asset from the pending array of given token.
    fn remove_asset(&mut self, token_id: Id, asset_id: AssetId) -> Result<(), PSP34Error> {
        self.is_accepted(&token_id, &asset_id)?;
        let token_owner = self.ensure_exists(&token_id)?;
        self.ensure_token_owner(token_owner)?;

        self.remove_from_accepted_assets(&token_id, &asset_id)?;

        self._emit_asset_removed_event(&token_id, &asset_id);
        Ok(())
    }

    /// Used to specify the priorities for a given token's active assets.
    fn set_priority(&mut self, token_id: Id, priorities: Vec<AssetId>) -> Result<(), PSP34Error> {
        let token_owner = self.ensure_exists(&token_id)?;
        self.ensure_token_owner(token_owner)?;
        if let Some(accepted_assets) = self.data::<Data>().accepted_assets.get(&token_id.clone()) {
            if accepted_assets.len() != priorities.len() {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::BadPriorityLength.as_str(),
                )))
            }
            for asset in priorities.clone() {
                if !accepted_assets.contains(&asset) {
                    return Err(PSP34Error::Custom(String::from(
                        RmrkError::AssetIdNotFound.as_str(),
                    )))
                }
            }
        }

        self.data::<Data>()
            .accepted_assets
            .insert(&token_id, &priorities);
        self._emit_asset_priority_set_event(&token_id, priorities);
        Ok(())
    }

    /// Used to retrieve the total number of asset entries
    fn total_assets(&self) -> u32 {
        self.data::<Data>().collection_asset_entries.len() as u32
    }

    /// Used to retrieve the total number of assets per token
    fn total_token_assets(&self, token_id: Id) -> Result<(u64, u64), PSP34Error> {
        self.ensure_exists(&token_id)?;

        let accepted_assets_on_token = match self.data::<Data>().accepted_assets.get(&token_id) {
            Some(assets) => assets.len() as u64,
            None => 0,
        };

        let pending_assets_on_token = match self.data::<Data>().pending_assets.get(&token_id) {
            Some(assets) => assets.len() as u64,
            None => 0,
        };

        Ok((accepted_assets_on_token, pending_assets_on_token))
    }

    /// Used to retrieve asset's uri
    fn get_asset_uri(&self, asset_id: AssetId) -> Option<String> {
        self.asset_id_exists(asset_id)
    }

    /// Fetch all accepted assets for the token_id
    fn get_accepted_token_assets(&self, token_id: Id) -> Result<Option<Vec<AssetId>>, PSP34Error> {
        self.ensure_exists(&token_id)?;
        Ok(self.data::<Data>().accepted_assets.get(&token_id))
    }
}

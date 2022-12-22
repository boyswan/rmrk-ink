use rmrk_common::{
    error::RmrkError,
    types::*,
};

pub use crate::traits::{
    Events,
    Internal,
    MultiAsset,
};

pub use crate::MultiAssetData;

use ink_prelude::vec::Vec;
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

/// Implement internal helper trait for MultiAsset
impl<T> Internal for T
where
    T: Storage<MultiAssetData> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if token is minted. Return the token uri
    default fn asset_id_exists(&self, asset_id: AssetId) -> Option<String> {
        if let Some(index) = self
            .data::<MultiAssetData>()
            .collection_asset_entries
            .iter()
            .position(|a| a.asset_id == asset_id)
        {
            let asset_uri =
                &self.data::<MultiAssetData>().collection_asset_entries[index].asset_uri;
            return Some(asset_uri.clone())
        }

        None
    }

    /// Check if token is minted. Return the owner
    default fn ensure_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
        let token_owner = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(id.clone())
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(token_owner)
    }

    /// Ensure that the caller is the token owner
    default fn ensure_token_owner(&self, token_owner: AccountId) -> Result<(), PSP34Error> {
        let caller = Self::env().caller();
        if caller != token_owner {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::NotAuthorised.as_str(),
            )))
        }
        Ok(())
    }

    /// Check if asset is already accepted
    default fn ensure_not_accepted(
        &self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self.data::<MultiAssetData>().accepted_assets.get(token_id) {
            if children.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AlreadyAddedAsset.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if asset is already pending
    default fn ensure_not_pending(
        &self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        if let Some(assets) = self.data::<MultiAssetData>().pending_assets.get(token_id) {
            if assets.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AddingPendingAsset.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if asset is already pending
    default fn ensure_pending(&self, token_id: &Id, asset_id: &AssetId) -> Result<(), PSP34Error> {
        if let Some(assets) = self.data::<MultiAssetData>().pending_assets.get(token_id) {
            if !assets.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AssetIdNotFound.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if asset is already accepted
    default fn ensure_accepted(&self, token_id: &Id, asset_id: &AssetId) -> Result<(), PSP34Error> {
        if let Some(assets) = self.data::<MultiAssetData>().accepted_assets.get(token_id) {
            if !assets.contains(asset_id) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AssetIdNotFound.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Add the asset to the list of accepted assets
    default fn add_to_accepted_assets(&mut self, token_id: &Id, asset_id: &AssetId) {
        let mut assets = self
            .data::<MultiAssetData>()
            .accepted_assets
            .get(&token_id)
            .unwrap_or(Vec::new());
        if !assets.contains(&asset_id) {
            assets.push(*asset_id);
            self.data::<MultiAssetData>()
                .accepted_assets
                .insert(&token_id, &assets);
        }
        self._emit_asset_accepted_event(token_id, asset_id);
    }

    /// Add the asset to the list of pending assets
    default fn add_to_pending_assets(&mut self, token_id: &Id, asset_id: &AssetId) {
        let mut assets = self
            .data::<MultiAssetData>()
            .pending_assets
            .get(&token_id)
            .unwrap_or(Vec::new());
        if !assets.contains(&asset_id) {
            assets.push(*asset_id);
            self.data::<MultiAssetData>()
                .pending_assets
                .insert(&token_id, &assets);
        }
    }

    /// remove the asset from the list of pending assets
    default fn remove_from_pending_assets(
        &mut self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        let mut assets = self
            .data::<MultiAssetData>()
            .pending_assets
            .get(&token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidAssetId.as_str(),
            )))?;

        let index = assets
            .iter()
            .position(|a| a == asset_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidTokenId.as_str(),
            )))?;
        assets.remove(index);

        self.data::<MultiAssetData>()
            .pending_assets
            .insert(&token_id, &assets);

        Ok(())
    }

    /// Remove the asset from the list of accepted assets
    default fn remove_from_accepted_assets(
        &mut self,
        token_id: &Id,
        asset_id: &AssetId,
    ) -> Result<(), PSP34Error> {
        let mut assets = self
            .data::<MultiAssetData>()
            .accepted_assets
            .get(&token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidAssetId.as_str(),
            )))?;

        let index = assets
            .iter()
            .position(|a| a == asset_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidTokenId.as_str(),
            )))?;
        assets.remove(index);

        self.data::<MultiAssetData>()
            .accepted_assets
            .insert(&token_id, &assets);

        Ok(())
    }
}

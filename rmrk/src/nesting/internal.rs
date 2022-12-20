//! This module enables nesting of RMRK or any other NFT which inherits PSP34.

use crate::{
    errors::RmrkError,
    nesting::Data,
    traits::nesting::{
        Internal,
        NestingEvents,
    },
    types::*,
};

use ink_env::CallFlags;
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

/// Implement internal helper trait for Nesting
impl<T> Internal for T
where
    T: Storage<Data> + Storage<psp34::Data<enumerable::Balances>> + NestingEvents,
{
    /// Check if child is already accepted
    default fn accepted(
        &self,
        parent_token_id: &Id,
        child_nft: &ChildNft,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self.data::<Data>().accepted_children.get(parent_token_id) {
            if children.contains(child_nft) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AlreadyAddedChild.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Check if child is already pending
    default fn pending(
        &self,
        parent_token_id: &Id,
        child_nft: &ChildNft,
    ) -> Result<(), PSP34Error> {
        if let Some(children) = self.data::<Data>().pending_children.get(parent_token_id) {
            if children.contains(child_nft) {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::AddingPendingChild.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Add the child to the list of accepted children
    default fn add_to_accepted(&mut self, parent_token_id: Id, child_nft: ChildNft) {
        let mut child_nfts = self
            .data::<Data>()
            .accepted_children
            .get(&parent_token_id)
            .unwrap_or(Vec::new());
        if !child_nfts.contains(&child_nft) {
            child_nfts.push(child_nft.clone());
            self.data::<Data>()
                .accepted_children
                .insert(&parent_token_id, &child_nfts);
            self._emit_child_accepted_event(&parent_token_id, &child_nft.0, &child_nft.1);
        }
    }

    /// Remove the child to the list of accepted children
    default fn remove_accepted(
        &mut self,
        parent_token_id: &Id,
        child_nft: &ChildNft,
    ) -> Result<(), PSP34Error> {
        let mut child_nfts = self
            .data::<Data>()
            .accepted_children
            .get(&parent_token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidParentId.as_str(),
            )))?;

        let index = child_nfts
            .iter()
            .position(|x| x == child_nft)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::ChildNotFound.as_str(),
            )))?;
        child_nfts.remove(index);

        self.data::<Data>()
            .accepted_children
            .insert(&parent_token_id, &child_nfts);

        self._emit_child_removed_event(&parent_token_id, &child_nft.0, &child_nft.1);
        Ok(())
    }

    /// Add the child to the list of pending children
    default fn add_to_pending(&mut self, parent_token_id: Id, child_nft: ChildNft) {
        let mut child_nfts = self
            .data::<Data>()
            .pending_children
            .get(&parent_token_id)
            .unwrap_or(Vec::new());
        if !child_nfts.contains(&child_nft) {
            child_nfts.push(child_nft);
            self.data::<Data>()
                .pending_children
                .insert(&parent_token_id, &child_nfts);
        }
    }

    /// Remove the child to the list of pending children
    default fn remove_from_pending(
        &mut self,
        parent_token_id: &Id,
        child_nft: &ChildNft,
    ) -> Result<(), PSP34Error> {
        let mut child_nfts = self
            .data::<Data>()
            .pending_children
            .get(&parent_token_id)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::InvalidParentId.as_str(),
            )))?;

        let index = child_nfts
            .iter()
            .position(|x| x == child_nft)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::ChildNotFound.as_str(),
            )))?;
        child_nfts.remove(index);

        self.data::<Data>()
            .pending_children
            .insert(&parent_token_id, &child_nfts);

        Ok(())
    }

    /// Check if token is minted. Return the owner
    default fn ensure_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
        let token_owner = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(id.clone())
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(token_owner)
    }

    /// Check if caller is the owner of this parent token
    default fn is_caller_parent_owner(
        &self,
        caller: AccountId,
        parent_token_id: &Id,
    ) -> Result<(), PSP34Error> {
        if let Some(token_owner) = self
            .data::<psp34::Data<enumerable::Balances>>()
            .owner_of(parent_token_id.clone())
        {
            if token_owner != caller {
                return Err(PSP34Error::Custom(String::from(
                    RmrkError::NotAuthorised.as_str(),
                )))
            }
        }
        Ok(())
    }

    /// Cross contract call to transfer child nft ownership
    default fn transfer_child_ownership(
        &self,
        to: AccountId,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        // TODO check child collection is approved by this (parent) collection
        // let collection = self.get_collection(child_nft.0)
        //      .ok_or(RmrkError::ChildContractNotApproved)?;

        PSP34Ref::transfer_builder(&child_nft.0, to, child_nft.1, Vec::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()?;
        ink_env::debug_println!("####### transfer  executed!!!!");

        Ok(())
    }
}

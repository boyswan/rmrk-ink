//! This module enables nesting of RMRK or any other NFT which inherits PSP34.

pub use crate::{
    nesting::Data,
    traits::nesting::{
        Internal,
        NestingEvents,
    },
};

use crate::{
    errors::RmrkError,
    types::*,
};
use ink_env::CallFlags;
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    storage::Mapping,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

/// Event trait for Nesting
impl<T> NestingEvents for T
where
    T: Storage<Data> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Emit ChildAdded event
    default fn _emit_added_child_event(
        &self,
        _to: &Id,
        _child_collection_address: &AccountId,
        _child_token_id: &Id,
    ) {
    }
    /// Emit ChildAccepted event
    default fn _emit_child_accepted_event(
        &self,
        _to: &Id,
        _child_collection_address: &AccountId,
        _child_token_id: &Id,
    ) {
    }

    /// Emit ChildRemoved event
    default fn _emit_child_removed_event(
        &self,
        _parent: &Id,
        _child_collection_address: &AccountId,
        _child_token_id: &Id,
    ) {
    }

    /// Emit ChildRejected event
    default fn _emit_child_rejected_event(
        &self,
        _parent: &Id,
        _child_collection_address: &AccountId,
        _child_token_id: &Id,
    ) {
    }
}

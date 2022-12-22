use crate::{
    traits::Events,
    NestingData,
};

use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        AccountId,
        Storage,
    },
};

/// Event trait for Nesting
impl<T> Events for T
where
    T: Storage<NestingData> + Storage<psp34::Data<enumerable::Balances>>,
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

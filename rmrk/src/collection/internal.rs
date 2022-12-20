//! Set of functions commonly used with PSP34 contract

use ink_prelude::string::{
    String as PreludeString,
    ToString,
};

use crate::errors::RmrkError;
pub use crate::traits::collection::{
    Collection,
    Internal,
};
use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::{
            enumerable::*,
            metadata::*,
        },
        reentrancy_guard::*,
    },
    modifiers,
    traits::{
        Balance,
        Storage,
        String,
    },
};

/// Helper trait for Psp34Custom
impl<T> Internal for T
where
    T: Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if token is minted
    default fn _token_exists(&self, id: Id) -> Result<(), PSP34Error> {
        self.data::<psp34::Data<enumerable::Balances>>()
            .owner_of(id)
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(())
    }
}

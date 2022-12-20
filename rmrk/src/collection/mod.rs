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

pub const STORAGE_PSP34_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_PSP34_KEY)]
pub struct Data {
    pub collection_id: u32,
}

impl<T> Collection for T
where
    T: Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    /// Set new value for the baseUri
    #[modifiers(only_owner)]
    default fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error> {
        let id = self
            .data::<psp34::Data<enumerable::Balances>>()
            .collection_id();
        self.data::<metadata::Data>()
            ._set_attribute(id, String::from("baseUri"), uri.into_bytes());
        Ok(())
    }

    /// Get URI for the token Id
    default fn token_uri(&self, token_id: u64) -> Result<PreludeString, PSP34Error> {
        self._token_exists(Id::U64(token_id))?;
        let value = self.get_attribute(
            self.data::<psp34::Data<enumerable::Balances>>()
                .collection_id(),
            String::from("baseUri"),
        );
        let mut token_uri = PreludeString::from_utf8(value.unwrap()).unwrap();
        token_uri = token_uri + &token_id.to_string() + &PreludeString::from(".json");
        Ok(token_uri)
    }
}

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

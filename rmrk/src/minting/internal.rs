//! RMRK minting implementation

use crate::{
    errors::RmrkError,
    minting::Data,
};

pub use crate::traits::minting::{
    Internal,
    Minting,
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
        AccountId,
        Balance,
        Storage,
        String,
    },
};

/// Helper trait for Psp34Custom
impl<T> Internal for T
where
    T: Storage<Data> + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if the transferred mint values is as expected
    default fn _check_value(
        &self,
        transfered_value: u128,
        mint_amount: u64,
    ) -> Result<(), PSP34Error> {
        if let Some(value) = (mint_amount as u128).checked_mul(self.data::<Data>().price_per_mint) {
            if transfered_value == value {
                return Ok(())
            }
        }
        return Err(PSP34Error::Custom(String::from(
            RmrkError::BadMintValue.as_str(),
        )))
    }

    /// Check amount of tokens to be minted
    default fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error> {
        if mint_amount == 0 {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::CannotMintZeroTokens.as_str(),
            )))
        }
        if let Some(amount) = self.data::<Data>().last_token_id.checked_add(mint_amount) {
            if amount <= self.data::<Data>().max_supply {
                return Ok(())
            }
        }
        return Err(PSP34Error::Custom(String::from(
            RmrkError::CollectionIsFull.as_str(),
        )))
    }

    default fn _token_exists(&self, id: Id) -> Result<(), PSP34Error> {
        self.data::<psp34::Data<enumerable::Balances>>()
            .owner_of(id)
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(())
    }
}

//! RMRK minting implementation
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod internal;
pub mod traits;

use rmrk_common::error::RmrkError;

use openbrush::{
    contracts::{
        ownable,
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

pub use traits::{
    Internal,
    Minting,
};

pub const STORAGE_MINTING_KEY: u32 = openbrush::storage_unique_key!(MintingData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_MINTING_KEY)]
pub struct MintingData {
    pub last_token_id: u64,
    pub max_supply: u64,
    pub price_per_mint: Balance,
}

impl<T> Minting for T
where
    T: Storage<MintingData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    /// Mint next available token for the caller
    default fn mint_next(&mut self) -> Result<(), PSP34Error> {
        self._check_value(Self::env().transferred_value(), 1)?;
        let caller = Self::env().caller();
        let token_id = self
            .data::<MintingData>()
            .last_token_id
            .checked_add(1)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::CollectionIsFull.as_str(),
            )))?;
        self.data::<psp34::Data<enumerable::Balances>>()
            ._mint_to(caller, Id::U64(token_id))?;
        self.data::<MintingData>().last_token_id += 1;

        self._emit_transfer_event(None, Some(caller), Id::U64(token_id));
        return Ok(())
    }

    /// Mint one or more tokens
    #[modifiers(non_reentrant)]
    default fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error> {
        self._check_value(Self::env().transferred_value(), mint_amount)?;
        self._check_amount(mint_amount)?;

        let next_to_mint = self.data::<MintingData>().last_token_id + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            self.data::<psp34::Data<enumerable::Balances>>()
                ._mint_to(to, Id::U64(mint_id))?;
            self.data::<MintingData>().last_token_id += 1;
            self._emit_transfer_event(None, Some(to), Id::U64(mint_id));
        }

        Ok(())
    }
}

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod collection;
pub mod errors;
pub mod minting;
pub mod multiasset;
pub mod nesting;
pub mod traits;
pub mod types;

use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::{
            enumerable::*,
            metadata::*,
        },
        reentrancy_guard::*,
    },
    traits::{
        AccountId,
        Balance,
        Storage,
        StorageAsMut,
        StorageAsRef,
        String,
    },
};

pub trait Rmrk<T> {
    fn configure(
        &mut self,
        name: String,
        symbol: String,
        base_uri: String,
        max_supply: u64,
        price_per_mint: Balance,
        collection_metadata: String,
    );

    fn configure_with_royalties(
        &mut self,
        name: String,
        symbol: String,
        base_uri: String,
        max_supply: u64,
        price_per_mint: Balance,
        collection_metadata: String,
        royalty_receiver: AccountId,
        royalty: u8,
    );
}

impl<T> Rmrk<T> for T
where
    T: openbrush::traits::DefaultEnv
        + Storage<minting::Data>
        + Storage<collection::Data>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    fn configure(
        &mut self,
        name: String,
        symbol: String,
        base_uri: String,
        max_supply: u64,
        price_per_mint: Balance,
        collection_metadata: String,
    ) {
        self._init_with_owner(<T as openbrush::traits::DefaultEnv>::env().caller());

        let psp34: &psp34::Data<enumerable::Balances> = <T as StorageAsRef>::data(self);
        let collection_id = psp34.collection_id();

        self._set_attribute(collection_id.clone(), String::from("name"), name);
        self._set_attribute(collection_id.clone(), String::from("symbol"), symbol);
        self._set_attribute(collection_id.clone(), String::from("baseUri"), base_uri);
        self._set_attribute(
            collection_id,
            String::from("collection_metadata"),
            collection_metadata,
        );

        let minting: &mut minting::Data = <T as StorageAsMut>::data(self);
        minting.max_supply = max_supply;
        minting.price_per_mint = price_per_mint;
    }

    fn configure_with_royalties(
        &mut self,
        name: String,
        symbol: String,
        base_uri: String,
        max_supply: u64,
        price_per_mint: Balance,
        collection_metadata: String,
        _royalty_receiver: AccountId,
        _royalty: u8,
    ) {
        Self::configure(
            self,
            name,
            symbol,
            base_uri,
            max_supply,
            price_per_mint,
            collection_metadata,
        );
    }
}

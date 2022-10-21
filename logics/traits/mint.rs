use crate::traits::errors::RmrkError;
use openbrush::contracts::psp34::*;
use openbrush::{modifiers, traits::AccountId};

#[openbrush::wrapper]
pub type RmrkMintableRef = dyn RmrkMintable;

#[openbrush::trait_definition]
pub trait RmrkMintable {
    #[ink(message)]
    #[modifiers(non_reentrant)]
    fn _mint_to(&mut self, _to: AccountId, _nft_id: Id) -> Result<(), RmrkError>;

    // fn nft_mint_directly_to_nft(&self, parent: AccountIdOrCollectionNftTuple) -> Result<(), RmrkError>;
}
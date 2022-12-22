mod mock;
use mock::mock_contract::Mock;

use ink_env::{
    pay_with_call,
    test,
    AccountId,
};

use ink_lang as ink;
use ink_prelude::string::String as PreludeString;
use rmrk::{
    error::*,
    types::*,
};

use crate::PSP34Error::TokenNotExists;
use openbrush::{
    contracts::psp34::extensions::{
        enumerable::*,
        metadata::*,
    },
    traits::{
        Balance,
        String,
    },
};
// use rmrk::traits::*;
use rmrk_common::utils::Utils;
// minting::Internal,

const PRICE: Balance = 100_000_000_000_000_000;
const BASE_URI: &str = "ipfs://myIpfsUri/";
const MAX_SUPPLY: u64 = 10;

#[ink::test]
fn init_works() {
    let rmrk = init();
    let collection_id = rmrk.collection_id();
    assert_eq!(
        rmrk.get_attribute(collection_id.clone(), String::from("name")),
        Some(String::from("Mock Project"))
    );
    assert_eq!(
        rmrk.get_attribute(collection_id.clone(), String::from("symbol")),
        Some(String::from("RMK"))
    );
    assert_eq!(
        rmrk.get_attribute(collection_id, String::from("baseUri")),
        Some(String::from(BASE_URI))
    );
    // assert_eq!(rmrk.max_supply(), MAX_SUPPLY);
    // assert_eq!(rmrk.price(), PRICE);
}

fn init() -> Mock {
    let accounts = default_accounts();
    Mock::new(
        String::from("Mock Project"),
        String::from("RMK"),
        String::from(BASE_URI),
        MAX_SUPPLY,
        PRICE,
        String::from(BASE_URI),
        accounts.eve,
        0,
    )
}

#[ink::test]
fn mint_single_works() {
    let mut rmrk = init();
    let accounts = default_accounts();
    assert_eq!(rmrk.owner(), accounts.alice);
    set_sender(accounts.bob);

    assert_eq!(rmrk.total_supply(), 0);
    test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
    assert!(rmrk.mint_next().is_ok());
    assert_eq!(rmrk.total_supply(), 1);
    assert_eq!(rmrk.owner_of(Id::U64(1)), Some(accounts.bob));
    assert_eq!(rmrk.balance_of(accounts.bob), 1);

    assert_eq!(rmrk.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
    assert_eq!(rmrk.minting.last_token_id, 1);
    assert_eq!(1, ink_env::test::recorded_events().count());
}

#[ink::test]
fn mint_multiple_works() {
    let mut rmrk = init();
    let accounts = default_accounts();
    set_sender(accounts.alice);
    let num_of_mints: u64 = 5;

    assert_eq!(rmrk.total_supply(), 0);
    test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE * num_of_mints as u128);
    assert!(rmrk.mint(accounts.bob, num_of_mints).is_ok());
    assert_eq!(rmrk.total_supply(), num_of_mints as u128);
    assert_eq!(rmrk.balance_of(accounts.bob), 5);
    assert_eq!(rmrk.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
    assert_eq!(rmrk.owners_token_by_index(accounts.bob, 1), Ok(Id::U64(2)));
    assert_eq!(rmrk.owners_token_by_index(accounts.bob, 2), Ok(Id::U64(3)));
    assert_eq!(rmrk.owners_token_by_index(accounts.bob, 3), Ok(Id::U64(4)));
    assert_eq!(rmrk.owners_token_by_index(accounts.bob, 4), Ok(Id::U64(5)));
    assert_eq!(5, ink_env::test::recorded_events().count());
    assert_eq!(
        rmrk.owners_token_by_index(accounts.bob, 5),
        Err(TokenNotExists)
    );
}

#[ink::test]
fn mint_above_limit_fails() {
    let mut rmrk = init();
    let accounts = default_accounts();
    set_sender(accounts.alice);
    let num_of_mints: u64 = MAX_SUPPLY + 1;

    assert_eq!(rmrk.total_supply(), 0);
    test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE * num_of_mints as u128);
    assert_eq!(
        rmrk.mint(accounts.bob, num_of_mints),
        Err(PSP34Error::Custom(RmrkError::CollectionIsFull.as_str()))
    );
}

#[ink::test]
fn mint_low_value_fails() {
    let mut rmrk = init();
    let accounts = default_accounts();
    set_sender(accounts.bob);
    let num_of_mints = 1;

    assert_eq!(rmrk.total_supply(), 0);
    test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE * num_of_mints as u128 - 1);
    assert_eq!(
        rmrk.mint(accounts.bob, num_of_mints),
        Err(PSP34Error::Custom(RmrkError::BadMintValue.as_str()))
    );
    test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE * num_of_mints as u128 - 1);
    assert_eq!(
        rmrk.mint_next(),
        Err(PSP34Error::Custom(RmrkError::BadMintValue.as_str()))
    );
    assert_eq!(rmrk.total_supply(), 0);
}

// #[ink::test]
// fn withdrawal_works() {
//     let mut rmrk = init();
//     let accounts = default_accounts();
//     set_balance(accounts.bob, PRICE);
//     set_sender(accounts.bob);

//     assert!(pay_with_call!(rmrk.mint_next(), PRICE).is_ok());
//     let expected_contract_balance = PRICE + rmrk.env().minimum_balance();
//     assert_eq!(rmrk.env().balance(), expected_contract_balance);

//     // Bob fails to withdraw
//     set_sender(accounts.bob);
//     assert!(rmrk.withdraw().is_err());
//     assert_eq!(rmrk.env().balance(), expected_contract_balance);

//     // Alice (contract owner) withdraws. Existential minimum is still set
//     set_sender(accounts.alice);
//     assert!(rmrk.withdraw().is_ok());
//     // assert_eq!(rmrk.env().balance(), rmrk.env().minimum_balance());
// }

#[ink::test]
fn token_uri_works() {
    let mut rmrk = init();
    let accounts = default_accounts();
    set_sender(accounts.alice);

    test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE);
    assert!(rmrk.mint_next().is_ok());
    // return error if request is for not yet minted token
    assert_eq!(rmrk.token_uri(42), Err(TokenNotExists));
    assert_eq!(
        rmrk.token_uri(1),
        Ok(PreludeString::from(BASE_URI.to_owned() + "1.json"))
    );

    // return error if request is for not yet minted token
    assert_eq!(rmrk.token_uri(42), Err(TokenNotExists));

    // verify token_uri when baseUri is empty
    set_sender(accounts.alice);
    assert!(rmrk.set_base_uri(PreludeString::from("")).is_ok());
    assert_eq!(
        rmrk.token_uri(1),
        Ok("".to_owned() + &PreludeString::from("1.json"))
    );
}

#[ink::test]
fn owner_is_set() {
    let accounts = default_accounts();
    let rmrk = init();
    assert_eq!(rmrk.owner(), accounts.alice);
}

#[ink::test]
fn set_base_uri_works() {
    let accounts = default_accounts();
    const NEW_BASE_URI: &str = "new_uri/";
    let mut rmrk = init();

    set_sender(accounts.alice);
    let collection_id = rmrk.collection_id();
    assert!(rmrk.set_base_uri(NEW_BASE_URI.into()).is_ok());
    assert_eq!(
        rmrk.get_attribute(collection_id, String::from("baseUri")),
        Some(String::from(NEW_BASE_URI))
    );
    set_sender(accounts.bob);
    assert_eq!(
        rmrk.set_base_uri(NEW_BASE_URI.into()),
        Err(PSP34Error::Custom(String::from("O::CallerIsNotOwner")))
    );
}

#[ink::test]
fn check_supply_overflow_ok() {
    let accounts = default_accounts();
    let max_supply = u64::MAX - 1;
    let mut rmrk = Mock::new(
        String::from("Remark Project"),
        String::from("RMK"),
        String::from(BASE_URI),
        max_supply,
        PRICE,
        String::from(BASE_URI),
        accounts.eve,
        0,
    );
    rmrk.minting.last_token_id = max_supply - 1;

    // check case when last_token_id.add(mint_amount) if more than u64::MAX
    assert_eq!(
        rmrk._check_amount(3),
        Err(PSP34Error::Custom(RmrkError::CollectionIsFull.as_str()))
    );

    // check case when mint_amount is 0
    assert_eq!(
        rmrk._check_amount(0),
        Err(PSP34Error::Custom(RmrkError::CannotMintZeroTokens.as_str()))
    );
}

#[ink::test]
fn check_value_overflow_ok() {
    let accounts = default_accounts();
    let max_supply = u64::MAX;
    let price = u128::MAX as u128;
    let rmrk = Mock::new(
        String::from("Remark Project"),
        String::from("RMK"),
        String::from(BASE_URI),
        max_supply,
        price,
        String::from(BASE_URI),
        accounts.eve,
        0,
    );
    let transferred_value = u128::MAX;
    let mint_amount = u64::MAX;
    assert_eq!(
        rmrk._check_value(transferred_value, mint_amount),
        Err(PSP34Error::Custom(RmrkError::BadMintValue.as_str()))
    );
}

#[ink::test]
fn add_asset_entry_works() {
    const ASSET_URI: &str = "asset_uri/";
    const ASSET_ID: AssetId = 1;
    let mut rmrk = init();
    assert!(rmrk
        .add_asset_entry(ASSET_ID, 1, String::from(ASSET_URI))
        .is_ok());
    assert_eq!(rmrk.total_assets(), 1);
    assert_eq!(rmrk.get_asset_uri(ASSET_ID), Some(String::from(ASSET_URI)));
    assert_eq!(rmrk.get_asset_uri(42), None);

    // reject adding asset with same asset_id
    assert_eq!(
        rmrk.add_asset_entry(ASSET_ID, 1, String::from(ASSET_URI)),
        Err(PSP34Error::Custom(RmrkError::AssetIdAlreadyExists.as_str()))
    );
}

#[ink::test]
fn add_asset_to_token_works() {
    let accounts = default_accounts();
    const ASSET_URI: &str = "asset_uri/";
    const ASSET_ID: AssetId = 1;
    const TOKEN_ID1: Id = Id::U64(1);
    const TOKEN_ID2: Id = Id::U64(2);

    let mut rmrk = init();
    // Add new asset entry
    assert!(rmrk
        .add_asset_entry(ASSET_ID, 1, String::from(ASSET_URI))
        .is_ok());
    assert_eq!(rmrk.total_assets(), 1);
    assert_eq!(1, ink_env::test::recorded_events().count());

    // mint token and add asset to it. Should be accepted without approval
    test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE as u128);
    assert!(rmrk.mint(accounts.alice, 1).is_ok());
    assert_eq!(2, ink_env::test::recorded_events().count());
    assert!(rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID, None).is_ok());
    assert_eq!(4, ink_env::test::recorded_events().count());
    assert_eq!(rmrk.total_token_assets(TOKEN_ID1), Ok((1, 0)));

    // error cases
    assert_eq!(
        rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID, None),
        Err(PSP34Error::Custom(RmrkError::AlreadyAddedAsset.as_str()))
    );
    assert_eq!(
        rmrk.add_asset_to_token(TOKEN_ID1, 42, None),
        Err(PSP34Error::Custom(RmrkError::AssetIdNotFound.as_str()))
    );

    // mint second token to non owner (Bob)
    set_sender(accounts.alice);
    test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE as u128);
    assert!(rmrk.mint(accounts.bob, 1).is_ok());
    assert_eq!(5, ink_env::test::recorded_events().count());
    set_sender(accounts.bob);
    assert_eq!(
        rmrk.add_asset_to_token(TOKEN_ID2, ASSET_ID, None),
        Err(PSP34Error::Custom(String::from("O::CallerIsNotOwner")))
    );

    // Add asset by alice and reject asset by Bob to test asset_reject
    set_sender(accounts.alice);
    assert!(rmrk.add_asset_to_token(TOKEN_ID2, ASSET_ID, None).is_ok());
    assert_eq!(6, ink_env::test::recorded_events().count());
    assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((0, 1)));
    set_sender(accounts.bob);
    assert!(rmrk.reject_asset(TOKEN_ID2, ASSET_ID).is_ok());
    assert_eq!(7, ink_env::test::recorded_events().count());
    assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((0, 0)));

    // Add asset by alice and accept asset by Bob, to test accept_asset
    set_sender(accounts.alice);
    assert!(rmrk.add_asset_to_token(TOKEN_ID2, ASSET_ID, None).is_ok());
    assert_eq!(8, ink_env::test::recorded_events().count());
    assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((0, 1)));
    set_sender(accounts.bob);
    assert!(rmrk.accept_asset(TOKEN_ID2, ASSET_ID).is_ok());
    assert_eq!(9, ink_env::test::recorded_events().count());
    assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((1, 0)));
    assert_eq!(rmrk.get_accepted_token_assets(TOKEN_ID2), Ok(Some(vec![1])));

    // Try adding asset to not minted token fails
    set_sender(accounts.alice);
    assert_eq!(
        rmrk.add_asset_to_token(Id::U64(3), ASSET_ID, None),
        Err(TokenNotExists)
    );

    // Try removing not added asset fails
    assert_eq!(
        rmrk.remove_asset(TOKEN_ID2, 42),
        Err(PSP34Error::Custom(RmrkError::AssetIdNotFound.as_str()))
    );

    // Try removing asset for not minted token fails
    assert_eq!(rmrk.remove_asset(Id::U64(3), ASSET_ID), Err(TokenNotExists));

    // Try removing asset by collection owner fails
    set_sender(accounts.alice);
    assert_eq!(
        rmrk.remove_asset(TOKEN_ID2, ASSET_ID),
        Err(PSP34Error::Custom(RmrkError::NotAuthorised.as_str()))
    );

    // Remove accepted asset
    set_sender(accounts.bob);
    assert!(rmrk.remove_asset(TOKEN_ID2, ASSET_ID).is_ok());
    assert_eq!(10, ink_env::test::recorded_events().count());
    assert_eq!(rmrk.get_accepted_token_assets(TOKEN_ID2), Ok(Some(vec![])));
    assert_eq!(rmrk.total_token_assets(TOKEN_ID2), Ok((0, 0)));
}

#[ink::test]
fn set_asset_priority_works() {
    let accounts = default_accounts();
    const ASSET_URI: &str = "asset_uri/";
    const ASSET_ID1: AssetId = 1;
    const ASSET_ID2: AssetId = 100;
    const TOKEN_ID1: Id = Id::U64(1);

    let mut rmrk = init();
    // Add new asset entry
    assert!(rmrk
        .add_asset_entry(ASSET_ID1, 1, String::from(ASSET_URI))
        .is_ok());
    assert!(rmrk
        .add_asset_entry(ASSET_ID2, 1, String::from(ASSET_URI))
        .is_ok());
    assert_eq!(rmrk.total_assets(), 2);

    // mint token and add two assets to it. Should be accepted without approval
    test::set_value_transferred::<ink_env::DefaultEnvironment>(PRICE * 2 as u128);
    assert!(rmrk.mint(accounts.alice, 2).is_ok());
    assert!(rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID1, None).is_ok());
    assert!(rmrk.add_asset_to_token(TOKEN_ID1, ASSET_ID2, None).is_ok());
    assert_eq!(rmrk.total_token_assets(TOKEN_ID1), Ok((2, 0)));
    assert_eq!(
        rmrk.get_accepted_token_assets(TOKEN_ID1),
        Ok(Some(vec![ASSET_ID1, ASSET_ID2]))
    );
    assert!(rmrk
        .set_priority(TOKEN_ID1, vec![ASSET_ID2, ASSET_ID1])
        .is_ok());
    assert_eq!(
        rmrk.get_accepted_token_assets(TOKEN_ID1),
        Ok(Some(vec![ASSET_ID2, ASSET_ID1]))
    );

    // error cases
    assert_eq!(
        rmrk.set_priority(TOKEN_ID1, vec![ASSET_ID2]),
        Err(PSP34Error::Custom(RmrkError::BadPriorityLength.as_str()))
    );
    assert_eq!(
        rmrk.set_priority(TOKEN_ID1, vec![ASSET_ID2, 42]),
        Err(PSP34Error::Custom(RmrkError::AssetIdNotFound.as_str()))
    );
}

#[ink::test]
fn add_parts_to_base_works() {
    const ASSET_URI: &str = "asset_uri/";
    const ASSET_ID: AssetId = 1;
    const TOKEN_ID1: Id = Id::U64(1);
    const TOKEN_ID2: Id = Id::U64(2);
    const EQUIPABLE_ADDRESS1: [u8; 32] = [1; 32];
    const EQUIPABLE_ADDRESS2: [u8; 32] = [2; 32];
    const EQUIPABLE_ADDRESS3: [u8; 32] = [3; 32];
    const PART_ID0: PartId = 0;
    const PART_ID1: PartId = 1;
    let part_list = vec![
        // Background option 1
        Part {
            part_type: PartType::Slot,
            z: 0,
            equippable: vec![EQUIPABLE_ADDRESS1.into(), EQUIPABLE_ADDRESS2.into()],
            metadata_uri: String::from("ipfs://backgrounds/1.svg"),
            is_equippable_by_all: false,
        },
        // Background option 2
        Part {
            part_type: PartType::Fixed,
            z: 0,
            equippable: vec![],
            metadata_uri: String::from("ipfs://backgrounds/2.svg"),
            is_equippable_by_all: false,
        },
    ];

    let bad_part_list1 = vec![Part {
        part_type: PartType::Fixed,
        z: 0,
        equippable: vec![EQUIPABLE_ADDRESS1.into()],
        metadata_uri: String::from("ipfs://backgrounds/2.svg"),
        is_equippable_by_all: false,
    }];
    let bad_part_list2 = vec![Part {
        part_type: PartType::Fixed,
        z: 0,
        equippable: vec![],
        metadata_uri: String::from("ipfs://backgrounds/2.svg"),
        is_equippable_by_all: true,
    }];

    let mut rmrk = init();

    // verify add/get parts
    assert!(rmrk.get_parts_count() == 0);
    assert!(rmrk.add_part_list(part_list.clone()).is_ok());
    assert_eq!(rmrk.get_parts_count(), part_list.len() as u32);
    assert_eq!(rmrk.get_part(0).unwrap().z, part_list[0].z);
    assert_eq!(
        rmrk.get_part(0).unwrap().metadata_uri,
        part_list[0].metadata_uri
    );

    // verify array of equippable addresses
    assert!(rmrk.is_equippable(PART_ID0, EQUIPABLE_ADDRESS1.into()));
    assert!(rmrk.is_equippable(PART_ID0, EQUIPABLE_ADDRESS2.into()));
    assert!(!rmrk.is_equippable(PART_ID1, EQUIPABLE_ADDRESS2.into()));

    assert!(!rmrk.is_equippable_by_all(PART_ID0));
    assert!(rmrk.set_equippable_by_all(PART_ID0).is_ok());
    assert!(rmrk.is_equippable_by_all(PART_ID0));
    assert!(!rmrk.is_equippable_by_all(42));

    assert!(rmrk.reset_equippable_addresses(PART_ID0).is_ok());
    assert!(!rmrk.is_equippable_by_all(PART_ID0));
    assert!(!rmrk.is_equippable(PART_ID0, EQUIPABLE_ADDRESS1.into()));
    assert!(rmrk
        .add_equippable_addresses(
            PART_ID0,
            vec![EQUIPABLE_ADDRESS1.into(), EQUIPABLE_ADDRESS2.into()]
        )
        .is_ok());
    assert!(rmrk.is_equippable(PART_ID0, EQUIPABLE_ADDRESS1.into()));
    assert_eq!(
        rmrk.add_equippable_addresses(PART_ID1, vec![EQUIPABLE_ADDRESS1.into()]),
        Err(PSP34Error::Custom(RmrkError::PartIsNotSlot.as_str()))
    );
    assert_eq!(
        rmrk.reset_equippable_addresses(PART_ID1),
        Err(PSP34Error::Custom(RmrkError::PartIsNotSlot.as_str()))
    );
    assert_eq!(
        rmrk.set_equippable_by_all(PART_ID1),
        Err(PSP34Error::Custom(RmrkError::PartIsNotSlot.as_str()))
    );
    assert_eq!(
        rmrk.add_part_list(bad_part_list1.clone()),
        Err(PSP34Error::Custom(RmrkError::BadConfig.as_str()))
    );
    assert_eq!(
        rmrk.add_part_list(bad_part_list2.clone()),
        Err(PSP34Error::Custom(RmrkError::BadConfig.as_str()))
    );

    assert!(!rmrk.is_equippable(PART_ID0, EQUIPABLE_ADDRESS3.into()));

    // verify set/get base metadata
    assert_eq!(rmrk.get_base_metadata(), "");
    assert!(rmrk
        .setup_base(String::from("ipfs://base_metadata"))
        .is_ok());
    assert_eq!(rmrk.get_base_metadata(), "ipfs://base_metadata");

    // assert_eq!(1, ink_env::test::recorded_events().count());
}

fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
    ink_env::test::default_accounts::<_>()
}

fn set_sender(sender: AccountId) {
    ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
}

fn set_balance(account_id: AccountId, balance: Balance) {
    ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account_id, balance)
}

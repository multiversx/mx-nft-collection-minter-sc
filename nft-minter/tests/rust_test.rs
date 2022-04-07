pub mod constants;
pub mod nft_minter_interactor;

use constants::*;
use elrond_wasm_debug::{managed_buffer, rust_biguint};
use nft_minter::nft_module::NftModule;
use nft_minter_interactor::*;

#[test]
fn init_test() {
    let _ = NftMinterSetup::new(nft_minter::contract_obj);
}

#[test]
fn create_brands_test() {
    let mut nm_setup = NftMinterSetup::new(nft_minter::contract_obj);
    nm_setup.create_default_brands();

    // try create brand, same collection
    nm_setup
        .call_create_new_brand(
            FIRST_COLLECTION_ID,
            THIRD_BRAND_ID,
            b"png",
            0,
            5,
            1,
            b"EGLD",
            1,
            b"",
            b"TICKER",
            &[],
        )
        .assert_user_error("Collection already exists");

    // try create brand, same brand ID
    nm_setup
        .call_create_new_brand(
            THIRD_COLLECTION_ID,
            FIRST_BRAND_ID,
            b"png",
            0,
            5,
            1,
            b"EGLD",
            1,
            b"",
            b"TICKER",
            &[],
        )
        .assert_user_error("Brand already exists");

    // try create brand, unsupported media type
    nm_setup
        .call_create_new_brand(
            THIRD_COLLECTION_ID,
            THIRD_BRAND_ID,
            b"exe",
            0,
            5,
            1,
            b"EGLD",
            1,
            b"",
            b"TICKER",
            &[],
        )
        .assert_user_error("Invalid media type");
}

#[test]
fn buy_random_nft_test() {
    let mut nm_setup = NftMinterSetup::new(nft_minter::contract_obj);
    nm_setup.create_default_brands();

    // try buy before start
    let first_user_addr = nm_setup.first_user_address.clone();
    nm_setup
        .call_buy_random_nft(
            &first_user_addr,
            FIRST_MINT_PRICE_TOKEN_ID,
            FIRST_MINT_PRICE_AMOUNT,
            FIRST_BRAND_ID,
            1,
        )
        .assert_user_error("May not mint yet");

    nm_setup
        .b_mock
        .set_block_timestamp(FIRST_MINT_START_TIMESTAMP);

    // buy random nft ok
    let first_user_addr = nm_setup.first_user_address.clone();
    nm_setup
        .call_buy_random_nft(
            &first_user_addr,
            FIRST_MINT_PRICE_TOKEN_ID,
            FIRST_MINT_PRICE_AMOUNT,
            FIRST_BRAND_ID,
            1,
        )
        .assert_ok();

    // user receives token with nonce 1, and ID 2
    let expected_attributes = nm_setup.build_nft_attributes_first_token(2);
    nm_setup.b_mock.check_nft_balance(
        &first_user_addr,
        FIRST_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Some(&expected_attributes),
    );

    // check unique ID mapper internal consistency
    // ID 2 was removed, so pos 2 should have the last item, i.e. ID 5
    nm_setup
        .b_mock
        .execute_query(&nm_setup.nm_wrapper, |sc| {
            let mapper = sc.available_ids(&managed_buffer!(FIRST_BRAND_ID));
            assert_eq!(mapper.len(), 4);
            assert_eq!(mapper.get(1), 1);
            assert_eq!(mapper.get(2), 5);
            assert_eq!(mapper.get(3), 3);
            assert_eq!(mapper.get(4), 4);
        })
        .assert_ok();

    // buy multiple NFTs - wrong payment amount
    let second_user_address = nm_setup.second_user_address.clone();
    nm_setup
        .call_buy_random_nft(
            &second_user_address,
            FIRST_MINT_PRICE_TOKEN_ID,
            FIRST_MINT_PRICE_AMOUNT,
            FIRST_BRAND_ID,
            2,
        )
        .assert_user_error("Invalid payment");

    // try buy too many
    nm_setup
        .call_buy_random_nft(
            &second_user_address,
            FIRST_MINT_PRICE_TOKEN_ID,
            FIRST_MINT_PRICE_AMOUNT * 5,
            FIRST_BRAND_ID,
            5,
        )
        .assert_user_error("Not enough NFTs available");

    // buy 2 ok
    nm_setup
        .call_buy_random_nft(
            &second_user_address,
            FIRST_MINT_PRICE_TOKEN_ID,
            FIRST_MINT_PRICE_AMOUNT * 2,
            FIRST_BRAND_ID,
            2,
        )
        .assert_ok();

    // second user gets ID 3 and 1
    let expected_attributes_first = nm_setup.build_nft_attributes_first_token(3);
    let expected_attributes_second = nm_setup.build_nft_attributes_first_token(1);
    nm_setup.b_mock.check_nft_balance(
        &second_user_address,
        FIRST_TOKEN_ID,
        2,
        &rust_biguint!(1),
        Some(&expected_attributes_first),
    );
    nm_setup.b_mock.check_nft_balance(
        &second_user_address,
        FIRST_TOKEN_ID,
        3,
        &rust_biguint!(1),
        Some(&expected_attributes_second),
    );

    // check unique ID mapper internal consistency
    // ID 3 was removed, and then ID 1, so mapper would look like this
    // initially: 1 5 3 4
    // after first rand (3 removed): 1 5 4
    // after second rand (1 removed): 4 5
    nm_setup
        .b_mock
        .execute_query(&nm_setup.nm_wrapper, |sc| {
            let mapper = sc.available_ids(&managed_buffer!(FIRST_BRAND_ID));
            assert_eq!(mapper.len(), 2);
            assert_eq!(mapper.get(1), 4);
            assert_eq!(mapper.get(2), 5);
        })
        .assert_ok();
}

#[test]
fn giveaway_test() {
    let mut nm_setup = NftMinterSetup::new(nft_minter::contract_obj);
    nm_setup.create_default_brands();

    // giveaway single nft
    let first_user_addr = nm_setup.first_user_address.clone();
    nm_setup
        .call_giveaway(SECOND_BRAND_ID, [(first_user_addr.clone(), 1)].to_vec())
        .assert_ok();

    // user received nonce 1 and ID 7
    let mut attr = nm_setup.build_nft_attributes_second_token(7);
    nm_setup.b_mock.check_nft_balance(
        &first_user_addr,
        SECOND_TOKEN_ID,
        1,
        &rust_biguint!(1),
        Some(&attr),
    );

    nm_setup
        .b_mock
        .execute_query(&nm_setup.nm_wrapper, |sc| {
            let mapper = sc.available_ids(&managed_buffer!(SECOND_BRAND_ID));
            assert_eq!(mapper.len(), 9);
            assert_eq!(mapper.get(1), 1);
            assert_eq!(mapper.get(2), 2);
            assert_eq!(mapper.get(3), 3);
            assert_eq!(mapper.get(4), 4);
            assert_eq!(mapper.get(5), 5);
            assert_eq!(mapper.get(6), 6);
            assert_eq!(mapper.get(7), 10); // this changed
            assert_eq!(mapper.get(8), 8);
            assert_eq!(mapper.get(9), 9);
        })
        .assert_ok();

    // giveaway two, single user
    nm_setup
        .call_giveaway(SECOND_BRAND_ID, [(first_user_addr.clone(), 2)].to_vec())
        .assert_ok();

    // received ID 4 and 5
    attr = nm_setup.build_nft_attributes_second_token(4);
    nm_setup.b_mock.check_nft_balance(
        &first_user_addr,
        SECOND_TOKEN_ID,
        2,
        &rust_biguint!(1),
        Some(&attr),
    );

    attr = nm_setup.build_nft_attributes_second_token(5);
    nm_setup.b_mock.check_nft_balance(
        &first_user_addr,
        SECOND_TOKEN_ID,
        3,
        &rust_biguint!(1),
        Some(&attr),
    );

    nm_setup
        .b_mock
        .execute_query(&nm_setup.nm_wrapper, |sc| {
            let mapper = sc.available_ids(&managed_buffer!(SECOND_BRAND_ID));
            assert_eq!(mapper.len(), 7);
            assert_eq!(mapper.get(1), 1);
            assert_eq!(mapper.get(2), 2);
            assert_eq!(mapper.get(3), 3);
            assert_eq!(mapper.get(4), 9); // pos 4
            assert_eq!(mapper.get(5), 8); // and 5 changed
            assert_eq!(mapper.get(6), 6);
            assert_eq!(mapper.get(7), 10);
        })
        .assert_ok();

    // giveaway, multiple users
    let second_user_addr = nm_setup.second_user_address.clone();
    nm_setup
        .call_giveaway(
            SECOND_BRAND_ID,
            [(first_user_addr.clone(), 2), (second_user_addr.clone(), 1)].to_vec(),
        )
        .assert_ok();

    // user 1 received IDs 10 and 1
    attr = nm_setup.build_nft_attributes_second_token(10);
    nm_setup.b_mock.check_nft_balance(
        &first_user_addr,
        SECOND_TOKEN_ID,
        4,
        &rust_biguint!(1),
        Some(&attr),
    );

    attr = nm_setup.build_nft_attributes_second_token(1);
    nm_setup.b_mock.check_nft_balance(
        &first_user_addr,
        SECOND_TOKEN_ID,
        5,
        &rust_biguint!(1),
        Some(&attr),
    );

    // user 2 received ID
    attr = nm_setup.build_nft_attributes_second_token(6);
    nm_setup.b_mock.check_nft_balance(
        &second_user_addr,
        SECOND_TOKEN_ID,
        6,
        &rust_biguint!(1),
        Some(&attr),
    );

    // mapper progress:
    // 1 2 3 9 8 6 10
    // 1 2 3 9 8 6
    // 6 2 3 9 8
    // 8 2 3 9
    nm_setup
        .b_mock
        .execute_query(&nm_setup.nm_wrapper, |sc| {
            let mapper = sc.available_ids(&managed_buffer!(SECOND_BRAND_ID));
            assert_eq!(mapper.len(), 4);
            assert_eq!(mapper.get(1), 8);
            assert_eq!(mapper.get(2), 2);
            assert_eq!(mapper.get(3), 3);
            assert_eq!(mapper.get(4), 9);
        })
        .assert_ok();
}

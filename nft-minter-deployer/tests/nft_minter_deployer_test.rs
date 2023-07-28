use multiversx_sc::{codec::multi_types::OptionalValue, types::Address};
use multiversx_sc_modules::pause::PauseModule;
use multiversx_sc_scenario::{
    managed_address, managed_buffer, rust_biguint, testing_framework::BlockchainStateWrapper,
};
use nft_minter::{admin_whitelist::AdminWhitelistModule, NftMinter};
use nft_minter_deployer::{factory::FactoryModule, NftMinterDeployer};

pub const NFT_MINTER_WASM_PATH: &str = "nft-minter/output/nft-minter.wasm";
pub const NFT_MINTER_DEPLOYER_WASM_PATH: &str =
    "nft-minter-deployer/output/nft-minter-deployer.wasm";
pub const CATEGORY: &[u8] = b"VeryCoolNfts";
pub const MAX_NFTS_PER_TX: usize = 2;

#[test]
fn test_nft_minter_deployer() {
    let rust_zero = rust_biguint!(0u64);
    let mut b_mock = BlockchainStateWrapper::new();
    let owner = b_mock.create_user_account(&rust_zero);
    let user = b_mock.create_user_account(&rust_zero);

    let nft_minter_deployer_wrapper = b_mock.create_sc_account(
        &rust_zero,
        Some(&owner),
        nft_minter_deployer::contract_obj,
        NFT_MINTER_DEPLOYER_WASM_PATH,
    );

    let nft_minter_template_wrapper = b_mock.create_sc_account(
        &rust_zero,
        Some(nft_minter_deployer_wrapper.address_ref()),
        nft_minter::contract_obj,
        NFT_MINTER_WASM_PATH,
    );

    // setup nft_minter
    b_mock
        .execute_tx(&owner, &nft_minter_template_wrapper, &rust_zero, |sc| {
            sc.init(
                managed_buffer!(CATEGORY),
                managed_address!(&user),
                managed_address!(&user),
                MAX_NFTS_PER_TX,
                OptionalValue::None,
            );
        })
        .assert_ok();

    let nft_minter_wrapper = b_mock.prepare_deploy_from_sc(
        nft_minter_deployer_wrapper.address_ref(),
        nft_minter::contract_obj,
    );

    b_mock
        .execute_tx(&owner, &nft_minter_deployer_wrapper, &rust_zero, |sc| {
            sc.init(OptionalValue::Some(managed_address!(
                nft_minter_template_wrapper.address_ref()
            )));
            sc.set_nft_minter_creation_enabled(true);
        })
        .assert_ok();

    // Deploy user NFT Minter contract
    let mut user_nft_minter_address = Address::zero();
    b_mock
        .execute_tx(&user, &nft_minter_deployer_wrapper, &rust_zero, |sc| {
            let new_nft_minter_address = sc.create_nft_minter_endpoint(
                managed_buffer!(CATEGORY),
                managed_address!(&user),
                managed_address!(&user),
                MAX_NFTS_PER_TX,
            );
            assert!(sc
                .user_nft_minter_contracts(&managed_address!(&user))
                .contains(&new_nft_minter_address));

            user_nft_minter_address = new_nft_minter_address.to_address();
        })
        .assert_ok();

    // Test pause
    b_mock
        .execute_tx(&user, &nft_minter_deployer_wrapper, &rust_zero, |sc| {
            sc.pause_nft_minter(managed_address!(&user_nft_minter_address));
        })
        .assert_ok();
    b_mock
        .execute_tx(&user, &nft_minter_wrapper, &rust_zero, |sc| {
            assert!(sc.is_paused());
        })
        .assert_ok();

    // Test resume
    b_mock
        .execute_tx(&user, &nft_minter_deployer_wrapper, &rust_zero, |sc| {
            sc.resume_nft_minter(managed_address!(&user_nft_minter_address));
        })
        .assert_ok();
    b_mock
        .execute_tx(&user, &nft_minter_wrapper, &rust_zero, |sc| {
            assert!(!sc.is_paused());
        })
        .assert_ok();

    // Test add new admin
    let user2 = b_mock.create_user_account(&rust_zero);
    b_mock
        .execute_tx(&user, &nft_minter_wrapper, &rust_zero, |sc| {
            assert!(!sc.admin_whitelist().contains(&managed_address!(&user2)));
        })
        .assert_ok();
    b_mock
        .execute_tx(&user, &nft_minter_deployer_wrapper, &rust_zero, |sc| {
            sc.add_admin_to_nft_minter_contract(
                managed_address!(&user2),
                managed_address!(&user_nft_minter_address),
            );
        })
        .assert_ok();
    b_mock
        .execute_tx(&user, &nft_minter_wrapper, &rust_zero, |sc| {
            assert!(sc.admin_whitelist().contains(&managed_address!(&user2)));
        })
        .assert_ok();

    b_mock
        .execute_tx(&owner, &nft_minter_deployer_wrapper, &rust_zero, |sc| {
            sc.upgrade_nft_minter_endpoint(
                managed_address!(&user_nft_minter_address),
                managed_buffer!(CATEGORY),
                managed_address!(&user),
                managed_address!(&user),
                MAX_NFTS_PER_TX,
            );
        })
        .assert_ok();
}

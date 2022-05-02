use super::constants::*;
use elrond_wasm::{
    elrond_codec::multi_types::OptionalValue,
    types::{Address, EsdtLocalRole, MultiValueEncoded},
};
use elrond_wasm_debug::{
    managed_address, managed_biguint, managed_buffer, rust_biguint,
    testing_framework::{BlockchainStateWrapper, ContractObjWrapper},
    tx_mock::TxResult,
    DebugApi,
};
use nft_minter::NftMinter;
use nft_minter::{common_storage::COLLECTION_HASH_LEN, nft_module::NftModule};

// Temporary re-implementation until next elrond-wasm version is released with the fix
#[macro_export]
macro_rules! managed_token_id {
    ($bytes:expr) => {{
        if $bytes == elrond_wasm::types::TokenIdentifier::<DebugApi>::EGLD_REPRESENTATION {
            elrond_wasm::types::TokenIdentifier::egld()
        } else {
            elrond_wasm::types::TokenIdentifier::from_esdt_bytes($bytes)
        }
    }};
}

pub struct NftMinterSetup<NftMinterObjBuilder>
where
    NftMinterObjBuilder: 'static + Copy + Fn() -> nft_minter::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub nm_wrapper: ContractObjWrapper<nft_minter::ContractObj<DebugApi>, NftMinterObjBuilder>,
}

impl<NftMinterObjBuilder> NftMinterSetup<NftMinterObjBuilder>
where
    NftMinterObjBuilder: 'static + Copy + Fn() -> nft_minter::ContractObj<DebugApi>,
{
    pub fn new(builder: NftMinterObjBuilder) -> Self {
        let rust_zero = rust_biguint!(0u64);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner_address = b_mock.create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE));
        let first_user_address = b_mock.create_user_account(&rust_biguint!(USER_EGLD_BALANCE));
        let second_user_address = b_mock.create_user_account(&rust_biguint!(USER_EGLD_BALANCE));
        let nm_wrapper =
            b_mock.create_sc_account(&rust_zero, Some(&owner_address), builder, "nft minter path");

        // init ESDT System SC mock
        b_mock.create_sc_account_fixed_address(
            &Address::from(ESDT_SYSTEM_SC_ADDRESS_ARRAY),
            &rust_zero,
            None,
            esdt_system_sc_mock::contract_obj,
            "ESDT system SC mock path",
        );

        b_mock
            .execute_tx(&owner_address, &nm_wrapper, &rust_zero, |sc| {
                sc.init(
                    managed_buffer!(CATEGORY),
                    managed_address!(&owner_address),
                    managed_address!(&owner_address),
                );
            })
            .assert_ok();

        Self {
            b_mock,
            owner_address,
            first_user_address,
            second_user_address,
            nm_wrapper,
        }
    }

    pub fn create_default_brands(&mut self) {
        self.call_create_new_brand(
            FIRST_COLLECTION_HASH,
            FIRST_BRAND_ID,
            FIRST_MEDIA_TYPE,
            0,
            FIRST_MAX_NFTS,
            FIRST_MINT_START_TIMESTAMP,
            FIRST_MINT_END_TIMESTAMP,
            FIRST_MINT_PRICE_TOKEN_ID,
            FIRST_MINT_PRICE_AMOUNT,
            FIRST_TOKEN_DISPLAY_NAME,
            FIRST_TOKEN_TICKER,
            FIRST_TAGS,
        )
        .assert_ok();

        self.call_create_new_brand(
            SECOND_COLLECTION_HASH,
            SECOND_BRAND_ID,
            SECOND_MEDIA_TYPE,
            0,
            SECOND_MAX_NFTS,
            SECOND_MINT_START_TIMESTAMP,
            SECOND_MINT_END_TIMESTAMP,
            SECOND_MINT_PRICE_TOKEN_ID,
            SECOND_MINT_PRICE_AMOUNT,
            SECOND_TOKEN_DISPLAY_NAME,
            SECOND_TOKEN_TICKER,
            SECOND_TAGS,
        )
        .assert_ok();

        self.b_mock.set_esdt_local_roles(
            self.nm_wrapper.address_ref(),
            FIRST_TOKEN_ID,
            &[EsdtLocalRole::NftCreate][..],
        );
        self.b_mock.set_esdt_local_roles(
            self.nm_wrapper.address_ref(),
            SECOND_TOKEN_ID,
            &[EsdtLocalRole::NftCreate][..],
        );
    }

    pub fn build_nft_attributes_first_token(&self, nft_id: usize) -> String {
        let mut attr = "metadata:".to_owned();
        attr += &String::from_utf8(FIRST_COLLECTION_HASH.to_vec()).unwrap();
        attr += "/";
        attr += &nft_id.to_string();
        attr += ".json;";
        attr += "tags:funny,sad,memes";

        attr
    }

    pub fn build_nft_attributes_second_token(&self, nft_id: usize) -> String {
        let mut attr = "metadata:".to_owned();
        attr += &String::from_utf8(SECOND_COLLECTION_HASH.to_vec()).unwrap();
        attr += "/";
        attr += &nft_id.to_string();
        attr += ".json;";
        attr += "tags:random,good,best";

        attr
    }
}

impl<NftMinterObjBuilder> NftMinterSetup<NftMinterObjBuilder>
where
    NftMinterObjBuilder: 'static + Copy + Fn() -> nft_minter::ContractObj<DebugApi>,
{
    pub fn call_create_new_brand(
        &mut self,
        collection_hash: &[u8; COLLECTION_HASH_LEN],
        brand_id: &[u8],
        media_type: &[u8],
        royalties: u64,
        max_nfts: usize,
        mint_start_timestamp: u64,
        mint_end_timestamp: u64,
        mint_price_token_id: &[u8],
        mint_price_amount: u64,
        token_display_name: &[u8],
        token_ticker: &[u8],
        tags: &[&[u8]],
    ) -> TxResult {
        self.b_mock.execute_tx(
            &self.owner_address,
            &self.nm_wrapper,
            &rust_biguint!(ISSUE_COST),
            |sc| {
                let mut managed_tags = MultiValueEncoded::new();
                for tag in tags {
                    managed_tags.push(managed_buffer!(&tag));
                }

                sc.issue_token_for_brand(
                    collection_hash.into(),
                    managed_buffer!(brand_id),
                    managed_buffer!(media_type),
                    managed_biguint!(royalties),
                    max_nfts,
                    mint_start_timestamp,
                    mint_end_timestamp,
                    managed_token_id!(mint_price_token_id),
                    managed_biguint!(mint_price_amount),
                    managed_buffer!(token_display_name),
                    managed_buffer!(token_ticker),
                    managed_tags,
                );
            },
        )
    }

    pub fn call_buy_random_nft(
        &mut self,
        buyer_address: &Address,
        payment_token: &[u8],
        payment_amount: u64,
        brand_id: &[u8],
        nfts_to_buy: usize,
    ) -> TxResult {
        let opt_nft_amount = if nfts_to_buy == 1 {
            OptionalValue::None
        } else {
            OptionalValue::Some(nfts_to_buy)
        };

        if payment_token == EGLD_TOKEN_ID {
            self.b_mock.execute_tx(
                buyer_address,
                &self.nm_wrapper,
                &rust_biguint!(payment_amount),
                |sc| {
                    sc.buy_random_nft(managed_buffer!(brand_id), opt_nft_amount);
                },
            )
        } else {
            self.b_mock.execute_esdt_transfer(
                buyer_address,
                &self.nm_wrapper,
                payment_token,
                0,
                &rust_biguint!(payment_amount),
                |sc| {
                    sc.buy_random_nft(managed_buffer!(brand_id), opt_nft_amount);
                },
            )
        }
    }

    pub fn call_giveaway(
        &mut self,
        brand_id: &[u8],
        dest_amount_pairs: Vec<(Address, usize)>,
    ) -> TxResult {
        self.b_mock.execute_tx(
            &self.owner_address,
            &self.nm_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                for (dest, amt) in dest_amount_pairs {
                    args.push((managed_address!(&dest), amt).into());
                }

                sc.giveaway_nfts(managed_buffer!(brand_id), args);
            },
        )
    }
}

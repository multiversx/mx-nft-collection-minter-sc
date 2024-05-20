use super::constants::*;
use multiversx_sc::{
    codec::multi_types::OptionalValue,
    types::{Address, EsdtLocalRole, ManagedVec, MultiValueEncoded},
};
use multiversx_sc_scenario::{
    imports::TxResult,
    imports::{BlockchainStateWrapper, ContractObjWrapper},
    managed_address, managed_biguint, managed_buffer, rust_biguint, DebugApi,
};
use nft_minter::brand_creation::BrandCreationModule;
use nft_minter::nft_attributes_builder::COLLECTION_HASH_LEN;
use nft_minter::nft_minting::NftMintingModule;
use nft_minter::NftMinter;

#[macro_export]
macro_rules! managed_token_id {
    ($bytes:expr) => {{
        if $bytes
            == multiversx_sc::types::EgldOrEsdtTokenIdentifier::<DebugApi>::EGLD_REPRESENTATION
        {
            multiversx_sc::types::EgldOrEsdtTokenIdentifier::egld()
        } else {
            let ___token_id___ = multiversx_sc::types::TokenIdentifier::from_esdt_bytes($bytes);
            multiversx_sc::types::EgldOrEsdtTokenIdentifier::esdt(___token_id___)
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
                    managed_address!(&owner_address),
                    managed_address!(&owner_address),
                    MAX_NFTS_PER_TX,
                    OptionalValue::None,
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
            FIRST_MINT_START_TIMESTAMP,
            FIRST_MINT_END_TIMESTAMP,
            FIRST_MINT_PRICE_TOKEN_ID,
            FIRST_MINT_PRICE_AMOUNT,
            FIRST_TOKEN_DISPLAY_NAME,
            FIRST_TOKEN_TICKER,
            FIRST_TAGS,
            FIRST_TIERS,
            FIRST_NFT_AMOUNTS,
            0,
        )
        .assert_ok();

        self.call_create_new_brand(
            SECOND_COLLECTION_HASH,
            SECOND_BRAND_ID,
            SECOND_MEDIA_TYPE,
            0,
            SECOND_MINT_START_TIMESTAMP,
            SECOND_MINT_END_TIMESTAMP,
            SECOND_MINT_PRICE_TOKEN_ID,
            SECOND_MINT_PRICE_AMOUNT,
            SECOND_TOKEN_DISPLAY_NAME,
            SECOND_TOKEN_TICKER,
            SECOND_TAGS,
            SECOND_TIERS,
            SECOND_NFT_AMOUNTS,
            0,
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
        mint_start_timestamp: u64,
        mint_end_timestamp: u64,
        mint_price_token_id: &[u8],
        mint_price_amount: u64,
        token_display_name: &[u8],
        token_ticker: &[u8],
        tags: &[&[u8]],
        tiers: &[&[u8]],
        nr_nfts_per_tier: &[usize],
        whitelist_expire_epoch: u64,
    ) -> TxResult {
        self.b_mock.execute_tx(
            &self.owner_address,
            &self.nm_wrapper,
            &rust_biguint!(ISSUE_COST),
            |sc| {
                let mut managed_tags = ManagedVec::new();
                for tag in tags {
                    managed_tags.push(managed_buffer!(&tag));
                }

                if tiers.len() != nr_nfts_per_tier.len() {
                    panic!("Tier args length mismatch");
                }

                let mut tier_args = MultiValueEncoded::new();
                for (tier, nr_nfts) in tiers.iter().zip(nr_nfts_per_tier.iter()) {
                    tier_args.push(
                        (
                            managed_buffer!(tier),
                            *nr_nfts,
                            managed_biguint!(mint_price_amount),
                        )
                            .into(),
                    );
                }

                sc.issue_token_for_brand(
                    collection_hash.into(),
                    managed_buffer!(brand_id),
                    managed_buffer!(media_type),
                    managed_biguint!(royalties),
                    mint_start_timestamp,
                    mint_end_timestamp,
                    managed_token_id!(mint_price_token_id),
                    managed_buffer!(token_display_name),
                    managed_buffer!(token_ticker),
                    whitelist_expire_epoch,
                    managed_tags,
                    tier_args,
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
        tier: &[u8],
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
                    sc.buy_random_nft(
                        managed_buffer!(brand_id),
                        managed_buffer!(tier),
                        opt_nft_amount,
                    );
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
                    sc.buy_random_nft(
                        managed_buffer!(brand_id),
                        managed_buffer!(tier),
                        opt_nft_amount,
                    );
                },
            )
        }
    }

    pub fn call_giveaway(
        &mut self,
        brand_id: &[u8],
        tier: &[u8],
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

                sc.giveaway_nfts(managed_buffer!(brand_id), managed_buffer!(tier), args);
            },
        )
    }
}
